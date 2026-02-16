//! Qdrant 向量数据库实现 — 生产环境的记忆存储

use async_trait::async_trait;
use qdrant_client::qdrant::{
    CreateCollectionBuilder, DeletePointsBuilder, Distance, PointStruct,
    ScalarQuantizationBuilder, SearchPointsBuilder, UpsertPointsBuilder, VectorParamsBuilder,
    PointsIdsList,
};
use qdrant_client::Qdrant;
use tracing::{debug, info};

use ai_school_core::config::QdrantConfig;
use ai_school_core::error::MemoryError;
use ai_school_core::traits::MemoryStore;
use ai_school_core::types::{AgentId, Memory, MemoryId, MemoryLayer, MemoryQuery, ScoredMemory};

/// Qdrant 记忆存储
pub struct QdrantMemoryStore {
    client: Qdrant,
    collection_prefix: String,
    vector_size: u64,
}

impl QdrantMemoryStore {
    pub async fn new(
        config: &QdrantConfig,
        simulation_id: &str,
    ) -> Result<Self, MemoryError> {
        let client = Qdrant::from_url(&config.url)
            .build()
            .map_err(|e| MemoryError::StoreError(format!("Failed to connect to Qdrant: {e}")))?;

        let store = Self {
            client,
            collection_prefix: simulation_id.to_string(),
            vector_size: config.vector_size,
        };

        store.ensure_collections().await?;

        Ok(store)
    }

    fn collection_name(&self) -> String {
        format!("{}_memories", self.collection_prefix)
    }

    async fn ensure_collections(&self) -> Result<(), MemoryError> {
        let name = self.collection_name();

        let exists = self
            .client
            .collection_exists(&name)
            .await
            .map_err(|e| MemoryError::StoreError(format!("Failed to check collection: {e}")))?;

        if !exists {
            self.client
                .create_collection(
                    CreateCollectionBuilder::new(&name)
                        .vectors_config(VectorParamsBuilder::new(
                            self.vector_size,
                            Distance::Cosine,
                        ))
                        .quantization_config(ScalarQuantizationBuilder::default()),
                )
                .await
                .map_err(|e| {
                    MemoryError::StoreError(format!("Failed to create collection: {e}"))
                })?;

            info!(collection = %name, "Created Qdrant collection");
        }

        Ok(())
    }

    fn memory_to_payload(memory: &Memory) -> serde_json::Map<String, serde_json::Value> {
        let mut payload = serde_json::Map::new();
        payload.insert(
            "agent_id".to_string(),
            serde_json::Value::String(memory.agent_id.0.to_string()),
        );
        payload.insert(
            "layer".to_string(),
            serde_json::Value::String(format!("{:?}", memory.layer)),
        );
        payload.insert(
            "content".to_string(),
            serde_json::Value::String(memory.content.clone()),
        );
        payload.insert(
            "timestamp".to_string(),
            serde_json::json!(memory.timestamp.tick),
        );
        payload.insert(
            "importance".to_string(),
            serde_json::json!(memory.importance),
        );
        payload.insert(
            "emotion_valence".to_string(),
            serde_json::json!(memory.emotion_valence),
        );
        payload.insert(
            "tags".to_string(),
            serde_json::json!(memory.tags),
        );
        if let Some(event_id) = &memory.event_id {
            payload.insert(
                "event_id".to_string(),
                serde_json::Value::String(event_id.0.to_string()),
            );
        }
        payload
    }
}

#[async_trait]
impl MemoryStore for QdrantMemoryStore {
    async fn store(
        &self,
        _agent_id: &AgentId,
        memory: &Memory,
        embedding: &[f32],
    ) -> Result<MemoryId, MemoryError> {
        let point = PointStruct::new(
            memory.id.0.to_string(),
            embedding.to_vec(),
            Self::memory_to_payload(memory),
        );

        self.client
            .upsert_points(
                UpsertPointsBuilder::new(&self.collection_name(), vec![point]),
            )
            .await
            .map_err(|e| MemoryError::StoreError(format!("Failed to store memory: {e}")))?;

        debug!(memory_id = %memory.id.0, "Stored memory in Qdrant");

        Ok(memory.id.clone())
    }

    async fn retrieve(
        &self,
        agent_id: &AgentId,
        query: &MemoryQuery,
        query_embedding: &[f32],
    ) -> Result<Vec<ScoredMemory>, MemoryError> {
        let mut filter_conditions = vec![qdrant_client::qdrant::Condition::matches(
            "agent_id",
            agent_id.0.to_string(),
        )];

        if let Some(layer) = &query.layer_filter {
            filter_conditions.push(qdrant_client::qdrant::Condition::matches(
                "layer",
                format!("{:?}", layer),
            ));
        }

        let filter = qdrant_client::qdrant::Filter::must(filter_conditions);

        let search_result = self
            .client
            .search_points(
                SearchPointsBuilder::new(
                    &self.collection_name(),
                    query_embedding.to_vec(),
                    query.limit as u64,
                )
                .filter(filter)
                .with_payload(true),
            )
            .await
            .map_err(|e| MemoryError::RetrievalError(format!("Search failed: {e}")))?;

        let results = search_result
            .result
            .into_iter()
            .filter_map(|point| {
                let payload = &point.payload;
                let content = payload.get("content")?.as_str()?.to_string();
                let importance = payload
                    .get("importance")
                    .and_then(|v| v.as_double())
                    .unwrap_or(0.5) as f32;

                let point_id_str = match &point.id {
                    Some(pid) => format!("{:?}", pid),
                    None => return None,
                };

                let memory = Memory {
                    id: MemoryId(uuid::Uuid::parse_str(&point_id_str).unwrap_or_else(|_| uuid::Uuid::now_v7())),
                    agent_id: agent_id.clone(),
                    layer: MemoryLayer::LongTerm,
                    content,
                    timestamp: ai_school_core::types::SimulationTime::new(),
                    importance,
                    emotion_valence: payload
                        .get("emotion_valence")
                        .and_then(|v| v.as_double())
                        .unwrap_or(0.0) as f32,
                    event_id: None,
                    tags: Vec::new(),
                    access_count: 0,
                    last_accessed: ai_school_core::types::SimulationTime::new(),
                };

                Some(ScoredMemory {
                    relevance: point.score,
                    recency: 1.0,
                    importance_score: importance,
                    score: point.score,
                    memory,
                })
            })
            .collect();

        Ok(results)
    }

    async fn get_recent(
        &self,
        _agent_id: &AgentId,
        _layer: MemoryLayer,
        _limit: usize,
    ) -> Result<Vec<Memory>, MemoryError> {
        // TODO: Implement scroll with timestamp ordering
        Ok(Vec::new())
    }

    async fn consolidate(
        &self,
        agent_id: &AgentId,
        source_ids: &[MemoryId],
        consolidated: &Memory,
        embedding: &[f32],
    ) -> Result<MemoryId, MemoryError> {
        let id = self.store(agent_id, consolidated, embedding).await?;
        self.forget(agent_id, source_ids).await?;
        Ok(id)
    }

    async fn forget(
        &self,
        _agent_id: &AgentId,
        memory_ids: &[MemoryId],
    ) -> Result<(), MemoryError> {
        let point_ids: Vec<qdrant_client::qdrant::PointId> = memory_ids
            .iter()
            .map(|id| id.0.to_string().into())
            .collect();

        self.client
            .delete_points(
                DeletePointsBuilder::new(&self.collection_name())
                    .points(PointsIdsList { ids: point_ids }),
            )
            .await
            .map_err(|e| MemoryError::StoreError(format!("Failed to delete memories: {e}")))?;

        Ok(())
    }
}

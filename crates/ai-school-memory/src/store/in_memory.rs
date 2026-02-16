//! 内存实现 — 测试用 + 感知记忆/短期缓冲

use std::collections::HashMap;
use std::sync::RwLock;

use async_trait::async_trait;

use ai_school_core::error::MemoryError;
use ai_school_core::traits::MemoryStore;
use ai_school_core::types::{AgentId, Memory, MemoryId, MemoryLayer, MemoryQuery, ScoredMemory};

/// 基于内存的记忆存储（用于测试和开发）
pub struct InMemoryStore {
    memories: RwLock<HashMap<MemoryId, (Memory, Vec<f32>)>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self {
            memories: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for InMemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MemoryStore for InMemoryStore {
    async fn store(
        &self,
        _agent_id: &AgentId,
        memory: &Memory,
        embedding: &[f32],
    ) -> Result<MemoryId, MemoryError> {
        let id = memory.id.clone();
        let mut store = self.memories.write().map_err(|e| {
            MemoryError::StoreError(format!("Lock poisoned: {e}"))
        })?;
        store.insert(id.clone(), (memory.clone(), embedding.to_vec()));
        Ok(id)
    }

    async fn retrieve(
        &self,
        agent_id: &AgentId,
        query: &MemoryQuery,
        query_embedding: &[f32],
    ) -> Result<Vec<ScoredMemory>, MemoryError> {
        let store = self.memories.read().map_err(|e| {
            MemoryError::RetrievalError(format!("Lock poisoned: {e}"))
        })?;

        let mut results: Vec<ScoredMemory> = store
            .values()
            .filter(|(m, _)| {
                m.agent_id == *agent_id
                    && query.layer_filter.as_ref().is_none_or(|l| m.layer == *l)
                    && (query.tag_filter.is_empty()
                        || query.tag_filter.iter().any(|t| m.tags.contains(t)))
            })
            .map(|(m, emb)| {
                let relevance = cosine_similarity(query_embedding, emb);
                let recency = 1.0; // Simplified for in-memory
                let importance_score = m.importance;
                let score = 0.5 * relevance + 0.3 * recency + 0.2 * importance_score;

                ScoredMemory {
                    memory: m.clone(),
                    score,
                    relevance,
                    recency,
                    importance_score,
                }
            })
            .collect();

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        results.truncate(query.limit);

        Ok(results)
    }

    async fn get_recent(
        &self,
        agent_id: &AgentId,
        layer: MemoryLayer,
        limit: usize,
    ) -> Result<Vec<Memory>, MemoryError> {
        let store = self.memories.read().map_err(|e| {
            MemoryError::RetrievalError(format!("Lock poisoned: {e}"))
        })?;

        let mut memories: Vec<Memory> = store
            .values()
            .filter(|(m, _)| m.agent_id == *agent_id && m.layer == layer)
            .map(|(m, _)| m.clone())
            .collect();

        memories.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        memories.truncate(limit);

        Ok(memories)
    }

    async fn consolidate(
        &self,
        _agent_id: &AgentId,
        source_ids: &[MemoryId],
        consolidated: &Memory,
        embedding: &[f32],
    ) -> Result<MemoryId, MemoryError> {
        let mut store = self.memories.write().map_err(|e| {
            MemoryError::StoreError(format!("Lock poisoned: {e}"))
        })?;

        // Remove source memories
        for id in source_ids {
            store.remove(id);
        }

        // Add consolidated memory
        let id = consolidated.id.clone();
        store.insert(id.clone(), (consolidated.clone(), embedding.to_vec()));

        Ok(id)
    }

    async fn forget(
        &self,
        _agent_id: &AgentId,
        memory_ids: &[MemoryId],
    ) -> Result<(), MemoryError> {
        let mut store = self.memories.write().map_err(|e| {
            MemoryError::StoreError(format!("Lock poisoned: {e}"))
        })?;

        for id in memory_ids {
            store.remove(id);
        }

        Ok(())
    }
}

/// 计算余弦相似度
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }

    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot / (norm_a * norm_b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ai_school_core::types::SimulationTime;

    #[tokio::test]
    async fn test_store_and_retrieve() {
        let store = InMemoryStore::new();
        let agent_id = AgentId::new();
        let time = SimulationTime::new();

        let memory = Memory {
            id: MemoryId::new(),
            agent_id: agent_id.clone(),
            layer: MemoryLayer::ShortTerm,
            content: "今天数学课学了微积分".to_string(),
            timestamp: time.clone(),
            importance: 0.5,
            emotion_valence: 0.3,
            event_id: None,
            tags: vec!["academic".to_string()],
            access_count: 0,
            last_accessed: time,
        };

        let embedding = vec![0.1, 0.2, 0.3, 0.4];
        let id = store.store(&agent_id, &memory, &embedding).await.unwrap();

        let query = MemoryQuery {
            query_text: "数学".to_string(),
            layer_filter: None,
            tag_filter: vec![],
            since: None,
            limit: 10,
        };

        let results = store.retrieve(&agent_id, &query, &embedding).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].memory.id, id);
    }
}

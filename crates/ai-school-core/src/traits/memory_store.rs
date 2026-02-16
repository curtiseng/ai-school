use async_trait::async_trait;

use crate::error::MemoryError;
use crate::types::{AgentId, Memory, MemoryId, MemoryLayer, MemoryQuery, ScoredMemory};

/// 记忆存储 trait — ADR-0005 选型四
///
/// 抽象 Qdrant 等向量数据库的记忆操作。
#[async_trait]
pub trait MemoryStore: Send + Sync {
    /// 存储一条记忆（含嵌入向量）
    async fn store(
        &self,
        agent_id: &AgentId,
        memory: &Memory,
        embedding: &[f32],
    ) -> Result<MemoryId, MemoryError>;

    /// 多维检索记忆
    /// Score = α×relevance + β×recency + γ×importance
    async fn retrieve(
        &self,
        agent_id: &AgentId,
        query: &MemoryQuery,
        query_embedding: &[f32],
    ) -> Result<Vec<ScoredMemory>, MemoryError>;

    /// 获取最近的记忆
    async fn get_recent(
        &self,
        agent_id: &AgentId,
        layer: MemoryLayer,
        limit: usize,
    ) -> Result<Vec<Memory>, MemoryError>;

    /// 记忆巩固：将多条记忆合并为更高层级的记忆
    async fn consolidate(
        &self,
        agent_id: &AgentId,
        source_ids: &[MemoryId],
        consolidated: &Memory,
        embedding: &[f32],
    ) -> Result<MemoryId, MemoryError>;

    /// 删除过期/遗忘的记忆
    async fn forget(
        &self,
        agent_id: &AgentId,
        memory_ids: &[MemoryId],
    ) -> Result<(), MemoryError>;
}

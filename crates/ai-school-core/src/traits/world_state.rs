use async_trait::async_trait;

use crate::error::WorldError;
use crate::types::{AgentId, AgentState, Location, LocationId, Relationship, WorldSnapshot};

/// 世界状态读取 trait
///
/// 提供对结构化世界状态的只读访问，供可视化层和 Agent 感知使用。
#[async_trait]
pub trait WorldStateReader: Send + Sync {
    /// 获取 Agent 当前状态
    async fn get_agent_state(&self, agent_id: &AgentId) -> Result<AgentState, WorldError>;

    /// 获取所有 Agent 状态
    async fn get_all_agents(&self) -> Result<Vec<AgentState>, WorldError>;

    /// 获取指定位置的 Agent 列表
    async fn get_agents_at_location(
        &self,
        location_id: &LocationId,
    ) -> Result<Vec<AgentState>, WorldError>;

    /// 获取两个 Agent 之间的关系
    async fn get_relationship(
        &self,
        agent_a: &AgentId,
        agent_b: &AgentId,
    ) -> Result<Option<Relationship>, WorldError>;

    /// 获取 Agent 的所有关系
    async fn get_agent_relationships(
        &self,
        agent_id: &AgentId,
    ) -> Result<Vec<Relationship>, WorldError>;

    /// 获取位置信息
    async fn get_location(&self, location_id: &LocationId) -> Result<Location, WorldError>;

    /// 获取所有位置
    async fn get_all_locations(&self) -> Result<Vec<Location>, WorldError>;

    /// 生成世界状态快照
    async fn snapshot(&self) -> Result<WorldSnapshot, WorldError>;
}

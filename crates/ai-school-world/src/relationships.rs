//! 关系矩阵
//!
//! Agent 间亲密度、群组归属、关系变更。

use std::collections::HashMap;

use ai_school_core::types::{AgentId, Relationship, SimulationTime};

/// 关系管理器
pub struct RelationshipManager {
    /// (agent_a, agent_b) → Relationship，保证 agent_a < agent_b
    relationships: HashMap<(AgentId, AgentId), Relationship>,
}

impl RelationshipManager {
    pub fn new() -> Self {
        Self {
            relationships: HashMap::new(),
        }
    }

    /// 标准化键：保证 agent_a < agent_b（按 ID 排序）
    fn key(a: &AgentId, b: &AgentId) -> (AgentId, AgentId) {
        if a.0 < b.0 {
            (a.clone(), b.clone())
        } else {
            (b.clone(), a.clone())
        }
    }

    /// 获取两个 Agent 之间的关系
    pub fn get(&self, a: &AgentId, b: &AgentId) -> Option<&Relationship> {
        let key = Self::key(a, b);
        self.relationships.get(&key)
    }

    /// 获取或创建两个 Agent 之间的关系
    pub fn get_or_create(&mut self, a: &AgentId, b: &AgentId) -> &mut Relationship {
        let key = Self::key(a, b);
        self.relationships.entry(key.clone()).or_insert_with(|| Relationship {
            agent_a: key.0.clone(),
            agent_b: key.1.clone(),
            closeness: 0.0,
            trust: 0.5,
            tags: Vec::new(),
            last_interaction: None,
        })
    }

    /// 更新关系亲密度
    pub fn update_closeness(
        &mut self,
        a: &AgentId,
        b: &AgentId,
        delta: f32,
        time: &SimulationTime,
    ) {
        let rel = self.get_or_create(a, b);
        rel.closeness = (rel.closeness + delta).clamp(-1.0, 1.0);
        rel.last_interaction = Some(time.clone());
    }

    /// 更新信任度
    pub fn update_trust(&mut self, a: &AgentId, b: &AgentId, delta: f32, time: &SimulationTime) {
        let rel = self.get_or_create(a, b);
        rel.trust = (rel.trust + delta).clamp(0.0, 1.0);
        rel.last_interaction = Some(time.clone());
    }

    /// 获取某个 Agent 的所有关系
    pub fn get_agent_relationships(&self, agent_id: &AgentId) -> Vec<&Relationship> {
        self.relationships
            .values()
            .filter(|r| r.agent_a == *agent_id || r.agent_b == *agent_id)
            .collect()
    }

    /// 获取所有关系
    pub fn all_relationships(&self) -> Vec<&Relationship> {
        self.relationships.values().collect()
    }

    /// 获取与指定 Agent 关系最亲密的 N 个 Agent
    pub fn closest_agents(&self, agent_id: &AgentId, limit: usize) -> Vec<(AgentId, f32)> {
        let mut related: Vec<(AgentId, f32)> = self
            .get_agent_relationships(agent_id)
            .into_iter()
            .map(|r| {
                let other = if r.agent_a == *agent_id {
                    r.agent_b.clone()
                } else {
                    r.agent_a.clone()
                };
                (other, r.closeness)
            })
            .collect();

        related.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        related.truncate(limit);
        related
    }
}

impl Default for RelationshipManager {
    fn default() -> Self {
        Self::new()
    }
}

//! M4.2 身心发展追踪

use serde::{Deserialize, Serialize};

use ai_school_core::types::{AgentId, SimulationTime};

/// 发展指标记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentRecord {
    pub agent_id: AgentId,
    pub timestamp: SimulationTime,
    pub academic_score: f32,
    pub social_score: f32,
    pub mental_health: f32,
    pub resilience: f32,
}

/// 发展追踪器
pub struct DevelopmentTracker {
    records: Vec<DevelopmentRecord>,
}

impl DevelopmentTracker {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    /// 记录一次发展快照
    pub fn record(&mut self, record: DevelopmentRecord) {
        self.records.push(record);
    }

    /// 获取某个 Agent 的发展历史
    pub fn get_history(&self, agent_id: &AgentId) -> Vec<&DevelopmentRecord> {
        self.records
            .iter()
            .filter(|r| r.agent_id == *agent_id)
            .collect()
    }

    /// 获取某个 Agent 最新的发展状态
    pub fn latest(&self, agent_id: &AgentId) -> Option<&DevelopmentRecord> {
        self.records
            .iter()
            .rev()
            .find(|r| r.agent_id == *agent_id)
    }

    /// 导出所有记录为 JSON
    pub fn export_json(&self) -> serde_json::Value {
        serde_json::json!(self.records)
    }
}

impl Default for DevelopmentTracker {
    fn default() -> Self {
        Self::new()
    }
}

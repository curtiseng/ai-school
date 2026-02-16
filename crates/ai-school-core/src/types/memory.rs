use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::agent::AgentId;
use super::event::EventId;
use super::world::SimulationTime;

/// 记忆唯一标识符
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct MemoryId(pub Uuid);

impl MemoryId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for MemoryId {
    fn default() -> Self {
        Self::new()
    }
}

/// 记忆层级 — ADR-0005 四层记忆架构
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum MemoryLayer {
    /// 感知记忆 — 瞬时感知，当前步结束即丢弃
    Sensory,
    /// 短期记忆 — 近期事件，可能被遗忘或巩固
    ShortTerm,
    /// 长期记忆 — 重要事件，从短期记忆筛选巩固而来
    LongTerm,
    /// 语义记忆 — 高度抽象的自我认知，由反思机制生成
    Semantic,
}

/// 记忆条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub id: MemoryId,
    pub agent_id: AgentId,
    pub layer: MemoryLayer,
    /// 记忆内容（自然语言）
    pub content: String,
    /// 创建时间
    pub timestamp: SimulationTime,
    /// 重要性分数 (0.0 ~ 1.0)
    pub importance: f32,
    /// 情绪效价 (-1.0 ~ +1.0)
    pub emotion_valence: f32,
    /// 关联事件 ID
    pub event_id: Option<EventId>,
    /// 标签
    pub tags: Vec<String>,
    /// 访问次数（用于巩固/遗忘判断）
    pub access_count: u32,
    /// 最后访问时间
    pub last_accessed: SimulationTime,
}

/// 记忆检索查询
#[derive(Debug, Clone)]
pub struct MemoryQuery {
    /// 查询文本（用于语义相似度检索）
    pub query_text: String,
    /// 限制层级
    pub layer_filter: Option<MemoryLayer>,
    /// 标签过滤
    pub tag_filter: Vec<String>,
    /// 时间范围过滤（最早时间）
    pub since: Option<SimulationTime>,
    /// 最大返回数量
    pub limit: usize,
}

/// 带评分的记忆检索结果
/// Score = α×relevance + β×recency + γ×importance
#[derive(Debug, Clone)]
pub struct ScoredMemory {
    pub memory: Memory,
    /// 综合评分
    pub score: f32,
    /// 语义相关性分数
    pub relevance: f32,
    /// 时间衰减分数
    pub recency: f32,
    /// 重要性分数
    pub importance_score: f32,
}

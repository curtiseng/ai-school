use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::agent::AgentId;
use super::world::{LocationId, SimulationTime};

/// 行为意图 — Agent LLM 输出
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BehaviorIntent {
    /// 发起 Agent
    pub agent_id: AgentId,
    /// 意图描述（自然语言）
    pub description: String,
    /// 目标位置（如果涉及移动）
    pub target_location: Option<LocationId>,
    /// 交互目标（如果涉及其他 Agent）
    pub target_agents: Vec<AgentId>,
    /// 意图类型
    pub intent_type: IntentType,
}

/// 意图类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum IntentType {
    /// 移动到某处
    Move,
    /// 发起对话
    Talk,
    /// 独自学习
    Study,
    /// 加入活动
    JoinActivity,
    /// 休息
    Rest,
    /// 发起合作
    Collaborate,
    /// 表达不满/争执
    Confront,
    /// 反思/独处
    Reflect,
    /// 其他
    Other,
}

/// 感知输入 — Agent 从环境中获取的信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Perception {
    /// 当前位置的 Agent 列表
    pub nearby_agents: Vec<AgentId>,
    /// 可观察到的活动
    pub observable_activities: Vec<String>,
    /// 环境描述
    pub environment_description: String,
    /// 最近的事件提醒
    pub recent_events: Vec<String>,
}

/// 情境上下文 — 组装 LLM 决策请求的完整输入
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SituationContext {
    /// Agent ID
    pub agent_id: AgentId,
    /// 当前仿真时间
    pub time: SimulationTime,
    /// 感知信息
    pub perception: Perception,
    /// 检索到的相关记忆
    pub relevant_memories: Vec<String>,
    /// 当前情绪摘要
    pub emotional_summary: String,
    /// 人格描述
    pub personality_description: String,
    /// 职业志向摘要
    pub career_summary: String,
}

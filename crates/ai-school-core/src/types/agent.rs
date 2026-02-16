use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::career::CareerAspiration;
use super::personality::PersonalityParams;
use super::world::{LocationId, SimulationTime};

/// Agent 唯一标识符
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct AgentId(pub Uuid);

impl AgentId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for AgentId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for AgentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Agent 配置 — 创建 Agent 时的输入参数
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AgentConfig {
    pub name: String,
    pub personality: PersonalityParams,
    pub career_aspiration: CareerAspiration,
    /// 可选的背景描述
    pub background: Option<String>,
    /// Agent 年龄（影响人格稳定度）
    pub age: u8,
}

/// Agent 当前活动状态
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum AgentActivity {
    /// 在上课
    Studying { subject: String },
    /// 社交互动
    Socializing { with: Vec<AgentId> },
    /// 休息
    Resting,
    /// 独处思考
    Reflecting,
    /// 参加活动
    Activity { name: String },
    /// 移动中
    Moving { to: LocationId },
    /// 困扰中
    Troubled { reason: String },
}

/// Agent 情绪状态
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EmotionalState {
    /// 整体情绪效价 (-1.0 消极 ~ +1.0 积极)
    pub valence: f32,
    /// 情绪唤醒度 (0.0 平静 ~ 1.0 激动)
    pub arousal: f32,
    /// 压力水平 (0.0 ~ 1.0)
    pub stress: f32,
}

impl Default for EmotionalState {
    fn default() -> Self {
        Self {
            valence: 0.3,
            arousal: 0.3,
            stress: 0.2,
        }
    }
}

/// Agent 能力指标
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AbilityMetrics {
    /// 学业能力 (0.0 ~ 1.0)
    pub academic: f32,
    /// 社交能力 (0.0 ~ 1.0)
    pub social: f32,
    /// 心理韧性 (0.0 ~ 1.0)
    pub resilience: f32,
    /// 创造力 (0.0 ~ 1.0)
    pub creativity: f32,
}

impl Default for AbilityMetrics {
    fn default() -> Self {
        Self {
            academic: 0.5,
            social: 0.5,
            resilience: 0.5,
            creativity: 0.5,
        }
    }
}

/// Agent 完整运行时状态
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AgentState {
    pub id: AgentId,
    pub config: AgentConfig,
    pub location: LocationId,
    pub activity: AgentActivity,
    pub emotion: EmotionalState,
    pub abilities: AbilityMetrics,
    /// 最近的想法/状态摘要
    pub current_thought: Option<String>,
    /// Agent 创建时间
    pub created_at: SimulationTime,
    /// 最后更新时间
    pub last_updated: SimulationTime,
}

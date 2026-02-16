use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::agent::AgentId;
use super::world::SimulationTime;

/// 事件唯一标识符
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct EventId(pub Uuid);

impl EventId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for EventId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for EventId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 事件类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum EventType {
    /// 日常活动（上课、吃饭等）
    Routine,
    /// 社交互动
    SocialInteraction,
    /// 学业事件（考试、作业等）
    Academic,
    /// 冲突事件
    Conflict,
    /// 合作事件
    Cooperation,
    /// 特殊事件（运动会、社团招新等）
    SpecialEvent,
    /// 系统事件（时间推进、课程表变化等）
    System,
    /// 用户干预事件
    Intervention,
}

/// 状态变更指令 — Game Master 输出的结构化变更
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StateChange {
    /// 变更目标（如 "agent:小明.emotion.valence"）
    pub target: String,
    /// 变更类型
    pub change_type: ChangeType,
    /// 变更值
    pub value: serde_json::Value,
}

/// 变更类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum ChangeType {
    /// 增量变更（如 emotion.valence += -0.2）
    Delta,
    /// 绝对设置（如 location = "library"）
    Set,
    /// 追加到列表
    Append,
}

/// 事件触发器类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum EventTrigger {
    /// 时间触发（课程表事件等）
    TimeSchedule,
    /// Agent 行为触发
    AgentAction,
    /// 状态阈值触发（如关系破裂）
    ThresholdReached,
    /// 用户手动触发
    UserIntervention,
    /// 随机事件
    Random,
}

/// 仿真事件 — ADR-0004 叙事-数据双视图
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationEvent {
    pub id: EventId,
    pub event_type: EventType,
    pub trigger: EventTrigger,
    pub timestamp: SimulationTime,
    /// 涉及的 Agent
    pub involved_agents: Vec<AgentId>,
    /// 叙事描述（自然语言）
    pub narrative: String,
    /// 结构化状态变更
    pub state_changes: Vec<StateChange>,
    /// 事件强度 (0.0 ~ 1.0)
    pub intensity: f32,
}

/// 预设事件模板
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum PresetEvent {
    /// 期中考试
    MidtermExam,
    /// 社团招新
    ClubRecruitment,
    /// 运动会
    SportsMeet,
    /// 友谊冲突
    FriendshipConflict { agent_a: AgentId, agent_b: AgentId },
    /// 新同学转入
    NewStudent { name: String },
    /// 老师表扬
    TeacherPraise { target: AgentId },
    /// 老师批评
    TeacherCriticism { target: AgentId },
    /// 自定义事件
    Custom { description: String, scope: EventScope },
}

/// 事件影响范围
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum EventScope {
    /// 全局事件
    Global,
    /// 影响指定 Agent
    Agents(Vec<AgentId>),
    /// 影响指定区域内的 Agent
    Location(super::world::LocationId),
}

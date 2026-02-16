//! M3.2 用户干预机制

use serde::{Deserialize, Serialize};

use ai_school_core::types::{
    AgentId, EventId, EventTrigger, EventType, PresetEvent,
    SimulationEvent, SimulationTime, StateChange,
};

/// 干预类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InterventionType {
    /// 对话干预（以角色身份与 Agent 对话）
    Chat {
        agent_id: AgentId,
        role: String,
        message: String,
    },
    /// 参数调整
    ParameterChange {
        parameter: EnvironmentParameter,
        value: f32,
    },
    /// 事件触发
    EventTriggerIntervention {
        event: PresetEvent,
    },
}

/// 环境参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnvironmentParameter {
    /// 课程难度
    CourseDifficulty,
    /// 社交密度
    SocialDensity,
    /// 竞争压力
    CompetitivePressure,
    /// 随机事件频率
    RandomEventFrequency,
}

/// 干预日志记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionLog {
    pub id: EventId,
    pub timestamp: SimulationTime,
    pub intervention_type: InterventionType,
    pub affected_agents: Vec<AgentId>,
    pub description: String,
}

/// 干预管理器
pub struct InterventionManager {
    pub logs: Vec<InterventionLog>,
}

impl InterventionManager {
    pub fn new() -> Self {
        Self { logs: Vec::new() }
    }

    /// 应用参数调整
    pub fn apply_parameter_change(
        &mut self,
        parameter: &EnvironmentParameter,
        value: f32,
        timestamp: &SimulationTime,
    ) -> Vec<StateChange> {
        let log = InterventionLog {
            id: EventId::new(),
            timestamp: timestamp.clone(),
            intervention_type: InterventionType::ParameterChange {
                parameter: parameter.clone(),
                value,
            },
            affected_agents: Vec::new(),
            description: format!("参数调整: {:?} = {value}", parameter),
        };
        self.logs.push(log);

        // Return state changes based on parameter type
        match parameter {
            EnvironmentParameter::CourseDifficulty => {
                // Modify all subjects' difficulty
                vec![] // TODO: implement
            }
            _ => vec![],
        }
    }

    /// 创建预设事件的仿真事件
    pub fn trigger_preset_event(
        &mut self,
        event: &PresetEvent,
        timestamp: &SimulationTime,
    ) -> SimulationEvent {
        let (narrative, involved, event_type) = match event {
            PresetEvent::MidtermExam => (
                "期中考试开始了！所有同学紧张地准备着。".to_string(),
                vec![],
                EventType::Academic,
            ),
            PresetEvent::ClubRecruitment => (
                "社团招新活动开始了，各个社团在操场设立了展位。".to_string(),
                vec![],
                EventType::SpecialEvent,
            ),
            PresetEvent::SportsMeet => (
                "学校运动会拉开帷幕！同学们热情高涨。".to_string(),
                vec![],
                EventType::SpecialEvent,
            ),
            PresetEvent::FriendshipConflict { agent_a, agent_b } => (
                "两位同学之间产生了矛盾。".to_string(),
                vec![agent_a.clone(), agent_b.clone()],
                EventType::Conflict,
            ),
            PresetEvent::TeacherPraise { target } => (
                "老师在全班面前表扬了一位同学。".to_string(),
                vec![target.clone()],
                EventType::Academic,
            ),
            PresetEvent::TeacherCriticism { target } => (
                "老师批评了一位同学的表现。".to_string(),
                vec![target.clone()],
                EventType::Academic,
            ),
            PresetEvent::NewStudent { name } => (
                format!("班级来了一位新同学：{name}。"),
                vec![],
                EventType::SpecialEvent,
            ),
            PresetEvent::Custom { description, .. } => (
                description.clone(),
                vec![],
                EventType::SpecialEvent,
            ),
        };

        let log = InterventionLog {
            id: EventId::new(),
            timestamp: timestamp.clone(),
            intervention_type: InterventionType::EventTriggerIntervention {
                event: event.clone(),
            },
            affected_agents: involved.clone(),
            description: narrative.clone(),
        };
        self.logs.push(log);

        SimulationEvent {
            id: EventId::new(),
            event_type,
            trigger: EventTrigger::UserIntervention,
            timestamp: timestamp.clone(),
            involved_agents: involved,
            narrative,
            state_changes: Vec::new(),
            intensity: 0.6,
        }
    }

    /// 导出干预日志
    pub fn export_logs(&self) -> serde_json::Value {
        serde_json::json!(self.logs)
    }
}

impl Default for InterventionManager {
    fn default() -> Self {
        Self::new()
    }
}

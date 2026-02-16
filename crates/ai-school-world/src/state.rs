//! 世界状态管理器（核心）
//!
//! 聚合所有子系统状态，状态变更的验证与应用。

use std::collections::HashMap;

use rand::seq::SliceRandom;
use tracing::{debug, warn};

use ai_school_core::error::WorldError;
use ai_school_core::types::{
    AgentActivity, AgentId, AgentState, ChangeType, Location, LocationId,
    SimulationEvent, StateChange, WorldSnapshot,
};

use crate::time::TimeEvent;

use crate::campus::create_default_campus;
use crate::curriculum::{create_default_schedule, create_default_subjects, ClassSchedule, Subject};
use crate::relationships::RelationshipManager;
use crate::social::{create_default_clubs, Club};
use crate::time::SimulationClock;

/// 世界状态管理器 — ADR-0003 结构化世界状态管理器
pub struct WorldState {
    /// 校园地图
    pub locations: Vec<Location>,
    /// Agent 状态表
    pub agents: HashMap<AgentId, AgentState>,
    /// 关系矩阵
    pub relationships: RelationshipManager,
    /// 课程表
    pub schedule: Vec<ClassSchedule>,
    /// 学科列表
    pub subjects: Vec<Subject>,
    /// 社团列表
    pub clubs: Vec<Club>,
    /// 仿真时钟
    pub clock: SimulationClock,
    /// 事件日志
    pub event_log: Vec<SimulationEvent>,
}

impl WorldState {
    /// 创建默认世界状态
    pub fn new(time_step_hours: u32) -> Self {
        Self {
            locations: create_default_campus(),
            agents: HashMap::new(),
            relationships: RelationshipManager::new(),
            schedule: create_default_schedule(),
            subjects: create_default_subjects(),
            clubs: create_default_clubs(),
            clock: SimulationClock::new(time_step_hours),
            event_log: Vec::new(),
        }
    }

    /// 添加 Agent 到世界
    pub fn add_agent(&mut self, agent: AgentState) {
        debug!(agent_id = %agent.id, name = %agent.config.name, "Agent added to world");
        self.agents.insert(agent.id.clone(), agent);
    }

    /// 获取 Agent 状态
    pub fn get_agent(&self, id: &AgentId) -> Result<&AgentState, WorldError> {
        self.agents
            .get(id)
            .ok_or_else(|| WorldError::AgentNotFound(id.clone()))
    }

    /// 获取可变 Agent 状态
    pub fn get_agent_mut(&mut self, id: &AgentId) -> Result<&mut AgentState, WorldError> {
        self.agents
            .get_mut(id)
            .ok_or_else(|| WorldError::AgentNotFound(id.clone()))
    }

    /// 获取指定位置的所有 Agent
    pub fn agents_at_location(&self, location_id: &LocationId) -> Vec<&AgentState> {
        self.agents
            .values()
            .filter(|a| a.location == *location_id)
            .collect()
    }

    /// 获取位置信息
    pub fn get_location(&self, id: &LocationId) -> Result<&Location, WorldError> {
        self.locations
            .iter()
            .find(|l| l.id == *id)
            .ok_or_else(|| WorldError::LocationNotFound(id.clone()))
    }

    /// 获取当前时间段的课程
    pub fn current_class(&self) -> Option<&ClassSchedule> {
        let time = self.clock.current_time();
        let period = match time.hour {
            8 => Some(1),
            9 => Some(2),
            11 => Some(3),
            14 => Some(4),
            15 => Some(5),
            _ => None,
        };

        period.and_then(|p| {
            self.schedule
                .iter()
                .find(|c| c.day_of_week == time.day_of_week && c.period == p)
        })
    }

    /// 应用状态变更 — 单一入口
    /// 验证合法性 → 执行状态更新 → 记录事件日志
    pub fn apply_state_changes(
        &mut self,
        changes: &[StateChange],
    ) -> Result<Vec<String>, WorldError> {
        let mut warnings = Vec::new();

        for change in changes {
            match self.apply_single_change(change) {
                Ok(()) => {
                    debug!(target = %change.target, "State change applied");
                }
                Err(e) => {
                    warn!(target = %change.target, error = %e, "State change failed");
                    warnings.push(format!("Failed to apply change to {}: {e}", change.target));
                }
            }
        }

        Ok(warnings)
    }

    fn apply_single_change(&mut self, change: &StateChange) -> Result<(), WorldError> {
        let parts: Vec<&str> = change.target.split('.').collect();
        if parts.len() < 2 {
            return Err(WorldError::InvalidStateChange(format!(
                "Invalid target format: {}",
                change.target
            )));
        }

        match parts[0] {
            target if target.starts_with("agent:") => {
                let agent_key = &target[6..];
                // Support both name and UUID lookup for robustness
                let agent = if let Some(a) = self.agents.values_mut().find(|a| a.config.name == agent_key) {
                    a
                } else if let Ok(uuid) = uuid::Uuid::parse_str(agent_key) {
                    self.agents.get_mut(&AgentId(uuid)).ok_or_else(|| {
                        WorldError::InvalidStateChange(format!("Agent not found: {agent_key}"))
                    })?
                } else {
                    return Err(WorldError::InvalidStateChange(format!("Agent not found: {agent_key}")));
                };

                match parts[1] {
                    "emotion" if parts.len() > 2 => match parts[2] {
                        "valence" => {
                            Self::apply_numeric_change(&mut agent.emotion.valence, change, -1.0, 1.0);
                        }
                        "arousal" => {
                            Self::apply_numeric_change(&mut agent.emotion.arousal, change, 0.0, 1.0);
                        }
                        "stress" => {
                            Self::apply_numeric_change(&mut agent.emotion.stress, change, 0.0, 1.0);
                        }
                        _ => return Err(WorldError::InvalidStateChange(format!("Unknown emotion field: {}", parts[2]))),
                    },
                    "location" => {
                        if let Some(loc_str) = change.value.as_str() {
                            agent.location = LocationId(loc_str.to_string());
                        }
                    }
                    _ => {
                        return Err(WorldError::InvalidStateChange(format!(
                            "Unknown agent field: {}",
                            parts[1]
                        )));
                    }
                }
            }
            target if target.starts_with("relationship[") => {
                // relationship[小明,小红].closeness
                if let (Some(start), Some(end)) = (target.find('['), target.find(']')) {
                    let agents_str = &target[start + 1..end];
                    let agent_names: Vec<&str> = agents_str.split(',').collect();
                    if agent_names.len() == 2 {
                        let time = self.clock.current_time().clone();
                        if let Some(delta) = change.value.as_f64() {
                            // Find agent IDs by name
                            let id_a = self.agents.values().find(|a| a.config.name == agent_names[0]).map(|a| a.id.clone());
                            let id_b = self.agents.values().find(|a| a.config.name == agent_names[1]).map(|a| a.id.clone());
                            if let (Some(id_a), Some(id_b)) = (id_a, id_b) {
                                match parts.get(1) {
                                    Some(&"closeness") => {
                                        self.relationships.update_closeness(&id_a, &id_b, delta as f32, &time);
                                    }
                                    Some(&"trust") => {
                                        self.relationships.update_trust(&id_a, &id_b, delta as f32, &time);
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                return Err(WorldError::InvalidStateChange(format!(
                    "Unknown target prefix: {}",
                    parts[0]
                )));
            }
        }

        Ok(())
    }

    fn apply_numeric_change(field: &mut f32, change: &StateChange, min: f32, max: f32) {
        if let Some(value) = change.value.as_f64() {
            match change.change_type {
                ChangeType::Delta => *field = (*field + value as f32).clamp(min, max),
                ChangeType::Set => *field = (value as f32).clamp(min, max),
                ChangeType::Append => {} // Not applicable for numeric
            }
        }
    }

    /// 处理时间事件：根据时间段自动移动 Agent 到对应位置
    pub fn process_time_events(&mut self, events: &[TimeEvent]) {
        let mut rng = rand::thread_rng();

        for event in events {
            match event {
                TimeEvent::ClassStart { period } => {
                    let time = self.clock.current_time();
                    let subject = self
                        .schedule
                        .iter()
                        .find(|c| c.day_of_week == time.day_of_week && c.period == *period)
                        .map(|c| c.subject.clone());

                    if let Some(subject_name) = subject {
                        let classroom = self
                            .subjects
                            .iter()
                            .find(|s| s.name == subject_name)
                            .map(|s| s.classroom.clone())
                            .unwrap_or(LocationId("classroom_math".to_string()));

                        let subject_for_activity = subject_name.clone();
                        debug!(subject = %subject_name, location = %classroom.0, "Moving agents to class");
                        for agent in self.agents.values_mut() {
                            agent.location = classroom.clone();
                            agent.activity = AgentActivity::Studying {
                                subject: subject_for_activity.clone(),
                            };
                        }
                    }
                }
                TimeEvent::Break => {
                    let break_locations = [
                        LocationId("rest_area".to_string()),
                        LocationId("hallway".to_string()),
                        LocationId("playground".to_string()),
                    ];
                    for agent in self.agents.values_mut() {
                        let loc = break_locations.choose(&mut rng).unwrap().clone();
                        agent.location = loc;
                        agent.activity = AgentActivity::Resting;
                    }
                    debug!("Break: agents moved to rest areas");
                }
                TimeEvent::LunchBreak | TimeEvent::Dinner => {
                    for agent in self.agents.values_mut() {
                        agent.location = LocationId("cafeteria".to_string());
                        agent.activity = AgentActivity::Resting;
                    }
                    debug!("Meal time: agents moved to cafeteria");
                }
                TimeEvent::FreeTime => {
                    let free_locations = [
                        LocationId("library".to_string()),
                        LocationId("playground".to_string()),
                        LocationId("club_room".to_string()),
                        LocationId("rest_area".to_string()),
                        LocationId("study_room".to_string()),
                    ];
                    for agent in self.agents.values_mut() {
                        let loc = free_locations.choose(&mut rng).unwrap().clone();
                        agent.location = loc;
                        agent.activity = AgentActivity::Activity {
                            name: "课外活动".to_string(),
                        };
                    }
                    debug!("Free time: agents dispersed to various locations");
                }
                TimeEvent::EveningStudy => {
                    let study_locations = [
                        LocationId("study_room".to_string()),
                        LocationId("library".to_string()),
                        LocationId("classroom_math".to_string()),
                    ];
                    for agent in self.agents.values_mut() {
                        let loc = study_locations.choose(&mut rng).unwrap().clone();
                        agent.location = loc;
                        agent.activity = AgentActivity::Studying {
                            subject: "自习".to_string(),
                        };
                    }
                    debug!("Evening study: agents moved to study areas");
                }
                TimeEvent::Bedtime | TimeEvent::NewDay => {
                    for agent in self.agents.values_mut() {
                        agent.location = LocationId("dormitory".to_string());
                        agent.activity = AgentActivity::Resting;
                    }
                    debug!("Bedtime: agents moved to dormitory");
                }
                TimeEvent::Weekend => {
                    let weekend_locations = [
                        LocationId("library".to_string()),
                        LocationId("playground".to_string()),
                        LocationId("dormitory".to_string()),
                        LocationId("rest_area".to_string()),
                        LocationId("club_room".to_string()),
                    ];
                    for agent in self.agents.values_mut() {
                        let loc = weekend_locations.choose(&mut rng).unwrap().clone();
                        agent.location = loc;
                        agent.activity = AgentActivity::Resting;
                    }
                    debug!("Weekend: agents dispersed freely");
                }
                _ => {}
            }
        }
    }

    /// 生成世界状态快照
    pub fn snapshot(&self) -> WorldSnapshot {
        WorldSnapshot {
            time: self.clock.current_time().clone(),
            agents: self.agents.values().cloned().collect(),
            relationships: self
                .relationships
                .all_relationships()
                .into_iter()
                .cloned()
                .collect(),
            active_events: Vec::new(),
        }
    }

    /// 生成情境描述（结构化 → 自然语言）
    pub fn describe_situation(&self, agent_id: &AgentId) -> String {
        let agent = match self.agents.get(agent_id) {
            Some(a) => a,
            None => return "未知情境".to_string(),
        };

        let location = self
            .locations
            .iter()
            .find(|l| l.id == agent.location)
            .map(|l| l.name.as_str())
            .unwrap_or("未知地点");

        let nearby: Vec<&str> = self
            .agents_at_location(&agent.location)
            .iter()
            .filter(|a| a.id != *agent_id)
            .map(|a| a.config.name.as_str())
            .collect();

        let period = self.clock.current_period_description();
        let time_desc = self.clock.current_time().display();

        let mut desc = format!("{time_desc}，{period}。你在{location}。");

        if !nearby.is_empty() {
            desc.push_str(&format!("附近有: {}。", nearby.join("、")));
        } else {
            desc.push_str("周围没有其他人。");
        }

        if let Some(class) = self.current_class() {
            desc.push_str(&format!("当前课程: {}。", class.subject));
        }

        desc
    }
}

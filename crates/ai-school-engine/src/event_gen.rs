//! M3.3 事件与冲突生成

use rand::Rng;

use ai_school_core::types::{
    AgentId, EventId, EventTrigger, EventType, SimulationEvent, SimulationTime,
};
use ai_school_world::state::WorldState;

/// 自动事件生成器
pub struct EventGenerator {
    pub random_event_frequency: f32,
}

impl EventGenerator {
    pub fn new(frequency: f32) -> Self {
        Self {
            random_event_frequency: frequency,
        }
    }

    /// 基于世界状态检测并生成事件
    pub fn check_and_generate(&self, world: &WorldState) -> Vec<SimulationEvent> {
        let mut events = Vec::new();
        let current_time = world.clock.current_time().clone();

        // 检查关系阈值触发
        for rel in world.relationships.all_relationships() {
            if rel.closeness < -0.7 {
                events.push(SimulationEvent {
                    id: EventId::new(),
                    event_type: EventType::Conflict,
                    trigger: EventTrigger::ThresholdReached,
                    timestamp: current_time.clone(),
                    involved_agents: vec![rel.agent_a.clone(), rel.agent_b.clone()],
                    narrative: format!(
                        "关系紧张：两位同学之间的矛盾加剧，亲密度降至 {:.1}",
                        rel.closeness
                    ),
                    state_changes: Vec::new(),
                    intensity: 0.7,
                });
            }
        }

        // 随机事件
        let mut rng = rand::thread_rng();
        if rng.r#gen::<f32>() < self.random_event_frequency {
            let agent_ids: Vec<AgentId> = world.agents.keys().cloned().collect();
            if !agent_ids.is_empty() {
                let idx = rng.r#gen::<usize>() % agent_ids.len();
                let random_agent = agent_ids[idx].clone();
                events.push(self.generate_random_event(&random_agent, &current_time));
            }
        }

        events
    }

    fn generate_random_event(
        &self,
        agent_id: &AgentId,
        time: &SimulationTime,
    ) -> SimulationEvent {
        let mut rng = rand::thread_rng();
        let templates = [
            "一个意外的机会出现了",
            "遇到了一位有趣的新朋友",
            "在走廊上捡到一本有意思的书",
            "收到了一个好消息",
            "遭遇了一个小挫折",
        ];

        let idx = rng.r#gen::<usize>() % templates.len();
        let narrative = templates[idx];

        SimulationEvent {
            id: EventId::new(),
            event_type: EventType::SpecialEvent,
            trigger: EventTrigger::Random,
            timestamp: time.clone(),
            involved_agents: vec![agent_id.clone()],
            narrative: narrative.to_string(),
            state_changes: Vec::new(),
            intensity: 0.3,
        }
    }
}

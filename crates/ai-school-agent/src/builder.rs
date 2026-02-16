//! Agent 配置构建器 — 用于 F3 配置面板

use rand::Rng;

use ai_school_core::types::{
    AgentConfig, AgentId, AgentState, AgentActivity, CareerAspiration, CareerCategory,
    EmotionalState, AbilityMetrics, LocationId, PersonalityParams, SimulationTime,
};
use crate::career::default_subject_preferences;
use crate::personality::generate_diverse_personalities;

/// Agent 构建器
pub struct AgentBuilder {
    name: Option<String>,
    personality: Option<PersonalityParams>,
    career: Option<CareerAspiration>,
    background: Option<String>,
    age: u8,
}

impl AgentBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            personality: None,
            career: None,
            background: None,
            age: 16,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn personality(mut self, params: PersonalityParams) -> Self {
        self.personality = Some(params);
        self
    }

    pub fn career(mut self, career: CareerAspiration) -> Self {
        self.career = Some(career);
        self
    }

    pub fn background(mut self, bg: impl Into<String>) -> Self {
        self.background = Some(bg.into());
        self
    }

    pub fn age(mut self, age: u8) -> Self {
        self.age = age;
        self
    }

    /// 构建 AgentState
    pub fn build(self, start_time: &SimulationTime) -> AgentState {
        let personality = self.personality.unwrap_or_else(|| {
            let mut rng = rand::thread_rng();
            PersonalityParams::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            )
        });

        let career = self.career.unwrap_or_else(|| CareerAspiration {
            ideal_career: "探索中".to_string(),
            category: CareerCategory::Other("未定".to_string()),
            subject_preferences: Vec::new(),
            clarity: 0.3,
        });

        let config = AgentConfig {
            name: self.name.unwrap_or_else(|| "未命名学生".to_string()),
            personality,
            career_aspiration: career,
            background: self.background,
            age: self.age,
        };

        AgentState {
            id: AgentId::new(),
            location: LocationId("dormitory".to_string()),
            activity: AgentActivity::Resting,
            emotion: EmotionalState::default(),
            abilities: AbilityMetrics::default(),
            current_thought: None,
            created_at: start_time.clone(),
            last_updated: start_time.clone(),
            config,
        }
    }
}

impl Default for AgentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// 批量生成随机 Agent 群体
pub fn generate_random_agents(count: usize, start_time: &SimulationTime) -> Vec<AgentState> {
    let names = [
        "小明", "小红", "小华", "小丽", "小强",
        "小芳", "小刚", "小美", "小龙", "小雪",
    ];

    let careers = [
        ("软件工程师", CareerCategory::Technology),
        ("心理咨询师", CareerCategory::SocialWork),
        ("医生", CareerCategory::Medicine),
        ("艺术家", CareerCategory::Arts),
        ("科研人员", CareerCategory::Science),
        ("教师", CareerCategory::Education),
        ("企业家", CareerCategory::Business),
        ("律师", CareerCategory::Law),
        ("产品经理", CareerCategory::Business),
        ("数据科学家", CareerCategory::Science),
    ];

    let personalities = generate_diverse_personalities(count);
    let mut rng = rand::thread_rng();

    personalities
        .into_iter()
        .enumerate()
        .map(|(i, personality)| {
            let name = names[i % names.len()];
            let (career_name, career_cat) = &careers[i % careers.len()];

            let career = CareerAspiration {
                ideal_career: career_name.to_string(),
                category: career_cat.clone(),
                subject_preferences: default_subject_preferences(&CareerAspiration {
                    ideal_career: career_name.to_string(),
                    category: career_cat.clone(),
                    subject_preferences: Vec::new(),
                    clarity: 0.5,
                }),
                clarity: rng.gen_range(0.3..0.8),
            };

            AgentBuilder::new()
                .name(name)
                .personality(personality)
                .career(career)
                .age(rng.gen_range(15..18))
                .build(start_time)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_builder() {
        let time = SimulationTime::new();
        let agent = AgentBuilder::new()
            .name("测试学生")
            .personality(PersonalityParams::new(0.5, -0.3, 0.7, -0.2))
            .age(16)
            .build(&time);

        assert_eq!(agent.config.name, "测试学生");
        assert_eq!(agent.config.age, 16);
    }

    #[test]
    fn test_generate_random_agents() {
        let time = SimulationTime::new();
        let agents = generate_random_agents(5, &time);
        assert_eq!(agents.len(), 5);

        // All should have unique IDs
        let ids: std::collections::HashSet<_> = agents.iter().map(|a| a.id.0).collect();
        assert_eq!(ids.len(), 5);
    }
}

//! Game Master 仲裁层 — ADR-0003 LLM 调用点 #2
//!
//! 验证行为合理性、仲裁多 Agent 交互、翻译自然语言→结构化 StateChange。

use serde::{Deserialize, Serialize};

use ai_school_core::error::SimulationError;
use ai_school_core::traits::llm::{ChatMessage, CompletionRequest, LlmProvider, MessageRole};
use ai_school_core::types::{BehaviorIntent, EventType, StateChange};

use ai_school_world::state::WorldState;

/// Game Master 仲裁输出
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMasterOutput {
    pub event_type: EventType,
    pub intensity: f32,
    pub state_changes: Vec<StateChange>,
    pub narrative: String,
}

/// Game Master
pub struct GameMaster {
    /// 是否使用 LLM 进行仲裁（关闭时使用简单规则）
    use_llm: bool,
}

impl GameMaster {
    pub fn new() -> Self {
        Self { use_llm: true }
    }

    /// 仲裁 Agent 行为意图
    pub async fn arbitrate<L: LlmProvider>(
        &self,
        intents: &[BehaviorIntent],
        world: &WorldState,
        llm: &L,
    ) -> Result<GameMasterOutput, SimulationError> {
        if intents.is_empty() {
            return Ok(GameMasterOutput {
                event_type: EventType::System,
                intensity: 0.0,
                state_changes: Vec::new(),
                narrative: "无 Agent 行为".to_string(),
            });
        }

        if self.use_llm && intents.len() > 1 {
            self.llm_arbitrate(intents, world, llm).await
        } else {
            Ok(self.simple_arbitrate(intents, world))
        }
    }

    /// 简单规则仲裁（不调用 LLM）
    fn simple_arbitrate(
        &self,
        intents: &[BehaviorIntent],
        _world: &WorldState,
    ) -> GameMasterOutput {
        let narratives: Vec<String> = intents
            .iter()
            .map(|i| i.description.clone())
            .collect();

        let event_type = if intents.iter().any(|i| {
            matches!(
                i.intent_type,
                ai_school_core::types::IntentType::Confront
            )
        }) {
            EventType::Conflict
        } else if intents.iter().any(|i| {
            matches!(
                i.intent_type,
                ai_school_core::types::IntentType::Talk
                    | ai_school_core::types::IntentType::Collaborate
            )
        }) {
            EventType::SocialInteraction
        } else if intents.iter().any(|i| {
            matches!(
                i.intent_type,
                ai_school_core::types::IntentType::Study
            )
        }) {
            EventType::Academic
        } else {
            EventType::Routine
        };

        GameMasterOutput {
            event_type,
            intensity: 0.3,
            state_changes: Vec::new(),
            narrative: narratives.join(" "),
        }
    }

    /// LLM 仲裁（调用点 #2）
    async fn llm_arbitrate<L: LlmProvider>(
        &self,
        intents: &[BehaviorIntent],
        world: &WorldState,
        llm: &L,
    ) -> Result<GameMasterOutput, SimulationError> {
        let time_desc = world.clock.current_time().display();

        let intents_desc: String = intents
            .iter()
            .map(|i| format!("- Agent {}: {}", i.agent_id, i.description))
            .collect::<Vec<_>>()
            .join("\n");

        let system = r#"你是 AI School 的 Game Master（游戏主持人）。你的职责是：
1. 评估所有 Agent 的行为意图是否合理
2. 当多个 Agent 的行为产生交互时，仲裁结果
3. 将结果翻译为结构化的状态变更

请以 JSON 格式输出：
{
  "event_type": "Routine|SocialInteraction|Academic|Conflict|Cooperation|SpecialEvent",
  "intensity": 0.0-1.0,
  "state_changes": [
    {"target": "agent:名字.emotion.valence", "change_type": "Delta", "value": -0.1}
  ],
  "narrative": "描述发生了什么（1-2句话）"
}"#;

        let user_msg = format!(
            "当前时间: {time_desc}\n\nAgent 行为意图:\n{intents_desc}\n\n请仲裁这些行为的结果。"
        );

        let request = CompletionRequest {
            system: system.to_string(),
            messages: vec![ChatMessage {
                role: MessageRole::User,
                content: user_msg,
            }],
            temperature: Some(0.5),
            max_tokens: Some(500),
        };

        let schema = serde_json::json!({
            "type": "object",
            "properties": {
                "event_type": {"type": "string"},
                "intensity": {"type": "number"},
                "state_changes": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "target": {"type": "string"},
                            "change_type": {"type": "string"},
                            "value": {}
                        }
                    }
                },
                "narrative": {"type": "string"}
            },
            "required": ["event_type", "intensity", "narrative"]
        });

        match llm.complete_structured::<GameMasterOutput>(&request, &schema).await {
            Ok(output) => Ok(output),
            Err(e) => {
                tracing::warn!(error = %e, "LLM GM arbitration failed, falling back to simple rules");
                Ok(self.simple_arbitrate(intents, world))
            }
        }
    }
}

impl Default for GameMaster {
    fn default() -> Self {
        Self::new()
    }
}

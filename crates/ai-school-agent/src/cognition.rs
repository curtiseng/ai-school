//! M1.3 认知与行为框架
//!
//! 感知-思考-行动循环。**不直接调用 LLM**，只组装决策输入。

use ai_school_core::traits::llm::{ChatMessage, CompletionRequest, MessageRole};
use ai_school_core::types::{AgentState, BehaviorIntent, IntentType, SituationContext};

use crate::career::CareerDatabase;
use crate::personality::personality_description;

/// 认知处理器 — 组装 Agent 的 LLM 决策请求
///
/// 不调用 LLM，而是返回 `CompletionRequest`，实际调用由 engine 执行。
pub struct CognitionProcessor;

impl CognitionProcessor {
    /// 感知阶段：从情境中提取与人格相关的信息
    pub fn perceive(agent: &AgentState, context: &SituationContext) -> String {
        let mut perception = String::new();

        perception.push_str(&format!(
            "当前时间: {}\n",
            context.time.display()
        ));

        perception.push_str(&format!(
            "你在: {}\n",
            agent.location.0
        ));

        if !context.perception.nearby_agents.is_empty() {
            perception.push_str(&format!(
                "附近有 {} 个同学\n",
                context.perception.nearby_agents.len()
            ));
        }

        if !context.perception.observable_activities.is_empty() {
            perception.push_str(&format!(
                "正在进行的活动: {}\n",
                context.perception.observable_activities.join(", ")
            ));
        }

        perception.push_str(&format!("环境: {}\n", context.perception.environment_description));

        if !context.perception.recent_events.is_empty() {
            perception.push_str("最近发生的事:\n");
            for event in &context.perception.recent_events {
                perception.push_str(&format!("  - {event}\n"));
            }
        }

        perception
    }

    /// 思考阶段：组装完整的 LLM 决策请求
    ///
    /// 返回 CompletionRequest，不执行 LLM 调用。
    pub fn think(agent: &AgentState, context: &SituationContext) -> CompletionRequest {
        let personality_desc = personality_description(&agent.config.personality);
        let career_desc = CareerDatabase::aspiration_description(&agent.config.career_aspiration);
        let perception = Self::perceive(agent, context);

        let system_prompt = format!(
            r#"你是"{}"，一个正在上学的学生。

## 你的人格特征
{personality_desc}

## 你的职业志向
{career_desc}

## 你的当前情绪
{emotional_summary}

## 角色扮演规则
1. 你必须始终以"{name}"的身份思考和行动
2. 你的行为必须符合你的人格特征
3. 用第一人称回应，描述你想做什么以及为什么
4. 回应应该简洁（50-100字），只描述你下一步的行动意图
5. 考虑你的记忆和当前情境做出自然的决策"#,
            agent.config.name,
            emotional_summary = context.emotional_summary,
            name = agent.config.name,
        );

        let mut user_message = format!("## 当前情境\n{perception}\n");

        if !context.relevant_memories.is_empty() {
            user_message.push_str("\n## 相关记忆\n");
            for (i, mem) in context.relevant_memories.iter().enumerate() {
                user_message.push_str(&format!("{}. {mem}\n", i + 1));
            }
        }

        user_message.push_str("\n请描述你接下来想做什么？");

        CompletionRequest {
            system: system_prompt,
            messages: vec![ChatMessage {
                role: MessageRole::User,
                content: user_message,
            }],
            temperature: Some(0.8),
            max_tokens: Some(200),
        }
    }

    /// 行动阶段：解析 LLM 输出为 BehaviorIntent
    pub fn act(agent: &AgentState, llm_response: &str) -> BehaviorIntent {
        let intent_type = Self::classify_intent(llm_response);

        BehaviorIntent {
            agent_id: agent.id.clone(),
            description: llm_response.to_string(),
            target_location: None,
            target_agents: Vec::new(),
            intent_type,
        }
    }

    /// 简单的意图分类（基于关键词）
    fn classify_intent(response: &str) -> IntentType {
        let lower = response.to_lowercase();

        if lower.contains("图书馆") || lower.contains("学习") || lower.contains("作业") || lower.contains("看书") {
            IntentType::Study
        } else if lower.contains("聊天") || lower.contains("说话") || lower.contains("一起") || lower.contains("交流") {
            IntentType::Talk
        } else if lower.contains("去") || lower.contains("走") || lower.contains("前往") {
            IntentType::Move
        } else if lower.contains("休息") || lower.contains("放松") || lower.contains("发呆") {
            IntentType::Rest
        } else if lower.contains("合作") || lower.contains("一起做") || lower.contains("帮忙") {
            IntentType::Collaborate
        } else if lower.contains("不满") || lower.contains("争") || lower.contains("生气") {
            IntentType::Confront
        } else if lower.contains("想") || lower.contains("反思") || lower.contains("思考") {
            IntentType::Reflect
        } else if lower.contains("活动") || lower.contains("参加") || lower.contains("社团") {
            IntentType::JoinActivity
        } else {
            IntentType::Other
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_intent() {
        assert_eq!(
            CognitionProcessor::classify_intent("我想去图书馆学习"),
            IntentType::Study
        );
        assert_eq!(
            CognitionProcessor::classify_intent("我想休息一下放松放松"),
            IntentType::Rest
        );
        assert_eq!(
            CognitionProcessor::classify_intent("我想和小红聊天"),
            IntentType::Talk
        );
    }
}

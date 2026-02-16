//! 反思机制
//!
//! 累积经历评估 → 触发反思 → 生成语义记忆

use ai_school_core::traits::llm::{ChatMessage, CompletionRequest, MessageRole};
use ai_school_core::types::{AgentId, Memory, MemoryLayer, MemoryId, SimulationTime};

/// 反思触发器
pub struct ReflectionTrigger {
    /// 触发反思的累积事件阈值
    threshold: usize,
    /// 每个 Agent 的累积计数
    counters: std::collections::HashMap<AgentId, usize>,
}

impl ReflectionTrigger {
    pub fn new(threshold: usize) -> Self {
        Self {
            threshold,
            counters: std::collections::HashMap::new(),
        }
    }

    /// 记录一次事件，返回是否应该触发反思
    pub fn record_event(&mut self, agent_id: &AgentId) -> bool {
        let count = self.counters.entry(agent_id.clone()).or_insert(0);
        *count += 1;

        if *count >= self.threshold {
            *count = 0;
            true
        } else {
            false
        }
    }
}

/// 构建反思 Prompt
pub fn build_reflection_request(
    agent_name: &str,
    personality_desc: &str,
    recent_memories: &[Memory],
    current_time: &SimulationTime,
) -> CompletionRequest {
    let memories_text: String = recent_memories
        .iter()
        .enumerate()
        .map(|(i, m)| format!("{}. {}", i + 1, m.content))
        .collect::<Vec<_>>()
        .join("\n");

    let system = format!(
        r#"你是一个心理分析助手。请基于以下学生的近期经历进行反思性总结。

学生: {agent_name}
人格: {personality_desc}
当前时间: {}

请从以下角度进行反思：
1. 这些经历对学生有什么影响？
2. 学生可能从中学到了什么？
3. 这些经历是否可能影响学生的性格发展？如果是，哪个方面？

请用 JSON 格式输出：
{{
  "summary": "反思总结（2-3句话）",
  "insight": "核心洞察",
  "personality_impact": {{
    "dimension": "EI/SN/TF/JP 或 null",
    "direction": "positive/negative 或 null",
    "magnitude": 0.01-0.05 或 0
  }}
}}"#,
        current_time.display(),
    );

    CompletionRequest {
        system,
        messages: vec![ChatMessage {
            role: MessageRole::User,
            content: format!("以下是该学生最近的经历：\n\n{memories_text}"),
        }],
        temperature: Some(0.6),
        max_tokens: Some(300),
    }
}

/// 从反思结果创建语义记忆
pub fn create_semantic_memory(
    agent_id: &AgentId,
    reflection_summary: &str,
    current_time: &SimulationTime,
) -> Memory {
    Memory {
        id: MemoryId::new(),
        agent_id: agent_id.clone(),
        layer: MemoryLayer::Semantic,
        content: reflection_summary.to_string(),
        timestamp: current_time.clone(),
        importance: 0.8, // 反思记忆默认高重要性
        emotion_valence: 0.0,
        event_id: None,
        tags: vec!["reflection".to_string(), "semantic".to_string()],
        access_count: 0,
        last_accessed: current_time.clone(),
    }
}

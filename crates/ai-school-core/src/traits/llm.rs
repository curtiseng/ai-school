use async_trait::async_trait;
use serde::de::DeserializeOwned;

use crate::error::LlmError;

/// LLM 补全请求
#[derive(Debug, Clone)]
pub struct CompletionRequest {
    /// System prompt
    pub system: String,
    /// 消息历史
    pub messages: Vec<ChatMessage>,
    /// 温度参数 (0.0 ~ 2.0)
    pub temperature: Option<f32>,
    /// 最大输出 token 数
    pub max_tokens: Option<u32>,
}

/// 聊天消息
#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
}

/// 消息角色
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

/// LLM 补全响应
#[derive(Debug, Clone)]
pub struct CompletionResponse {
    pub content: String,
    pub usage: Option<TokenUsage>,
}

/// Token 使用量
#[derive(Debug, Clone)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// LLM 提供者 trait — ADR-0005 选型五
///
/// 三个方法精确覆盖 ADR-0003 的两个 LLM 调用点 + M4 的嵌入需求：
/// - `complete` → Agent 决策（调用点 #1）
/// - `complete_structured` → GM 仲裁（调用点 #2，输出 JSON Schema）
/// - `embed` → 记忆向量化（写入 Qdrant）
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// 自然语言补全
    async fn complete(&self, request: &CompletionRequest) -> Result<CompletionResponse, LlmError>;

    /// 结构化输出（JSON Schema 约束）
    async fn complete_structured<T: DeserializeOwned + Send>(
        &self,
        request: &CompletionRequest,
        schema: &serde_json::Value,
    ) -> Result<T, LlmError>;

    /// 文本嵌入（记忆向量化）
    async fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, LlmError>;
}

use async_openai::config::OpenAIConfig;
use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessage,
    ChatCompletionRequestUserMessage, CreateChatCompletionRequestArgs,
    CreateEmbeddingRequestArgs, EmbeddingInput,
};
use async_openai::Client;
use async_trait::async_trait;
use tracing::{debug, instrument};

use ai_school_core::config::LlmConfig;
use ai_school_core::error::LlmError;
use ai_school_core::traits::llm::{
    CompletionRequest, CompletionResponse, LlmProvider, MessageRole, TokenUsage,
};

/// DeepSeek LLM 提供者 — ADR-0005 选型五
///
/// 使用 DeepSeek 进行 chat 补全，智谱 AI 进行 embedding。
pub struct DeepSeekProvider {
    chat_client: Client<OpenAIConfig>,
    embedding_client: Client<OpenAIConfig>,
    chat_model: String,
    embedding_model: String,
    default_temperature: f32,
}

impl DeepSeekProvider {
    pub fn new(config: &LlmConfig) -> Self {
        let chat_config = OpenAIConfig::new()
            .with_api_key(&config.chat_api_key)
            .with_api_base(&config.chat_base_url);

        let embedding_config = OpenAIConfig::new()
            .with_api_key(&config.embedding_api_key)
            .with_api_base(&config.embedding_base_url);

        Self {
            chat_client: Client::with_config(chat_config),
            embedding_client: Client::with_config(embedding_config),
            chat_model: config.chat_model.clone(),
            embedding_model: config.embedding_model.clone(),
            default_temperature: config.default_temperature,
        }
    }

    fn build_messages(request: &CompletionRequest) -> Vec<ChatCompletionRequestMessage> {
        let mut messages = Vec::new();

        // System message
        if !request.system.is_empty() {
            messages.push(ChatCompletionRequestMessage::System(
                ChatCompletionRequestSystemMessage {
                    content: async_openai::types::ChatCompletionRequestSystemMessageContent::Text(
                        request.system.clone(),
                    ),
                    name: None,
                },
            ));
        }

        // Chat history
        for msg in &request.messages {
            let chat_msg = match msg.role {
                MessageRole::System => ChatCompletionRequestMessage::System(
                    ChatCompletionRequestSystemMessage {
                        content:
                            async_openai::types::ChatCompletionRequestSystemMessageContent::Text(
                                msg.content.clone(),
                            ),
                        name: None,
                    },
                ),
                MessageRole::User => ChatCompletionRequestMessage::User(
                    ChatCompletionRequestUserMessage {
                        content:
                            async_openai::types::ChatCompletionRequestUserMessageContent::Text(
                                msg.content.clone(),
                            ),
                        name: None,
                    },
                ),
                MessageRole::Assistant => {
                    ChatCompletionRequestMessage::Assistant(
                        async_openai::types::ChatCompletionRequestAssistantMessage {
                            content: Some(async_openai::types::ChatCompletionRequestAssistantMessageContent::Text(msg.content.clone())),
                            name: None,
                            tool_calls: None,
                            refusal: None,
                            audio: None,
                            function_call: None,
                        },
                    )
                }
            };
            messages.push(chat_msg);
        }

        messages
    }
}

#[async_trait]
impl LlmProvider for DeepSeekProvider {
    #[instrument(skip(self, request), fields(model = %self.chat_model))]
    async fn complete(
        &self,
        request: &CompletionRequest,
    ) -> Result<CompletionResponse, LlmError> {
        let temperature = request.temperature.unwrap_or(self.default_temperature);
        let messages = Self::build_messages(request);

        let mut req_builder = CreateChatCompletionRequestArgs::default();
        req_builder
            .model(&self.chat_model)
            .messages(messages)
            .temperature(temperature);

        if let Some(max_tokens) = request.max_tokens {
            req_builder.max_tokens(max_tokens);
        }

        let req = req_builder.build().map_err(|e| LlmError::ApiError(e.to_string()))?;

        let response = self
            .chat_client
            .chat()
            .create(req)
            .await
            .map_err(|e| LlmError::ApiError(e.to_string()))?;

        let choice = response
            .choices
            .first()
            .ok_or_else(|| LlmError::ApiError("No choices in response".to_string()))?;

        let content = choice
            .message
            .content
            .clone()
            .unwrap_or_default();

        let usage = response.usage.map(|u| TokenUsage {
            prompt_tokens: u.prompt_tokens,
            completion_tokens: u.completion_tokens,
            total_tokens: u.total_tokens,
        });

        debug!(tokens = ?usage, "LLM completion done");

        Ok(CompletionResponse { content, usage })
    }

    #[instrument(skip(self, request, schema), fields(model = %self.chat_model))]
    async fn complete_structured<T: serde::de::DeserializeOwned + Send>(
        &self,
        request: &CompletionRequest,
        schema: &serde_json::Value,
    ) -> Result<T, LlmError> {
        // Inject JSON Schema constraint into system prompt
        let schema_str = serde_json::to_string_pretty(schema)
            .map_err(|e| LlmError::SchemaValidation(e.to_string()))?;

        let mut structured_request = request.clone();
        structured_request.system = format!(
            "{}\n\n你必须以下面的 JSON Schema 格式输出，不要输出任何其他内容：\n```json\n{}\n```",
            request.system, schema_str
        );

        let response = self.complete(&structured_request).await?;

        // Extract JSON from response
        let json_str = crate::structured::extract_json(&response.content)?;

        // Validate against schema
        crate::structured::validate_json(&json_str, schema)?;

        // Deserialize
        serde_json::from_str(&json_str).map_err(|e| LlmError::ParseError(e.to_string()))
    }

    #[instrument(skip(self, texts), fields(model = %self.embedding_model, count = texts.len()))]
    async fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, LlmError> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        let input = EmbeddingInput::StringArray(texts.to_vec());

        let req = CreateEmbeddingRequestArgs::default()
            .model(&self.embedding_model)
            .input(input)
            .build()
            .map_err(|e| LlmError::EmbeddingError(e.to_string()))?;

        let response = self
            .embedding_client
            .embeddings()
            .create(req)
            .await
            .map_err(|e| LlmError::EmbeddingError(e.to_string()))?;

        let embeddings = response
            .data
            .into_iter()
            .map(|d| d.embedding)
            .collect();

        debug!(count = texts.len(), "Embeddings generated");

        Ok(embeddings)
    }
}

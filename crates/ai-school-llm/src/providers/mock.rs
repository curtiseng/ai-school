use async_trait::async_trait;
use std::sync::atomic::{AtomicU32, Ordering};

use ai_school_core::error::LlmError;
use ai_school_core::traits::llm::{
    CompletionRequest, CompletionResponse, LlmProvider, TokenUsage,
};

/// Mock LLM 提供者 — 用于测试和开发
///
/// 返回预设的响应，不需要真实 LLM API。
pub struct MockLlmProvider {
    call_count: AtomicU32,
    embedding_dim: usize,
}

impl MockLlmProvider {
    pub fn new(embedding_dim: usize) -> Self {
        Self {
            call_count: AtomicU32::new(0),
            embedding_dim,
        }
    }

    pub fn call_count(&self) -> u32 {
        self.call_count.load(Ordering::Relaxed)
    }
}

impl Default for MockLlmProvider {
    fn default() -> Self {
        Self::new(1536)
    }
}

#[async_trait]
impl LlmProvider for MockLlmProvider {
    async fn complete(
        &self,
        _request: &CompletionRequest,
    ) -> Result<CompletionResponse, LlmError> {
        self.call_count.fetch_add(1, Ordering::Relaxed);

        Ok(CompletionResponse {
            content: r#"我想去图书馆看看今天的数学作业，顺便看看有没有关于编程的书。"#.to_string(),
            usage: Some(TokenUsage {
                prompt_tokens: 100,
                completion_tokens: 50,
                total_tokens: 150,
            }),
        })
    }

    async fn complete_structured<T: serde::de::DeserializeOwned + Send>(
        &self,
        _request: &CompletionRequest,
        _schema: &serde_json::Value,
    ) -> Result<T, LlmError> {
        self.call_count.fetch_add(1, Ordering::Relaxed);

        // Return a default JSON that the caller can parse
        let default_json = serde_json::json!({
            "event_type": "Routine",
            "intensity": 0.3,
            "state_changes": [],
            "narrative": "Mock GM 仲裁结果：日常活动正常进行。"
        });

        serde_json::from_value(default_json).map_err(|e| LlmError::ParseError(e.to_string()))
    }

    async fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, LlmError> {
        self.call_count.fetch_add(1, Ordering::Relaxed);

        // Generate deterministic mock embeddings based on text content
        let embeddings = texts
            .iter()
            .map(|text| {
                let seed = text.bytes().fold(0u32, |acc, b| acc.wrapping_add(b as u32));
                let mut rng_state = seed;
                (0..self.embedding_dim)
                    .map(|_| {
                        // Simple LCG for deterministic pseudo-random
                        rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
                        ((rng_state >> 16) as f32 / 32768.0) - 1.0
                    })
                    .collect::<Vec<f32>>()
            })
            .collect();

        Ok(embeddings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ai_school_core::traits::llm::ChatMessage;
    use ai_school_core::traits::llm::MessageRole;

    #[tokio::test]
    async fn test_mock_complete() {
        let provider = MockLlmProvider::default();
        let request = CompletionRequest {
            system: "你是一个学生".to_string(),
            messages: vec![ChatMessage {
                role: MessageRole::User,
                content: "你现在想做什么？".to_string(),
            }],
            temperature: None,
            max_tokens: None,
        };

        let response = provider.complete(&request).await.unwrap();
        assert!(!response.content.is_empty());
        assert_eq!(provider.call_count(), 1);
    }

    #[tokio::test]
    async fn test_mock_embed() {
        let provider = MockLlmProvider::new(128);
        let texts = vec!["hello".to_string(), "world".to_string()];

        let embeddings = provider.embed(&texts).await.unwrap();
        assert_eq!(embeddings.len(), 2);
        assert_eq!(embeddings[0].len(), 128);

        // Deterministic: same input produces same output
        let embeddings2 = provider.embed(&texts).await.unwrap();
        assert_eq!(embeddings[0], embeddings2[0]);
    }
}

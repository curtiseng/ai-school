use std::time::Duration;
use tokio::time::sleep;
use tracing::warn;

use ai_school_core::error::LlmError;

/// 带退避的重试策略
pub async fn with_retry<F, Fut, T>(max_retries: u32, mut f: F) -> Result<T, LlmError>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, LlmError>>,
{
    let mut last_error = None;

    for attempt in 0..=max_retries {
        match f().await {
            Ok(result) => return Ok(result),
            Err(LlmError::RateLimited { retry_after_ms }) => {
                warn!(attempt, retry_after_ms, "Rate limited, backing off");
                sleep(Duration::from_millis(retry_after_ms)).await;
                last_error = Some(LlmError::RateLimited { retry_after_ms });
            }
            Err(LlmError::Timeout) if attempt < max_retries => {
                let backoff = Duration::from_millis(1000 * 2u64.pow(attempt));
                warn!(attempt, backoff_ms = backoff.as_millis(), "Timeout, retrying");
                sleep(backoff).await;
                last_error = Some(LlmError::Timeout);
            }
            Err(e) => return Err(e),
        }
    }

    Err(last_error.unwrap_or(LlmError::ApiError("Max retries exceeded".to_string())))
}

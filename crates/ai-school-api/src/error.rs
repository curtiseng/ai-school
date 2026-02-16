use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

/// API 错误包装器（解决 orphan rule）
pub struct AppError(pub ai_school_core::error::ApiError);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        use ai_school_core::error::ApiError;

        let (status, message) = match &self.0 {
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            ApiError::Simulation(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };

        let body = json!({
            "error": message,
            "status": status.as_u16(),
        });

        (status, axum::Json(body)).into_response()
    }
}

use thiserror::Error;

use crate::types::{AgentId, LocationId, MemoryId};

/// Agent 系统错误
#[derive(Debug, Error)]
pub enum AgentError {
    #[error("Agent not found: {0}")]
    NotFound(AgentId),

    #[error("Invalid personality params: {0}")]
    InvalidPersonality(String),

    #[error("Agent configuration error: {0}")]
    ConfigError(String),

    #[error("Cognition error: {0}")]
    CognitionError(String),
}

/// LLM 集成错误
#[derive(Debug, Error)]
pub enum LlmError {
    #[error("LLM API error: {0}")]
    ApiError(String),

    #[error("LLM rate limited, retry after {retry_after_ms}ms")]
    RateLimited { retry_after_ms: u64 },

    #[error("Structured output parsing error: {0}")]
    ParseError(String),

    #[error("Schema validation error: {0}")]
    SchemaValidation(String),

    #[error("Prompt rendering error: {0}")]
    PromptError(String),

    #[error("Embedding error: {0}")]
    EmbeddingError(String),

    #[error("Request timeout")]
    Timeout,
}

/// 记忆系统错误
#[derive(Debug, Error)]
pub enum MemoryError {
    #[error("Memory not found: {0:?}")]
    NotFound(MemoryId),

    #[error("Memory store error: {0}")]
    StoreError(String),

    #[error("Memory retrieval error: {0}")]
    RetrievalError(String),

    #[error("Consolidation error: {0}")]
    ConsolidationError(String),
}

/// 世界系统错误
#[derive(Debug, Error)]
pub enum WorldError {
    #[error("Location not found: {0}")]
    LocationNotFound(LocationId),

    #[error("Agent not found in world: {0}")]
    AgentNotFound(AgentId),

    #[error("Invalid state change: {0}")]
    InvalidStateChange(String),

    #[error("Capacity exceeded at location: {0}")]
    CapacityExceeded(LocationId),

    #[error("World state error: {0}")]
    StateError(String),
}

/// 仿真引擎错误
#[derive(Debug, Error)]
pub enum SimulationError {
    #[error("Simulation not running")]
    NotRunning,

    #[error("Simulation already running")]
    AlreadyRunning,

    #[error("Agent error: {0}")]
    Agent(#[from] AgentError),

    #[error("LLM error: {0}")]
    Llm(#[from] LlmError),

    #[error("Memory error: {0}")]
    Memory(#[from] MemoryError),

    #[error("World error: {0}")]
    World(#[from] WorldError),

    #[error("Game Master error: {0}")]
    GameMaster(String),

    #[error("Consistency violation: {0}")]
    ConsistencyViolation(String),

    #[error("Intervention error: {0}")]
    Intervention(String),
}

/// API 层错误
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Simulation error: {0}")]
    Simulation(#[from] SimulationError),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

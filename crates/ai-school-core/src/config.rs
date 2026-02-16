use serde::{Deserialize, Serialize};

/// 仿真配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationConfig {
    /// 最大 Agent 数量
    pub max_agents: usize,
    /// 仿真步时间间隔（仿真时间中的小时数）
    pub time_step_hours: u32,
    /// 是否启用自动事件生成
    pub auto_events_enabled: bool,
    /// 随机事件频率 (0.0 ~ 1.0)
    pub random_event_frequency: f32,
    /// 记忆反思触发阈值（累积事件数）
    pub reflection_threshold: usize,
    /// 人格演化衰减系数
    pub personality_decay_factor: f32,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            max_agents: 10,
            time_step_hours: 1,
            auto_events_enabled: true,
            random_event_frequency: 0.1,
            reflection_threshold: 10,
            personality_decay_factor: 0.8,
        }
    }
}

/// LLM 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    /// Chat 补全 API base URL
    pub chat_base_url: String,
    /// Chat API key
    pub chat_api_key: String,
    /// Chat 模型名称
    pub chat_model: String,
    /// Embedding API base URL
    pub embedding_base_url: String,
    /// Embedding API key
    pub embedding_api_key: String,
    /// Embedding 模型名称
    pub embedding_model: String,
    /// 默认温度
    pub default_temperature: f32,
    /// 最大重试次数
    pub max_retries: u32,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            chat_base_url: "https://api.deepseek.com".to_string(),
            chat_api_key: String::new(),
            chat_model: "deepseek-chat".to_string(),
            embedding_base_url: "https://api.openai.com/v1".to_string(),
            embedding_api_key: String::new(),
            embedding_model: "text-embedding-3-small".to_string(),
            default_temperature: 0.7,
            max_retries: 3,
        }
    }
}

/// Qdrant 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QdrantConfig {
    pub url: String,
    /// 向量维度（取决于 embedding 模型）
    pub vector_size: u64,
}

impl Default for QdrantConfig {
    fn default() -> Self {
        Self {
            url: "http://localhost:6333".to_string(),
            vector_size: 1536, // OpenAI text-embedding-3-small
        }
    }
}

/// 数据库配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgres://ai_school:dev_password@localhost:5432/ai_school".to_string(),
        }
    }
}

/// 服务端配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 3000,
        }
    }
}

/// 应用全局配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub simulation: SimulationConfig,
    pub llm: LlmConfig,
    pub qdrant: QdrantConfig,
    pub database: DatabaseConfig,
    pub server: ServerConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            simulation: SimulationConfig::default(),
            llm: LlmConfig::default(),
            qdrant: QdrantConfig::default(),
            database: DatabaseConfig::default(),
            server: ServerConfig::default(),
        }
    }
}

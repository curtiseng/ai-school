use serde::{Deserialize, Serialize};

use ai_school_core::types::{PresetEvent, SimulationSpeed};

/// 创建 Agent 请求
#[derive(Debug, Deserialize)]
pub struct CreateAgentRequest {
    pub name: String,
    /// MBTI E/I 维度 (-1.0 ~ 1.0)
    pub e_i: f32,
    /// MBTI S/N 维度
    pub s_n: f32,
    /// MBTI T/F 维度
    pub t_f: f32,
    /// MBTI J/P 维度
    pub j_p: f32,
    /// 理想职业
    pub ideal_career: Option<String>,
    /// 背景描述
    pub background: Option<String>,
    /// 年龄
    pub age: Option<u8>,
}

/// 批量生成 Agent 请求
#[derive(Debug, Deserialize)]
pub struct GenerateAgentsRequest {
    pub count: usize,
}

/// 设置仿真速度请求
#[derive(Debug, Deserialize)]
pub struct SetSpeedRequest {
    pub speed: SimulationSpeed,
}

/// 触发事件请求
#[derive(Debug, Deserialize)]
pub struct TriggerEventRequest {
    pub event: PresetEvent,
}

/// 参数调整请求
#[derive(Debug, Deserialize)]
pub struct AdjustParamsRequest {
    pub parameter: String,
    pub value: f32,
}

/// 对话请求
#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub role: String,
    pub message: String,
}

/// 仿真状态响应
#[derive(Debug, Serialize)]
pub struct SimulationStatusResponse {
    pub running: bool,
    pub tick: u64,
    pub time_display: String,
    pub agent_count: usize,
    pub speed: SimulationSpeed,
}

/// 通用成功响应
#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub success: bool,
    pub message: String,
}

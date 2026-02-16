//! 状态广播 — 供 WebSocket 消费

use serde::Serialize;

use ai_school_core::types::{SimulationEvent, SimulationSpeed, SimulationTime, WorldSnapshot};

/// 仿真更新事件
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum SimulationUpdate {
    /// 仿真步进
    Tick {
        time: SimulationTime,
        snapshot: WorldSnapshot,
        events: Vec<SimulationEvent>,
    },
    /// 速度变更
    SpeedChanged { speed: SimulationSpeed },
    /// 仿真开始
    Started,
    /// 仿真停止
    Stopped,
}

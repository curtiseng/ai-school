use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::agent::{AgentId, AgentState};

/// 位置唯一标识符
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct LocationId(pub String);

impl std::fmt::Display for LocationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 校园区域类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum LocationType {
    /// 教室
    Classroom { subject: Option<String> },
    /// 图书馆
    Library,
    /// 操场
    Playground,
    /// 食堂
    Cafeteria,
    /// 宿舍
    Dormitory,
    /// 社团活动室
    ClubRoom { club_name: Option<String> },
    /// 礼堂
    Auditorium,
    /// 自习室
    StudyRoom,
    /// 休息区
    RestArea,
}

/// 校园位置定义
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Location {
    pub id: LocationId,
    pub name: String,
    pub location_type: LocationType,
    /// 最大容纳人数
    pub capacity: usize,
    /// 地图坐标 (x, y)
    pub position: (f32, f32),
    /// 相邻位置
    pub adjacent: Vec<LocationId>,
}

/// 仿真时间
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
pub struct SimulationTime {
    /// 学期编号 (从 1 开始)
    pub semester: u32,
    /// 第几周 (1-20)
    pub week: u32,
    /// 星期几 (1=Monday, 7=Sunday)
    pub day_of_week: u32,
    /// 时段 (0-23)
    pub hour: u32,
    /// 总步数 (全局递增)
    pub tick: u64,
}

impl SimulationTime {
    pub fn new() -> Self {
        Self {
            semester: 1,
            week: 1,
            day_of_week: 1,
            hour: 8,
            tick: 0,
        }
    }

    /// 格式化显示
    pub fn display(&self) -> String {
        let day_name = match self.day_of_week {
            1 => "周一",
            2 => "周二",
            3 => "周三",
            4 => "周四",
            5 => "周五",
            6 => "周六",
            7 => "周日",
            _ => "未知",
        };
        format!(
            "第{}学期 第{}周 {} {:02}:00",
            self.semester, self.week, day_name, self.hour
        )
    }

    /// 获取总天数（用于计算时间差）
    pub fn total_hours(&self) -> u64 {
        let semester_hours = (self.semester as u64 - 1) * 20 * 7 * 24;
        let week_hours = (self.week as u64 - 1) * 7 * 24;
        let day_hours = (self.day_of_week as u64 - 1) * 24;
        semester_hours + week_hours + day_hours + self.hour as u64
    }
}

impl Default for SimulationTime {
    fn default() -> Self {
        Self::new()
    }
}

/// Agent 间关系
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Relationship {
    pub agent_a: AgentId,
    pub agent_b: AgentId,
    /// 亲密度 (-1.0 敌对 ~ +1.0 亲密)
    pub closeness: f32,
    /// 信任度 (0.0 ~ 1.0)
    pub trust: f32,
    /// 关系类型标签
    pub tags: Vec<String>,
    /// 最近互动时间
    pub last_interaction: Option<SimulationTime>,
}

/// 世界状态快照 — 用于可视化和回放
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSnapshot {
    pub time: SimulationTime,
    pub agents: Vec<AgentState>,
    pub relationships: Vec<Relationship>,
    pub active_events: Vec<String>,
}

/// 仿真速度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum SimulationSpeed {
    Paused,
    Normal,     // 1x
    Fast,       // 2x
    VeryFast,   // 5x
    Maximum,    // 10x
    Unlimited,  // 最大速度
}

impl SimulationSpeed {
    /// 每个仿真步的现实等待时间（毫秒）
    pub fn step_interval_ms(&self) -> Option<u64> {
        match self {
            SimulationSpeed::Paused => None,
            SimulationSpeed::Normal => Some(2000),
            SimulationSpeed::Fast => Some(1000),
            SimulationSpeed::VeryFast => Some(400),
            SimulationSpeed::Maximum => Some(200),
            SimulationSpeed::Unlimited => Some(0),
        }
    }
}

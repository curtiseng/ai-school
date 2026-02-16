use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::event::EventId;
use super::world::SimulationTime;

/// MBTI 人格维度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum PersonalityDimension {
    /// 外倾 (E) ←→ 内倾 (I)
    EI,
    /// 感知 (S) ←→ 直觉 (N)
    SN,
    /// 思考 (T) ←→ 情感 (F)
    TF,
    /// 判断 (J) ←→ 知觉 (P)
    JP,
}

/// MBTI 4 维连续人格参数 — 对应 ADR-0002 的人格参数空间 Θ
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PersonalityParams {
    /// 外倾 (-1.0) ←→ 内倾 (+1.0)
    pub e_i: f32,
    /// 感知 (-1.0) ←→ 直觉 (+1.0)
    pub s_n: f32,
    /// 思考 (-1.0) ←→ 情感 (+1.0)
    pub t_f: f32,
    /// 判断 (-1.0) ←→ 知觉 (+1.0)
    pub j_p: f32,
    /// 人格稳定度（随年龄/经历增长，越高越难改变）
    pub stability: f32,
    /// 人格变化历史
    pub shift_history: Vec<PersonalityShift>,
}

impl PersonalityParams {
    /// 创建新的人格参数，稳定度默认为 1.0
    pub fn new(e_i: f32, s_n: f32, t_f: f32, j_p: f32) -> Self {
        Self {
            e_i: e_i.clamp(-1.0, 1.0),
            s_n: s_n.clamp(-1.0, 1.0),
            t_f: t_f.clamp(-1.0, 1.0),
            j_p: j_p.clamp(-1.0, 1.0),
            stability: 1.0,
            shift_history: Vec::new(),
        }
    }

    /// 获取指定维度的值
    pub fn get_dimension(&self, dim: PersonalityDimension) -> f32 {
        match dim {
            PersonalityDimension::EI => self.e_i,
            PersonalityDimension::SN => self.s_n,
            PersonalityDimension::TF => self.t_f,
            PersonalityDimension::JP => self.j_p,
        }
    }

    /// 设置指定维度的值
    pub fn set_dimension(&mut self, dim: PersonalityDimension, value: f32) {
        let clamped = value.clamp(-1.0, 1.0);
        match dim {
            PersonalityDimension::EI => self.e_i = clamped,
            PersonalityDimension::SN => self.s_n = clamped,
            PersonalityDimension::TF => self.t_f = clamped,
            PersonalityDimension::JP => self.j_p = clamped,
        }
    }

    /// 获取 MBTI 类型标签（如 "INTJ", "ENFP"）
    pub fn mbti_label(&self) -> String {
        let e_i = if self.e_i < 0.0 { 'E' } else { 'I' };
        let s_n = if self.s_n < 0.0 { 'S' } else { 'N' };
        let t_f = if self.t_f < 0.0 { 'T' } else { 'F' };
        let j_p = if self.j_p < 0.0 { 'J' } else { 'P' };
        format!("{e_i}{s_n}{t_f}{j_p}")
    }

    /// 应用人格微调 — ADR-0002 演化速率控制
    /// ΔΘ_actual = ΔΘ_signal × (1/stability) × decay_factor
    pub fn apply_shift(
        &mut self,
        dimension: PersonalityDimension,
        delta_signal: f32,
        decay_factor: f32,
        timestamp: SimulationTime,
        trigger_event_id: EventId,
        reason: String,
    ) {
        let actual_delta = delta_signal * (1.0 / self.stability) * decay_factor;

        let old_value = self.get_dimension(dimension);
        self.set_dimension(dimension, old_value + actual_delta);

        self.shift_history.push(PersonalityShift {
            timestamp,
            trigger_event_id,
            dimension,
            delta: actual_delta,
            reason,
        });

        // 每次经历后稍微增加稳定度
        self.stability = (self.stability + 0.01).min(10.0);
    }
}

/// 人格微调记录 — 支持 M5 研究分析的轨迹回溯
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PersonalityShift {
    pub timestamp: SimulationTime,
    pub trigger_event_id: EventId,
    pub dimension: PersonalityDimension,
    pub delta: f32,
    pub reason: String,
}

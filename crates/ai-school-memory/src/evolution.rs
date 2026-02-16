//! M4.3 人格动态演变 — ADR-0002
//!
//! 反思结论 → 人格微调信号 → 更新人格参数

use ai_school_core::types::{EventId, PersonalityDimension, PersonalityParams, SimulationTime};

/// 人格演化控制器
pub struct PersonalityEvolution {
    /// 衰减系数（防止频繁微调）
    pub decay_factor: f32,
    /// 最小变化阈值（低于此值不触发演化）
    pub min_delta: f32,
}

impl PersonalityEvolution {
    pub fn new(decay_factor: f32) -> Self {
        Self {
            decay_factor,
            min_delta: 0.005,
        }
    }

    /// 评估反思结论是否应该触发人格微调
    ///
    /// 返回 (维度, 信号强度) 如果应该微调
    pub fn evaluate_reflection(
        &self,
        reflection_impact: &ReflectionImpact,
    ) -> Option<(PersonalityDimension, f32)> {
        if reflection_impact.magnitude.abs() < self.min_delta {
            return None;
        }

        let dimension = match reflection_impact.dimension.as_str() {
            "EI" => PersonalityDimension::EI,
            "SN" => PersonalityDimension::SN,
            "TF" => PersonalityDimension::TF,
            "JP" => PersonalityDimension::JP,
            _ => return None,
        };

        let signal = if reflection_impact.direction == "positive" {
            reflection_impact.magnitude
        } else {
            -reflection_impact.magnitude
        };

        Some((dimension, signal))
    }

    /// 应用人格微调
    /// ΔΘ_actual = ΔΘ_signal × (1/stability) × decay_factor
    pub fn apply_evolution(
        &self,
        personality: &mut PersonalityParams,
        dimension: PersonalityDimension,
        delta_signal: f32,
        timestamp: SimulationTime,
        trigger_event_id: EventId,
        reason: String,
    ) {
        personality.apply_shift(
            dimension,
            delta_signal,
            self.decay_factor,
            timestamp,
            trigger_event_id,
            reason,
        );
    }
}

impl Default for PersonalityEvolution {
    fn default() -> Self {
        Self::new(0.8)
    }
}

/// 反思影响评估结果
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ReflectionImpact {
    pub dimension: String,
    pub direction: String,
    pub magnitude: f32,
}

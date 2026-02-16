//! ADR-0004 一致性检测
//!
//! 叙事 vs 数据变更的一致性校验。

use ai_school_core::types::SimulationEvent;

/// 一致性警告
#[derive(Debug, Clone)]
pub struct ConsistencyWarning {
    pub event_id: String,
    pub warning_type: WarningType,
    pub description: String,
}

/// 警告类型
#[derive(Debug, Clone)]
pub enum WarningType {
    /// 叙事描述与数据变更不匹配
    NarrativeDataMismatch,
    /// 情感强度不匹配
    EmotionIntensityMismatch,
    /// 因果关系异常
    CausalAnomaly,
}

/// 检查事件一致性
pub fn check_consistency(event: &SimulationEvent) -> Vec<ConsistencyWarning> {
    let mut warnings = Vec::new();

    // 检查叙事与状态变更是否匹配
    if event.narrative.is_empty() && !event.state_changes.is_empty() {
        warnings.push(ConsistencyWarning {
            event_id: event.id.to_string(),
            warning_type: WarningType::NarrativeDataMismatch,
            description: "有状态变更但无叙事描述".to_string(),
        });
    }

    // 检查强度与情感变更是否匹配
    if event.intensity > 0.7 && event.state_changes.is_empty() {
        warnings.push(ConsistencyWarning {
            event_id: event.id.to_string(),
            warning_type: WarningType::EmotionIntensityMismatch,
            description: "高强度事件但无状态变更".to_string(),
        });
    }

    warnings
}

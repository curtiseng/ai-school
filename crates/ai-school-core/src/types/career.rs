use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// 职业类别
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum CareerCategory {
    Science,
    Technology,
    Engineering,
    Arts,
    Medicine,
    Education,
    Business,
    Law,
    SocialWork,
    Other(String),
}

/// 职业志向
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CareerAspiration {
    /// 理想职业名称
    pub ideal_career: String,
    /// 职业类别
    pub category: CareerCategory,
    /// 相关学科偏好（学科名 → 偏好强度 0.0~1.0）
    pub subject_preferences: Vec<SubjectPreference>,
    /// 职业清晰度 (0.0 迷茫 ~ 1.0 确定)
    pub clarity: f32,
}

/// 学科偏好
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SubjectPreference {
    pub subject: String,
    pub preference: f32,
}

/// 职业匹配度评估
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CareerMatch {
    pub career: String,
    pub category: CareerCategory,
    /// 匹配度 (0.0 ~ 1.0)
    pub score: f32,
    /// 匹配理由
    pub reasons: Vec<String>,
}

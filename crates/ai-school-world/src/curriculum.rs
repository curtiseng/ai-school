//! M2.1 课程与学习活动
//!
//! 课程表定义、学科难度模型、学业反馈计算。

use serde::{Deserialize, Serialize};

use ai_school_core::types::LocationId;

/// 学科定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
    pub name: String,
    /// 基础难度 (0.0 ~ 1.0)
    pub base_difficulty: f32,
    /// 当前难度调整（可通过干预修改）
    pub difficulty_modifier: f32,
    /// 对应教室
    pub classroom: LocationId,
}

/// 课程时间段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassSchedule {
    /// 星期几 (1=周一, 5=周五)
    pub day_of_week: u32,
    /// 第几节课 (1-5)
    pub period: u32,
    /// 学科
    pub subject: String,
}

/// 创建默认课程表
pub fn create_default_schedule() -> Vec<ClassSchedule> {
    vec![
        // 周一
        ClassSchedule { day_of_week: 1, period: 1, subject: "数学".to_string() },
        ClassSchedule { day_of_week: 1, period: 2, subject: "语文".to_string() },
        ClassSchedule { day_of_week: 1, period: 3, subject: "英语".to_string() },
        ClassSchedule { day_of_week: 1, period: 4, subject: "物理".to_string() },
        ClassSchedule { day_of_week: 1, period: 5, subject: "体育".to_string() },
        // 周二
        ClassSchedule { day_of_week: 2, period: 1, subject: "英语".to_string() },
        ClassSchedule { day_of_week: 2, period: 2, subject: "数学".to_string() },
        ClassSchedule { day_of_week: 2, period: 3, subject: "化学".to_string() },
        ClassSchedule { day_of_week: 2, period: 4, subject: "语文".to_string() },
        ClassSchedule { day_of_week: 2, period: 5, subject: "美术".to_string() },
        // 周三
        ClassSchedule { day_of_week: 3, period: 1, subject: "物理".to_string() },
        ClassSchedule { day_of_week: 3, period: 2, subject: "数学".to_string() },
        ClassSchedule { day_of_week: 3, period: 3, subject: "语文".to_string() },
        ClassSchedule { day_of_week: 3, period: 4, subject: "生物".to_string() },
        ClassSchedule { day_of_week: 3, period: 5, subject: "信息技术".to_string() },
        // 周四
        ClassSchedule { day_of_week: 4, period: 1, subject: "语文".to_string() },
        ClassSchedule { day_of_week: 4, period: 2, subject: "英语".to_string() },
        ClassSchedule { day_of_week: 4, period: 3, subject: "数学".to_string() },
        ClassSchedule { day_of_week: 4, period: 4, subject: "历史".to_string() },
        ClassSchedule { day_of_week: 4, period: 5, subject: "音乐".to_string() },
        // 周五
        ClassSchedule { day_of_week: 5, period: 1, subject: "数学".to_string() },
        ClassSchedule { day_of_week: 5, period: 2, subject: "化学".to_string() },
        ClassSchedule { day_of_week: 5, period: 3, subject: "英语".to_string() },
        ClassSchedule { day_of_week: 5, period: 4, subject: "地理".to_string() },
        ClassSchedule { day_of_week: 5, period: 5, subject: "社团活动".to_string() },
    ]
}

/// 创建默认学科列表
pub fn create_default_subjects() -> Vec<Subject> {
    vec![
        Subject { name: "数学".to_string(), base_difficulty: 0.7, difficulty_modifier: 0.0, classroom: LocationId("classroom_math".to_string()) },
        Subject { name: "语文".to_string(), base_difficulty: 0.5, difficulty_modifier: 0.0, classroom: LocationId("classroom_chinese".to_string()) },
        Subject { name: "英语".to_string(), base_difficulty: 0.6, difficulty_modifier: 0.0, classroom: LocationId("classroom_english".to_string()) },
        Subject { name: "物理".to_string(), base_difficulty: 0.8, difficulty_modifier: 0.0, classroom: LocationId("classroom_science".to_string()) },
        Subject { name: "化学".to_string(), base_difficulty: 0.7, difficulty_modifier: 0.0, classroom: LocationId("classroom_science".to_string()) },
        Subject { name: "生物".to_string(), base_difficulty: 0.6, difficulty_modifier: 0.0, classroom: LocationId("classroom_science".to_string()) },
        Subject { name: "历史".to_string(), base_difficulty: 0.4, difficulty_modifier: 0.0, classroom: LocationId("classroom_chinese".to_string()) },
        Subject { name: "地理".to_string(), base_difficulty: 0.5, difficulty_modifier: 0.0, classroom: LocationId("classroom_chinese".to_string()) },
        Subject { name: "信息技术".to_string(), base_difficulty: 0.5, difficulty_modifier: 0.0, classroom: LocationId("classroom_science".to_string()) },
        Subject { name: "美术".to_string(), base_difficulty: 0.3, difficulty_modifier: 0.0, classroom: LocationId("club_room".to_string()) },
        Subject { name: "音乐".to_string(), base_difficulty: 0.3, difficulty_modifier: 0.0, classroom: LocationId("auditorium".to_string()) },
        Subject { name: "体育".to_string(), base_difficulty: 0.3, difficulty_modifier: 0.0, classroom: LocationId("playground".to_string()) },
    ]
}

/// 计算学业反馈：Agent 在某学科的表现
pub fn calculate_academic_feedback(
    subject_difficulty: f32,
    agent_academic_ability: f32,
    agent_subject_preference: f32,
) -> AcademicFeedback {
    let effective_ability = agent_academic_ability * (0.7 + 0.3 * agent_subject_preference);
    let performance = (effective_ability - subject_difficulty + 1.0) / 2.0;
    let performance = performance.clamp(0.0, 1.0);

    let satisfaction = if performance > 0.7 {
        0.3 + (performance - 0.7) * 2.0
    } else if performance > 0.4 {
        0.0
    } else {
        -0.3 - (0.4 - performance) * 2.0
    };

    AcademicFeedback {
        performance: performance.clamp(0.0, 1.0),
        satisfaction: satisfaction.clamp(-1.0, 1.0),
    }
}

/// 学业反馈
#[derive(Debug, Clone)]
pub struct AcademicFeedback {
    /// 表现 (0.0 ~ 1.0)
    pub performance: f32,
    /// 满意度 (-1.0 ~ 1.0)
    pub satisfaction: f32,
}

//! M1.2 职业志向模型
//!
//! 职业与学科偏好的映射，职业志向对行为的影响。

use ai_school_core::types::{
    CareerAspiration, CareerCategory, CareerMatch, PersonalityParams, SubjectPreference,
};

/// 预定义职业列表及其与 MBTI 的关联
pub struct CareerDatabase;

impl CareerDatabase {
    /// 根据人格参数推荐职业匹配
    pub fn suggest_careers(personality: &PersonalityParams) -> Vec<CareerMatch> {
        let mut matches = Vec::new();

        // 基于 MBTI 维度的职业匹配规则
        let careers = Self::career_personality_mappings();

        for (career, category, ideal_e_i, ideal_s_n, ideal_t_f, ideal_j_p) in &careers {
            let score = Self::calculate_match_score(
                personality,
                *ideal_e_i,
                *ideal_s_n,
                *ideal_t_f,
                *ideal_j_p,
            );

            if score > 0.4 {
                matches.push(CareerMatch {
                    career: career.to_string(),
                    category: category.clone(),
                    score,
                    reasons: Self::generate_reasons(personality, career),
                });
            }
        }

        matches.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        matches.truncate(5);
        matches
    }

    /// 生成职业志向的描述文本（用于 Prompt）
    pub fn aspiration_description(aspiration: &CareerAspiration) -> String {
        let clarity_desc = if aspiration.clarity > 0.7 {
            "非常明确"
        } else if aspiration.clarity > 0.4 {
            "有初步想法"
        } else {
            "还在探索"
        };

        let subjects: Vec<String> = aspiration
            .subject_preferences
            .iter()
            .filter(|s| s.preference > 0.5)
            .map(|s| format!("{}({:.0}%)", s.subject, s.preference * 100.0))
            .collect();

        format!(
            "理想职业: {} (清晰度: {})\n偏好学科: {}",
            aspiration.ideal_career,
            clarity_desc,
            if subjects.is_empty() {
                "暂无明确偏好".to_string()
            } else {
                subjects.join(", ")
            }
        )
    }

    fn career_personality_mappings() -> Vec<(&'static str, CareerCategory, f32, f32, f32, f32)> {
        vec![
            // (职业, 类别, E/I, S/N, T/F, J/P)
            ("软件工程师", CareerCategory::Technology, 0.3, 0.5, -0.5, -0.3),
            ("数据科学家", CareerCategory::Science, 0.5, 0.7, -0.7, -0.2),
            ("产品经理", CareerCategory::Business, -0.5, 0.3, -0.2, -0.5),
            ("心理咨询师", CareerCategory::SocialWork, -0.3, 0.4, 0.8, 0.2),
            ("教师", CareerCategory::Education, -0.4, -0.2, 0.5, -0.3),
            ("医生", CareerCategory::Medicine, 0.0, -0.3, -0.3, -0.7),
            ("艺术家", CareerCategory::Arts, 0.3, 0.8, 0.5, 0.7),
            ("律师", CareerCategory::Law, -0.3, -0.2, -0.7, -0.8),
            ("科研人员", CareerCategory::Science, 0.7, 0.8, -0.5, -0.3),
            ("企业家", CareerCategory::Business, -0.7, 0.5, -0.3, 0.3),
        ]
    }

    fn calculate_match_score(
        personality: &PersonalityParams,
        ideal_e_i: f32,
        ideal_s_n: f32,
        ideal_t_f: f32,
        ideal_j_p: f32,
    ) -> f32 {
        let diff_e_i = (personality.e_i - ideal_e_i).abs();
        let diff_s_n = (personality.s_n - ideal_s_n).abs();
        let diff_t_f = (personality.t_f - ideal_t_f).abs();
        let diff_j_p = (personality.j_p - ideal_j_p).abs();

        // 距离越小，匹配越高；最大距离为 2.0（从 -1.0 到 1.0）
        let avg_diff = (diff_e_i + diff_s_n + diff_t_f + diff_j_p) / 4.0;
        (1.0 - avg_diff / 2.0).max(0.0)
    }

    fn generate_reasons(personality: &PersonalityParams, career: &str) -> Vec<String> {
        let mut reasons = Vec::new();

        match career {
            "软件工程师" => {
                if personality.e_i > 0.0 {
                    reasons.push("偏好深度专注的独立工作".to_string());
                }
                if personality.s_n > 0.0 {
                    reasons.push("善于抽象思维和系统设计".to_string());
                }
                if personality.t_f < 0.0 {
                    reasons.push("逻辑分析能力强".to_string());
                }
            }
            "心理咨询师" => {
                if personality.t_f > 0.0 {
                    reasons.push("富有同理心，善于理解他人".to_string());
                }
                if personality.e_i < 0.0 {
                    reasons.push("善于与人建立信任关系".to_string());
                }
            }
            _ => {
                reasons.push("基于整体人格特征的综合匹配".to_string());
            }
        }

        reasons
    }
}

/// 为 Agent 生成默认的学科偏好
pub fn default_subject_preferences(career: &CareerAspiration) -> Vec<SubjectPreference> {
    match &career.category {
        CareerCategory::Science | CareerCategory::Technology => vec![
            SubjectPreference { subject: "数学".to_string(), preference: 0.8 },
            SubjectPreference { subject: "物理".to_string(), preference: 0.7 },
            SubjectPreference { subject: "信息技术".to_string(), preference: 0.9 },
        ],
        CareerCategory::Arts => vec![
            SubjectPreference { subject: "美术".to_string(), preference: 0.9 },
            SubjectPreference { subject: "音乐".to_string(), preference: 0.7 },
            SubjectPreference { subject: "语文".to_string(), preference: 0.6 },
        ],
        CareerCategory::Medicine => vec![
            SubjectPreference { subject: "生物".to_string(), preference: 0.9 },
            SubjectPreference { subject: "化学".to_string(), preference: 0.8 },
            SubjectPreference { subject: "数学".to_string(), preference: 0.6 },
        ],
        _ => vec![
            SubjectPreference { subject: "语文".to_string(), preference: 0.6 },
            SubjectPreference { subject: "数学".to_string(), preference: 0.5 },
            SubjectPreference { subject: "英语".to_string(), preference: 0.5 },
        ],
    }
}

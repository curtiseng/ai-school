//! M1.1 人格初始化引擎
//!
//! 从 MBTI 4D 分数生成行为倾向权重，支持随机生成保证分布多样性。

use rand::Rng;

use ai_school_core::types::PersonalityParams;

/// 人格行为倾向权重 — 从 MBTI 4D 映射到具体行为倾向
#[derive(Debug, Clone)]
pub struct BehaviorTendencies {
    /// 社交主动性 (0.0 被动 ~ 1.0 主动)
    pub social_initiative: f32,
    /// 学习方式偏好 (0.0 实践导向 ~ 1.0 理论导向)
    pub learning_style: f32,
    /// 决策方式 (0.0 逻辑分析 ~ 1.0 情感直觉)
    pub decision_style: f32,
    /// 计划性 (0.0 即兴 ~ 1.0 计划)
    pub planning: f32,
    /// 冒险倾向 (0.0 保守 ~ 1.0 冒险)
    pub risk_taking: f32,
    /// 共情能力 (0.0 ~ 1.0)
    pub empathy: f32,
    /// 独立性 (0.0 依赖 ~ 1.0 独立)
    pub independence: f32,
}

impl BehaviorTendencies {
    /// 从 MBTI 人格参数计算行为倾向
    pub fn from_personality(params: &PersonalityParams) -> Self {
        // E/I 维度影响：社交主动性、独立性
        let social_initiative = (1.0 - params.e_i) / 2.0; // E(-1) → 1.0, I(+1) → 0.0
        let independence = (1.0 + params.e_i) / 2.0; // I(+1) → 1.0, E(-1) → 0.0

        // S/N 维度影响：学习方式、冒险倾向
        let learning_style = (1.0 + params.s_n) / 2.0; // N(+1) → 理论, S(-1) → 实践
        let risk_taking = (1.0 + params.s_n) / 2.0; // N(+1) → 冒险, S(-1) → 保守

        // T/F 维度影响：决策方式、共情能力
        let decision_style = (1.0 + params.t_f) / 2.0; // F(+1) → 情感, T(-1) → 逻辑
        let empathy = (1.0 + params.t_f) / 2.0; // F(+1) → 高共情, T(-1) → 低共情

        // J/P 维度影响：计划性
        let planning = (1.0 - params.j_p) / 2.0; // J(-1) → 计划, P(+1) → 即兴

        Self {
            social_initiative,
            learning_style,
            decision_style,
            planning,
            risk_taking,
            empathy,
            independence,
        }
    }
}

/// 随机生成一组多样化的人格参数
pub fn generate_diverse_personalities(count: usize) -> Vec<PersonalityParams> {
    let mut rng = rand::thread_rng();
    let mut personalities = Vec::with_capacity(count);

    for i in 0..count {
        // 使用分层策略保证多样性
        let sector = i % 4;
        let (e_i_bias, s_n_bias) = match sector {
            0 => (-0.5, -0.5), // E+S 倾向
            1 => (0.5, 0.5),   // I+N 倾向
            2 => (-0.5, 0.5),  // E+N 倾向
            _ => (0.5, -0.5),  // I+S 倾向
        };

        let e_i = (e_i_bias + rng.gen_range(-0.5_f32..0.5)).clamp(-1.0, 1.0);
        let s_n = (s_n_bias + rng.gen_range(-0.5_f32..0.5)).clamp(-1.0, 1.0);
        let t_f: f32 = rng.gen_range(-1.0..1.0);
        let j_p: f32 = rng.gen_range(-1.0..1.0);

        let mut params = PersonalityParams::new(e_i, s_n, t_f, j_p);
        // 年轻学生的人格稳定度较低
        params.stability = rng.gen_range(0.5..1.5);

        personalities.push(params);
    }

    personalities
}

/// 根据 MBTI 类型生成人格描述文本（用于 Prompt）
pub fn personality_description(params: &PersonalityParams) -> String {
    let tendencies = BehaviorTendencies::from_personality(params);
    let mbti = params.mbti_label();

    let mut desc = format!("人格类型: {mbti}\n");

    // E/I
    if params.e_i < -0.3 {
        desc.push_str("性格外向，喜欢与人交流，从社交中获得能量。");
    } else if params.e_i > 0.3 {
        desc.push_str("性格内向，偏好独处和深度思考，社交时需要更多缓冲时间。");
    } else {
        desc.push_str("在社交和独处之间取得平衡，视情境调整。");
    }

    // S/N
    if params.s_n < -0.3 {
        desc.push_str("注重实际和细节，偏好动手实践的学习方式。");
    } else if params.s_n > 0.3 {
        desc.push_str("富有想象力和创造力，喜欢探索抽象概念和理论。");
    } else {
        desc.push_str("兼顾理论与实践，灵活运用不同的学习方式。");
    }

    // T/F
    if params.t_f < -0.3 {
        desc.push_str("倾向逻辑分析做决策，注重公平和客观标准。");
    } else if params.t_f > 0.3 {
        desc.push_str("重视他人感受，决策时考虑人际和谐，富有同理心。");
    } else {
        desc.push_str("在理性分析和情感考量之间保持平衡。");
    }

    // J/P
    if params.j_p < -0.3 {
        desc.push_str("做事有计划有条理，喜欢按时完成任务。");
    } else if params.j_p > 0.3 {
        desc.push_str("灵活应变，喜欢保持选择开放，适应性强。");
    } else {
        desc.push_str("在计划性和灵活性之间保持平衡。");
    }

    desc.push_str(&format!(
        "\n社交主动性: {:.0}%, 计划性: {:.0}%, 共情能力: {:.0}%",
        tendencies.social_initiative * 100.0,
        tendencies.planning * 100.0,
        tendencies.empathy * 100.0,
    ));

    desc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_behavior_tendencies() {
        // ENTJ: 外向 + 直觉 + 思考 + 判断
        let params = PersonalityParams::new(-0.8, 0.6, -0.7, -0.5);
        let tendencies = BehaviorTendencies::from_personality(&params);

        assert!(tendencies.social_initiative > 0.7); // E → 高社交主动性
        assert!(tendencies.learning_style > 0.5); // N → 理论导向
        assert!(tendencies.decision_style < 0.3); // T → 逻辑分析
        assert!(tendencies.planning > 0.5); // J → 有计划
    }

    #[test]
    fn test_generate_diverse() {
        let personalities = generate_diverse_personalities(8);
        assert_eq!(personalities.len(), 8);

        // Check diversity: not all should have the same MBTI label
        let labels: std::collections::HashSet<String> =
            personalities.iter().map(|p| p.mbti_label()).collect();
        assert!(labels.len() > 1);
    }

    #[test]
    fn test_personality_description() {
        let params = PersonalityParams::new(-0.8, 0.6, -0.7, -0.5);
        let desc = personality_description(&params);
        assert!(desc.contains("ENTJ"));
        assert!(desc.contains("外向"));
    }
}

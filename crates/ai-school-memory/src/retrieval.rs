//! 三维检索评分
//!
//! Score = α×relevance + β×recency + γ×importance

use ai_school_core::types::{ScoredMemory, SimulationTime};

/// 检索评分参数
#[derive(Debug, Clone)]
pub struct RetrievalWeights {
    /// 语义相关性权重
    pub alpha: f32,
    /// 时间衰减权重
    pub beta: f32,
    /// 重要性权重
    pub gamma: f32,
}

impl Default for RetrievalWeights {
    fn default() -> Self {
        Self {
            alpha: 0.5,
            beta: 0.3,
            gamma: 0.2,
        }
    }
}

/// 计算时间衰减分数
/// recency = exp(-decay_rate × hours_elapsed)
pub fn recency_score(memory_time: &SimulationTime, current_time: &SimulationTime) -> f32 {
    let elapsed = current_time.total_hours().saturating_sub(memory_time.total_hours());
    let decay_rate = 0.01; // 每小时衰减 1%
    (-decay_rate * elapsed as f64).exp() as f32
}

/// 重新评分记忆列表（应用时间衰减）
pub fn rescore_memories(
    memories: &mut [ScoredMemory],
    current_time: &SimulationTime,
    weights: &RetrievalWeights,
) {
    for mem in memories.iter_mut() {
        mem.recency = recency_score(&mem.memory.timestamp, current_time);
        mem.score = weights.alpha * mem.relevance
            + weights.beta * mem.recency
            + weights.gamma * mem.importance_score;
    }

    memories.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
}

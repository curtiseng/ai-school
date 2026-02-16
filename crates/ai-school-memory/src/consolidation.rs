//! 记忆巩固与遗忘

use ai_school_core::types::{Memory, MemoryLayer};

/// 判断记忆是否应该从短期提升到长期
pub fn should_consolidate(memory: &Memory) -> bool {
    // 高重要性的短期记忆应该巩固
    memory.layer == MemoryLayer::ShortTerm
        && (memory.importance >= 0.6 || memory.access_count >= 3)
}

/// 判断记忆是否应该被遗忘
pub fn should_forget(memory: &Memory, current_tick: u64) -> bool {
    if memory.layer == MemoryLayer::Semantic {
        return false; // 语义记忆不会遗忘
    }

    let age = current_tick.saturating_sub(memory.timestamp.tick);

    match memory.layer {
        MemoryLayer::Sensory => true, // 感知记忆总是被遗忘
        MemoryLayer::ShortTerm => {
            // 短期记忆：低重要性 + 超过 48 小时 + 低访问
            memory.importance < 0.3 && age > 48 && memory.access_count < 2
        }
        MemoryLayer::LongTerm => {
            // 长期记忆：非常低重要性 + 超过 1000 小时 + 零访问
            memory.importance < 0.1 && age > 1000 && memory.access_count == 0
        }
        MemoryLayer::Semantic => false,
    }
}

/// 提升记忆层级
pub fn promote_memory(memory: &mut Memory) {
    match memory.layer {
        MemoryLayer::ShortTerm => memory.layer = MemoryLayer::LongTerm,
        MemoryLayer::LongTerm => memory.layer = MemoryLayer::Semantic,
        _ => {}
    }
}

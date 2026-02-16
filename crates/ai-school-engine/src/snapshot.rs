//! 状态快照与回放

use ai_school_core::types::WorldSnapshot;

/// 快照管理器
pub struct SnapshotManager {
    snapshots: Vec<WorldSnapshot>,
    max_snapshots: usize,
}

impl SnapshotManager {
    pub fn new(max_snapshots: usize) -> Self {
        Self {
            snapshots: Vec::new(),
            max_snapshots,
        }
    }

    /// 保存快照
    pub fn save(&mut self, snapshot: WorldSnapshot) {
        if self.snapshots.len() >= self.max_snapshots {
            self.snapshots.remove(0);
        }
        self.snapshots.push(snapshot);
    }

    /// 获取最近的快照
    pub fn latest(&self) -> Option<&WorldSnapshot> {
        self.snapshots.last()
    }

    /// 获取指定 tick 的快照
    pub fn at_tick(&self, tick: u64) -> Option<&WorldSnapshot> {
        self.snapshots.iter().find(|s| s.time.tick == tick)
    }

    /// 获取所有快照的时间戳
    pub fn timeline(&self) -> Vec<u64> {
        self.snapshots.iter().map(|s| s.time.tick).collect()
    }
}

impl Default for SnapshotManager {
    fn default() -> Self {
        Self::new(1000)
    }
}

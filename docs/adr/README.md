# Architecture Decision Records

本目录包含 AI School 项目的架构决策记录（ADR）。

## 索引

| ADR | 标题 | 状态 | 日期 |
|-----|------|------|------|
| [0001](0001-system-architecture-overview.md) | AI School 仿真平台整体架构 | Proposed | 2026-02-16 |
| [0002](0002-personality-evolution-mechanism.md) | 人格演化跨模块闭环机制 | Proposed | 2026-02-16 |
| [0003](0003-world-system-llm-interaction.md) | 学校世界系统与 LLM 交互架构 | Proposed | 2026-02-16 |
| [0004](0004-narrative-data-dual-view.md) | 叙事-数据双视图对照设计 | Proposed | 2026-02-16 |
| [0005](0005-rust-tech-stack.md) | Rust 技术栈与核心依赖选型 | Proposed | 2026-02-16 |
| [0006](0006-project-structure.md) | 项目工程结构设计 | Proposed | 2026-02-16 |

## 创建新 ADR

1. 以 `NNNN-title-with-dashes.md` 格式命名
2. 按照 MADR 模板填写内容
3. 提交 PR 进行评审
4. 评审通过后更新本索引

## ADR 状态说明

- **Proposed**：讨论中，尚未最终决定
- **Accepted**：决策已确定，进入实施
- **Deprecated**：不再适用
- **Superseded**：被新的 ADR 替代
- **Rejected**：经讨论后未采纳

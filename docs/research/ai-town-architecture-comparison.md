# AI 小镇项目架构对比分析

> **对比对象**：Stanford Generative Agents (Smallville) · a16z AI Town · AI School（本项目）
>
> 最后更新：2026-02-16

---

## 目录

- [一、项目概览](#一项目概览)
- [二、项目结构详解](#二项目结构详解)
  - [Stanford Generative Agents](#1-stanford-generative-agentssmallville)
  - [a16z AI Town](#2-a16z-ai-town)
  - [AI School（本项目）](#3-ai-school本项目)
- [三、核心维度对比矩阵](#三核心维度对比矩阵)
- [四、关键架构差异分析](#四关键架构差异分析)
- [五、AI School 的差异化设计](#五ai-school-的差异化设计)
- [六、AI School 当前的差距与风险](#六ai-school-当前的差距与风险)
- [七、可借鉴的设计模式](#七可借鉴的设计模式)

---

## 一、项目概览

| 项目 | Stanford Generative Agents | a16z AI Town | AI School |
|------|---------------------------|-------------|-----------|
| **GitHub** | [joonspk-research/generative_agents](https://github.com/joonspk-research/generative_agents) | [a16z-infra/ai-town](https://github.com/a16z-infra/ai-town) | 本仓库 |
| **Stars** | 20,600+ | 9,200+ | — |
| **语言** | Python (Django) | TypeScript (Convex) | Rust (Cargo Workspace) |
| **定位** | 学术研究原型 | 可部署的 Starter Kit | 教育领域研究平台 + 未来产品化 |
| **论文** | [Generative Agents (UIST 2023)](https://arxiv.org/abs/2304.03442) | 灵感来自上述论文 | 基于 34 篇论文的系统设计 |
| **Agent 数量** | 25 | ~10（前端限制） | 设计目标未限定 |
| **场景** | 虚拟小镇 Smallville | 虚拟小镇（像素风） | 虚拟校园（教育仿真） |

---

## 二、项目结构详解

### 1. Stanford Generative Agents（Smallville）

原始论文项目，Python + Django，结构较为扁平：

```
generative_agents/
├── reverie/                              # 核心仿真引擎
│   ├── backend_server/
│   │   ├── reverie.py                    # 仿真主循环入口
│   │   ├── persona/                      # Agent 人格系统
│   │   │   ├── memory_structures/        # 记忆结构
│   │   │   │   ├── associative_memory.py # 联想记忆（记忆流）
│   │   │   │   └── spatial_memory.py     # 空间记忆
│   │   │   ├── cognitive_modules/        # 认知模块
│   │   │   │   ├── perceive.py           # 感知
│   │   │   │   ├── retrieve.py           # 记忆检索
│   │   │   │   ├── plan.py               # 规划
│   │   │   │   ├── reflect.py            # 反思
│   │   │   │   └── execute.py            # 执行
│   │   │   └── prompt_template/          # Prompt 模板
│   │   └── global_methods.py
│   └── compress_sim_storage.py
│
├── environment/
│   └── frontend_server/                  # Django 前端
│       ├── static_dirs/assets/           # 地图资源、角色精灵
│       │   └── the_ville/
│       │       ├── matrix/               # 瓦片地图矩阵
│       │       └── visuals/              # 视觉资源
│       ├── storage/                      # 仿真存档（JSON 文件）
│       ├── temp_storage/                 # 临时存储
│       └── templates/                    # HTML 模板
│
└── requirements.txt
```

**架构特点**：

- **单体结构**：所有后端逻辑集中在 `reverie/backend_server` 一个目录
- **认知管线清晰**：`perceive → retrieve → plan → reflect → execute` 五阶段管线
- **文件系统存储**：所有仿真状态以 JSON 文件存储在 `storage/` 目录，无数据库
- **前端简单**：Django 模板渲染 2D 瓦片地图，主要用于可视化调试
- **无抽象层**：Agent 直接调用 OpenAI API，无 provider 抽象或重试机制

---

### 2. a16z AI Town

工程化的 TypeScript 实现，前后端一体 monorepo，依赖 Convex 平台：

```
ai-town/
├── convex/                               # 后端（运行在 Convex 平台）
│   ├── aiTown/                           # 游戏逻辑层
│   │   ├── world.ts                      # 世界数据模型
│   │   ├── player.ts                     # 玩家模型（位置、路径寻找）
│   │   ├── conversations.ts              # 对话系统
│   │   ├── conversationMembership.ts     # 对话成员管理
│   │   ├── agent.ts                      # Agent 游戏循环逻辑
│   │   ├── inputs.ts                     # 输入处理（join/leave/moveTo/...）
│   │   ├── schema.ts                     # 游戏状态 Schema
│   │   ├── agentInputs.ts               # Agent 输入定义
│   │   └── main.ts                       # runStep 入口
│   │
│   ├── engine/                           # 通用游戏引擎
│   │   ├── abstractGame.ts               # 抽象游戏类（tick 循环）
│   │   └── schema.ts                     # 引擎 Schema
│   │
│   ├── agent/                            # Agent AI 系统
│   │   ├── conversations.ts              # 对话 Prompt 工程
│   │   ├── memory.ts                     # 记忆系统（向量检索）
│   │   └── embeddingsCache.ts            # 嵌入缓存
│   │
│   ├── util/
│   │   └── openai.ts                     # LLM 调用封装
│   │
│   └── crons.ts                          # 定时任务（背景音乐等）
│
├── src/                                  # 前端 UI
│   └── ...                               # pixi-react 像素游戏渲染
│
├── data/                                 # 配置数据
│   ├── characters.ts                     # 角色定义（名称、描述、精灵表）
│   └── gentle.js                         # 地图数据
│
├── assets/                               # 精灵表、图片资源
├── public/                               # 静态文件
├── fly/                                  # Fly.io 部署配置
├── docker-compose.yml
├── package.json
└── ARCHITECTURE.md                       # 架构文档
```

**架构特点**：

- **四层分离**：游戏逻辑（`aiTown`）/ 通用引擎（`engine`）/ Agent AI（`agent`）/ 前端 UI（`src`）
- **Tick-based 引擎**：60fps tick 推进仿真，每秒 1 次数据库写入（step），支持历史回放实现平滑运动
- **单线程引擎保证**：通过 generation number 机制确保同一世界只有一个引擎实例运行
- **输入驱动**：所有状态变更通过 `inputs` 表提交，人类玩家和 AI Agent 使用相同的输入接口
- **Convex 深度绑定**：后端完全依赖 Convex 平台（数据库、向量搜索、调度、实时订阅）
- **简化的记忆**：每次对话 → GPT 摘要 → 向量化 → 检索最相似 3 条记忆，无反思/遗忘机制

---

### 3. AI School（本项目）

Rust Cargo Workspace，8 crates，领域驱动设计（详见 [ADR-0006](../adr/0006-project-structure.md)）：

```
ai-school/
├── crates/
│   ├── ai-school-core/            # 共享类型 + trait 定义（领域语言）
│   ├── ai-school-llm/             # LLM 集成层（多 provider + Prompt 渲染）
│   ├── ai-school-agent/           # M1: 学生 Agent（人格、认知、职业）
│   ├── ai-school-world/           # M2: 学校世界（空间、时间、课程、社交）
│   ├── ai-school-memory/          # M4: 多层记忆（Qdrant + 反思 + 巩固 + 遗忘）
│   ├── ai-school-engine/          # M3: 演化引擎（仿真主循环 + GM 仲裁）
│   ├── ai-school-api/             # HTTP/WebSocket API 层
│   └── ai-school-cli/             # CLI 工具（批量实验/调试）
│
├── prompts/                        # Prompt 模板（.j2 Jinja2 文件）
│   ├── agent/                      # Agent 决策相关
│   ├── game_master/                # GM 仲裁相关
│   ├── memory/                     # 记忆反思相关
│   ├── consistency/                # 一致性检测
│   └── system/                     # 系统 Prompt
│
├── migrations/                     # PostgreSQL 迁移脚本
├── tests/                          # 跨 crate 集成测试
├── frontend/                       # 前端项目（独立技术栈，待定）
├── docs/                           # 文档
│   ├── adr/                        # 架构决策记录（6 篇）
│   ├── prd/                        # 产品需求文档
│   └── research/                   # 论文研究综述（本文件所在目录）
│
├── docker-compose.yml              # 开发环境（PostgreSQL + Qdrant）
├── Cargo.toml                      # Workspace 根配置
└── rust-toolchain.toml             # Rust 版本锁定
```

**架构特点**：

- **领域驱动的 crate 划分**：每个 crate 对应一个业务模块，依赖关系是单向 DAG
- **Trait 解耦**：`LlmProvider`、`MemoryStore` 等核心 trait 在 core 定义，实现在各自 crate
- **Agent 不依赖 LLM**：agent crate 只构建 `CompletionRequest`，实际调用由 engine 编排
- **双层仲裁架构**（ADR-0003）：结构化世界状态 + Game Master LLM 仲裁
- **四层记忆 + 六种遗忘**：感知 → 短期 → 长期 → 语义，参考 MaRS 论文
- **人格动态演化**（ADR-0002）：MBTI 4D 连续参数 + stability + shift_history
- **叙事-数据一致性检测**（ADR-0004）：防止 LLM 输出与结构化状态矛盾

---

## 三、核心维度对比矩阵

| 维度 | Stanford Smallville | a16z AI Town | AI School |
|------|-------------------|-------------|-----------|
| **语言** | Python | TypeScript | Rust |
| **模块化程度** | 低（单目录） | 中（4 层分离） | 高（8 个独立 crate） |
| **记忆系统** | 扁平记忆流 + 反思 | 简单向量检索（3 条） | 四层记忆 + 反思 + 巩固 + 遗忘 |
| **记忆检索** | α·recency + β·importance + γ·relevance | 余弦相似度 top-3 | 三维评分（relevance + recency + importance） |
| **人格系统** | 自然语言描述（静态） | 自然语言描述（静态） | MBTI 4D 连续参数 + 动态演化 |
| **世界系统** | 2D 瓦片地图 + 空间记忆 | 2D 像素地图 + 路径寻找 | 结构化校园 + 课程 + 时间层次 |
| **仲裁机制** | 无（Agent 直接行动） | 无（规则验证 moveTo 等） | Game Master LLM 仲裁层 |
| **仿真循环** | 单线程顺序执行 | tick-based 60fps + 异步 Agent | tokio 并发 Agent 决策 + GM 统一仲裁 |
| **存储** | 文件系统（JSON） | Convex 云数据库 + 向量搜索 | PostgreSQL + Qdrant |
| **LLM 抽象** | 直接调用 OpenAI | 简单封装 `openai.ts` | Trait 抽象 + 多 Provider + 结构化输出 |
| **前后端耦合** | 紧耦合（Django 模板） | 中度（Convex 实时订阅） | 松耦合（REST + WebSocket API 层） |
| **可测试性** | 低（无 mock 机制） | 中（Convex 测试工具） | 高（trait mock + 单元/集成/E2E） |
| **Prompt 管理** | 硬编码在 Python 中 | 硬编码在 TS 中 | 独立 .j2 模板文件，非程序员可维护 |
| **一致性检测** | 无 | 无 | ADR-0004 叙事-数据双视图校验 |
| **用户干预** | 可加载历史文件 | 人类玩家直接参与 | 参数调整 + 事件触发 + 角色对话 |
| **部署** | 本地 Python 服务 | Convex 云 / Docker / Fly.io | Docker Compose（PG + Qdrant） |

---

## 四、关键架构差异分析

### 4.1 记忆系统复杂度

**Smallville** — 开创性但简单：

```
记忆流（扁平列表）→ score = α·recency + β·importance + γ·relevance
反思触发：重要性累积 > 150 → LLM 生成高层认知
```

消融实验证明：完整架构与无记忆基线差异达 **8.16 个标准差**。

**a16z AI Town** — 极度简化：

```
每次对话结束 → GPT 摘要 → 向量化 → 存入 Convex 向量数据库
新对话开始 → 嵌入 "What you think about X?" → 检索 top-3 记忆 → 注入 Prompt
```

无反思机制、无重要性评分、无时间衰减。

**AI School** — 三者中最复杂：

```
四层架构：感知记忆 → 短期记忆 → 长期记忆 → 语义记忆
三维检索：α×relevance + β×recency + γ×importance
反思机制：累积经历超阈值 → 触发 LLM 反思 → 生成语义记忆
巩固机制：短期 → 长期的策略性迁移
遗忘机制：参考 MaRS 的 6 种遗忘策略
Qdrant 向量库：per-simulation Collection，支持 payload 过滤
```

### 4.2 仿真循环设计

**Smallville**：

```python
# reverie.py 伪代码
for step in range(num_steps):
    for persona in personas:
        persona.perceive(environment)
        persona.retrieve(perceived)
        persona.plan(retrieved)
        persona.reflect_if_needed()
        persona.execute(plan)
    environment.update()
```

顺序执行，每个 Agent 依次完成感知-检索-规划-反思-执行全流程。

**a16z AI Town**：

```typescript
// 引擎分离：tick vs step
// tick: 60fps 推进仿真（位置移动、碰撞检测等）
// step: 每秒 1 次，批量加载/保存游戏状态到数据库
// Agent: 异步 action，可调用 LLM，通过 input 提交状态变更
```

关键创新：将游戏引擎（确定性 tick）与 AI 行为（异步 action）分离。

**AI School**：

```rust
// SimulationRunner::step() 九步流程
// 1. 时间推进 → 2. 构建情境
// 3. 并发 Agent 决策（tokio::JoinSet，LLM 调用点 #1）
// 4. GM 仲裁（LLM 调用点 #2）
// 5. 一致性检测（ADR-0004）
// 6. 应用状态变更 → 7. 写入记忆
// 8. 检查人格演化 → 9. 广播更新
```

关键设计：Agent 决策并发化 + GM 统一仲裁 + 一致性断路器。

### 4.3 Agent 与 LLM 的耦合度

| 项目 | Agent 是否直接调用 LLM | 可测试性影响 |
|------|----------------------|------------|
| Smallville | 是，`cognitive_modules` 直接调用 OpenAI | 无法脱离 LLM 测试 Agent 逻辑 |
| a16z AI Town | 是，`convex/agent` 直接调用 | 依赖 Convex 环境 |
| AI School | 否，agent crate 只构建 `CompletionRequest` | 可用 mock LLM 独立测试 |

AI School 的设计使得 `ai-school-agent` crate 可以独立编译和测试，不依赖任何 LLM 服务。

### 4.4 世界状态管理

**Smallville**：`spatial_memory.py` 维护 Agent 在瓦片地图上的位置和已知区域。世界状态嵌入在 Agent 的空间记忆中，缺乏独立的世界状态管理器。

**a16z AI Town**：`convex/aiTown/world.ts` 定义世界模型，所有状态变更通过 `inputs` 提交。引擎保证单线程写入，无并发问题。状态通过 Convex 实时查询推送到前端。

**AI School**：`ai-school-world` crate 独立管理世界状态，通过 `apply_state_changes()` 单一入口保证"验证 → 执行 → 记录"的一致流程。世界系统包含空间、时间、课程、社交、关系矩阵等教育领域特化的子系统。

---

## 五、AI School 的差异化设计

以下是 AI School 相对于两个参考项目的独创或显著增强设计：

### 5.1 人格动态演化（ADR-0002）

Smallville 和 AI Town 的人格都是**静态的自然语言描述**，在整个仿真过程中不会改变。

AI School 设计了基于 MBTI 4D 连续参数的动态演化机制：

```
ΔΘ_actual = ΔΘ_signal × (1/stability) × decay
```

- 反思结论可触发人格微调信号
- stability 随年龄/经历增长，越高越难改变
- 每次变化记录 `PersonalityShift`，支持轨迹回溯

### 5.2 Game Master 仲裁层（ADR-0003）

两个参考项目都没有独立的仲裁层：
- Smallville：Agent 直接在环境中行动
- a16z AI Town：简单规则验证（如 moveTo 检查是否在对话中）

AI School 的双层架构：
- **结构化层**：`ai-school-world` 维护确定性世界状态
- **仲裁层**：`engine/game_master.rs` 用 LLM 验证行为合理性、仲裁多 Agent 交互、将自然语言翻译为结构化 `StateChange`

### 5.3 叙事-数据一致性检测（ADR-0004）

完全独创。用于解决 LLM 输出与结构化状态可能产生矛盾的问题：
- 情感强度匹配检测
- 因果关系检测
- 时间矛盾检测
- R3 噪声放大回路断路器

### 5.4 记忆巩固与遗忘

- Smallville 只有反思（重要性阈值触发），没有遗忘
- a16z AI Town 连反思都没有
- AI School 参考 MaRS 论文设计了 6 种遗忘策略 + 记忆巩固机制

### 5.5 教育领域特化

- 课程系统（`curriculum.rs`）：课程表、学科难度、学业反馈
- 职业志向模型（`career.rs`）：职业与学科偏好映射
- 身心发展追踪（`development.rs`）：心理/社交/学业能力指标
- 时间系统（`time.rs`）：学期/周/天/时段的层次化时间

### 5.6 用户干预系统

| 干预方式 | Smallville | a16z AI Town | AI School |
|---------|-----------|-------------|-----------|
| 加载历史数据 | ✓ | ✗ | ✓ |
| 人类玩家参与 | ✗ | ✓ | ✓（角色对话） |
| 参数调整 | ✗ | ✗ | ✓（课程难度、社交密度等） |
| 事件触发 | ✗ | ✗ | ✓（预设 + 自定义事件） |

---

## 六、AI School 当前的差距与风险

### 6.1 尚无可运行代码

两个参考项目都是**可运行的完整系统**。AI School 目前处于设计阶段，已有：
- 6 篇 ADR 架构文档
- 2 篇 PRD
- 4 篇研究综述
- Cargo Workspace 骨架（部分 crate 已初始化）

**建议**：尽快完成 P0 最小子集（core + llm + agent + memory），让系统"活"起来。

### 6.2 前端方案未定

- Smallville：Django 模板 + 2D 瓦片地图
- a16z AI Town：pixi-react 像素游戏
- AI School：`frontend/` 目录规划存在，但技术栈和设计未确定

### 6.3 复杂度风险

AI School 的设计复杂度远超两个参考项目：

| 指标 | Smallville | a16z AI Town | AI School |
|------|-----------|-------------|-----------|
| 模块数 | ~5 文件 | ~15 文件 | 8 crates × 多文件 |
| 记忆层级 | 1 层 | 1 层 | 4 层 |
| LLM 调用点 | 1 个（Agent） | 1 个（对话） | 2 个（Agent + GM）+ 反思 + 重要性评分 |
| 外部依赖 | OpenAI API | Convex + OpenAI/Ollama | PostgreSQL + Qdrant + 多 LLM Provider |

需要警惕 "设计完美但难以实现" 的风险。ADR-0006 的 P0-P4 分期策略是缓解措施。

### 6.4 LLM 成本

GM 仲裁层引入了额外的 LLM 调用。每个仿真步骤至少 2 次 LLM 调用（Agent 决策 + GM 仲裁），加上记忆反思和重要性评分，成本显著高于参考项目。需要在设计中考虑批量化和缓存策略。

---

## 七、可借鉴的设计模式

### 从 Smallville 借鉴

1. **认知管线**：`perceive → retrieve → plan → reflect → execute` 五阶段管线是经过消融实验验证的有效架构，AI School 的 `cognition.rs` 应保持类似结构
2. **反思触发机制**：重要性累积阈值（原始值 150）是一个经过验证的简单有效的设计
3. **空间记忆**：Agent 对环境的空间认知（已知区域、最近访问等）是行为合理性的重要支撑

### 从 a16z AI Town 借鉴

1. **引擎与游戏逻辑分离**：`engine/abstractGame.ts` 与 `aiTown/` 的分离模式值得参考——AI School 的 engine crate 也应注意通用引擎逻辑与仿真特定逻辑的边界
2. **Input 驱动的状态变更**：所有变更通过统一的 input 接口提交，保证单一入口——对应 AI School 的 `apply_state_changes()` 设计
3. **历史回放机制**：`HistoricalObject` 记录每个 tick 的状态变化用于平滑渲染——AI School 的 `snapshot.rs` 可参考这一思路
4. **人机统一接口**：人类玩家和 AI Agent 使用相同的输入接口——AI School 的干预系统也应遵循这一原则
5. **嵌入缓存**：`embeddingsCache.ts` 避免重复计算相同文本的嵌入——AI School 的 memory crate 应包含类似优化

---

## 参考链接

- [Stanford Generative Agents 论文](https://arxiv.org/abs/2304.03442)
- [Stanford Generative Agents GitHub](https://github.com/joonspk-research/generative_agents)
- [a16z AI Town GitHub](https://github.com/a16z-infra/ai-town)
- [a16z AI Town 架构文档](https://github.com/a16z-infra/ai-town/blob/main/ARCHITECTURE.md)
- [Stanford 1,000 People 项目](https://github.com/joonspk-research/genagents)
- [本项目 ADR-0006 项目结构设计](../adr/0006-project-structure.md)
- [本项目研究综述](generative-agents-deep-synthesis.md)

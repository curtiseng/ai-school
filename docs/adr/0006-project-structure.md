# ADR-0006: 项目工程结构设计

**Status**: Proposed
**Date**: 2026-02-16
**Deciders**: Product & Architecture Team

## Context

ADR-0001 定义了 6 大模块 18 子模块的业务架构，ADR-0005 确定了 Rust 技术栈。本 ADR 解决如何将业务模块映射为 Rust Cargo Workspace 的 crate 组织结构——这是从"架构图"到"可编译代码"的桥梁。

### 核心约束

1. **模块依赖是单向的**：ADR-0001 的依赖图 `M1→M3→M5, M2→M3, M4→M6` 是有向无环的，crate 结构必须保持这一性质
2. **跨模块闭环需要解耦**：ADR-0002 的人格演化闭环跨 M1↔M3↔M4，但 crate 依赖不能循环——需要通过 trait 抽象在共享层解耦
3. **渐进式构建**：P0→P1→P2 的优先级意味着 crate 间应低耦合，允许 P0 crate 独立编译和测试
4. **双入口**：系统需要 HTTP API 入口（给前端）和可能的 CLI 入口（用于批量实验/调试），两者共享引擎层

### 设计原则

- **依赖方向即信息流方向**：底层 crate 定义类型和 trait，上层 crate 提供实现和编排
- **trait 定义与实现分离**：核心 trait（`LlmProvider`、`MemoryStore`）在 `core` 中定义，实现在各自 crate
- **一个 crate 一个职责**：避免"上帝 crate"，每个 crate 对应一个清晰的领域边界

## Decision Drivers

- **编译速度**：合理的 crate 粒度使增量编译更快（改一个模块不需要重编所有代码）
- **可测试性**：每个 crate 可以独立测试，mock 掉外部依赖
- **并行开发**：不同 crate 可以由不同开发者并行推进
- **依赖隔离**：重量级依赖（如 qdrant-client、sqlx）被限制在特定 crate，不污染全局

## Considered Options

### Option 1: 单 crate 单二进制

所有代码放在一个 crate 中，用 module 划分。

- **Pros**: 简单，无跨 crate 依赖管理
- **Cons**: 编译慢（改任何文件重编全部），依赖不隔离，无法独立测试模块，违背模块化原则

### Option 2: 按技术层划分（controller / service / repository）

传统三层架构：API 层、业务逻辑层、数据访问层。

- **Pros**: 对有 Java/Spring 背景的团队熟悉
- **Cons**: 不匹配 AI School 的领域结构（M1-M6 是按领域划分，不是按技术层）

### Option 3: 按业务模块划分 + 共享核心层（推荐）

每个业务模块（M1-M4）对应一个 crate，加上共享核心层和基础设施层。

- **Pros**: crate 结构直接映射业务架构，领域边界清晰，依赖方向与信息流一致
- **Cons**: crate 间接口设计需要前期投入，共享核心层的类型设计是关键路径

## Decision

采用 **Option 3：按业务模块划分 + 共享核心层**，设计 8 个 crate 的 Cargo Workspace。

---

### 工程总览

```
ai-school/
├── Cargo.toml                         # Workspace 根配置
├── Cargo.lock
├── .cargo/
│   └── config.toml                    # Cargo 配置（linker 优化等）
├── .env.example                       # 环境变量模板
├── docker-compose.yml                 # 开发环境（PostgreSQL + Qdrant）
├── rust-toolchain.toml                # Rust 版本锁定
│
├── crates/
│   ├── ai-school-core/                # 共享类型 + trait 定义
│   ├── ai-school-llm/                 # LLM 集成层
│   ├── ai-school-memory/              # M4: 记忆与成长系统
│   ├── ai-school-agent/               # M1: 学生 Agent 系统
│   ├── ai-school-world/               # M2: 学校世界系统
│   ├── ai-school-engine/              # M3: 演化引擎（核心编排）
│   ├── ai-school-api/                 # HTTP/WebSocket API 层
│   └── ai-school-cli/                 # CLI 工具（批量实验/调试）
│
├── prompts/                           # Prompt 模板（.j2 文件）
│   ├── agent_decision.j2
│   ├── game_master_arbitrate.j2
│   ├── memory_reflection.j2
│   └── consistency_check.j2
│
├── migrations/                        # PostgreSQL 迁移脚本（sqlx）
│   ├── 20260216000001_create_agents.sql
│   ├── 20260216000002_create_world_state.sql
│   └── ...
│
├── tests/                             # 集成测试（跨 crate）
│   ├── simulation_loop_test.rs
│   └── memory_retrieval_test.rs
│
├── docs/                              # 文档（已有）
│   ├── adr/
│   ├── prd/
│   └── research/
│
└── frontend/                          # 前端项目（独立技术栈）
    ├── package.json
    ├── src/
    └── ...
```

---

### Crate 依赖图

```
                         ┌─────────────────┐
                         │  ai-school-core  │
                         │                  │
                         │  · 领域类型      │
                         │  · Trait 定义     │
                         │  · 错误类型      │
                         │  · 配置类型      │
                         └────────┬─────────┘
                                  │
                    ┌─────────────┼────────────────┐
                    │             │                 │
                    ▼             ▼                 ▼
           ┌──────────────┐ ┌──────────┐  ┌──────────────┐
           │ai-school-llm │ │ai-school │  │ ai-school    │
           │              │ │ -agent   │  │   -world     │
           │ · OpenAI     │ │          │  │              │
           │ · Anthropic  │ │ · 人格    │  │ · 校园空间   │
           │ · Prompt 渲染│ │ · 认知    │  │ · 课程系统   │
           │ · 嵌入       │ │ · 职业    │  │ · 社交活动   │
           └──────┬───────┘ └─────┬────┘  │ · 时间系统   │
                  │               │       │ · 关系矩阵   │
                  │               │       └──────┬───────┘
                  │               │              │
                  ▼               ▼              ▼
           ┌──────────────────────────────────────────────┐
           │               ai-school-memory                │
           │                                               │
           │  · Qdrant 集成（向量存储/检索）                 │
           │  · 多层记忆管理（感知→短期→长期→语义）           │
           │  · 三维检索评分（relevance + recency + importance）│
           │  · 反思机制                                    │
           │  · 记忆巩固与遗忘                              │
           └──────────────────────┬────────────────────────┘
                                  │
                                  ▼
           ┌──────────────────────────────────────────────┐
           │              ai-school-engine                  │
           │                                               │
           │  · M3.1 自主演化循环（仿真主循环）              │
           │  · Game Master 仲裁层                          │
           │  · M3.2 用户干预机制                           │
           │  · M3.3 事件与冲突生成                         │
           │  · 状态快照与回放                              │
           │  · ADR-0004 一致性检测                         │
           └──────────────────────┬────────────────────────┘
                                  │
                    ┌─────────────┴────────────────┐
                    ▼                              ▼
           ┌──────────────┐               ┌──────────────┐
           │ai-school-api │               │ai-school-cli │
           │              │               │              │
           │ · REST 路由   │               │ · 批量仿真   │
           │ · WebSocket   │               │ · 数据导出   │
           │ · CORS/压缩   │               │ · 调试工具   │
           └──────────────┘               └──────────────┘
```

**依赖矩阵**（✓ = 直接依赖）：

| crate ↓ 依赖于 → | core | llm | agent | world | memory | engine |
|------------------|------|-----|-------|-------|--------|--------|
| **llm**          | ✓    |     |       |       |        |        |
| **agent**        | ✓    |     |       |       |        |        |
| **world**        | ✓    |     |       |       |        |        |
| **memory**       | ✓    | ✓   |       |       |        |        |
| **engine**       | ✓    | ✓   | ✓     | ✓     | ✓      |        |
| **api**          | ✓    |     |       |       |        | ✓      |
| **cli**          | ✓    |     |       |       |        | ✓      |

**关键设计决策**：

- `agent` 和 `world` 不依赖 `llm`——它们只定义领域逻辑，LLM 调用由 `engine` 编排
- `memory` 依赖 `llm`——因为记忆向量化需要调用 `LlmProvider::embed()`
- `api` 和 `cli` 只依赖 `engine`——通过 engine 的 facade 访问所有能力，不直接穿透到底层 crate
- **循环依赖通过 trait 解耦**——ADR-0002 的人格演化闭环（M1↔M3↔M4）在运行时由 engine 连接，编译时各 crate 只依赖 core 中定义的 trait

---

### 各 Crate 详细设计

#### 1. `ai-school-core` — 共享核心层

**职责**：定义所有 crate 共享的领域类型、trait 接口和错误类型。这是整个系统的"语言"——所有模块用 core 中的类型进行交流。

```
ai-school-core/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── types/
    │   ├── mod.rs
    │   ├── agent.rs          # AgentId, AgentConfig, AgentState
    │   ├── personality.rs    # PersonalityParams (MBTI 4D + stability + shift_history)
    │   ├── memory.rs         # Memory, MemoryLayer, MemoryQuery, ScoredMemory, MemoryId
    │   ├── world.rs          # Location, SimulationTime, Relationship, WorldSnapshot
    │   ├── event.rs          # SimulationEvent, StateChange, EventType, EventTrigger
    │   ├── behavior.rs       # BehaviorIntent, Perception, SituationContext
    │   └── career.rs         # CareerAspiration, CareerMatch
    │
    ├── traits/
    │   ├── mod.rs
    │   ├── llm.rs            # LlmProvider trait (complete, complete_structured, embed)
    │   ├── memory_store.rs   # MemoryStore trait (store, retrieve, get_recent)
    │   └── world_state.rs    # WorldStateReader trait (get_agent_state, snapshot, ...)
    │
    ├── error.rs              # 领域错误枚举 (AgentError, MemoryError, SimulationError, ...)
    └── config.rs             # SimulationConfig, LlmConfig, QdrantConfig, DatabaseConfig
```

**核心 trait 设计**：

```rust
// === traits/llm.rs ===
// 对应 ADR-0005 选型五，覆盖 ADR-0003 的两个 LLM 调用点 + M4 嵌入需求

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn complete(
        &self,
        request: &CompletionRequest,
    ) -> Result<CompletionResponse, LlmError>;

    async fn complete_structured<T: DeserializeOwned + Send>(
        &self,
        request: &CompletionRequest,
        schema: &serde_json::Value,
    ) -> Result<T, LlmError>;

    async fn embed(
        &self,
        texts: &[String],
    ) -> Result<Vec<Vec<f32>>, LlmError>;
}

// === traits/memory_store.rs ===
// 对应 ADR-0005 选型四的 Qdrant 记忆存储

#[async_trait]
pub trait MemoryStore: Send + Sync {
    async fn store(
        &self,
        agent_id: &AgentId,
        memory: &Memory,
        embedding: &[f32],
    ) -> Result<MemoryId, MemoryError>;

    async fn retrieve(
        &self,
        agent_id: &AgentId,
        query: &MemoryQuery,
    ) -> Result<Vec<ScoredMemory>, MemoryError>;

    async fn get_recent(
        &self,
        agent_id: &AgentId,
        layer: MemoryLayer,
        limit: usize,
    ) -> Result<Vec<Memory>, MemoryError>;

    async fn consolidate(
        &self,
        agent_id: &AgentId,
        memories: &[MemoryId],
        consolidated: &Memory,
        embedding: &[f32],
    ) -> Result<MemoryId, MemoryError>;
}
```

**核心类型设计**（与 ADR-0002 人格参数空间对齐）：

```rust
// === types/personality.rs ===

/// MBTI 4 维连续人格参数 — 对应 ADR-0002 的人格参数空间 Θ
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PersonalityParams {
    /// 外倾 (-1.0) ←→ 内倾 (+1.0)
    pub e_i: f32,
    /// 感知 (-1.0) ←→ 直觉 (+1.0)
    pub s_n: f32,
    /// 思考 (-1.0) ←→ 情感 (+1.0)
    pub t_f: f32,
    /// 判断 (-1.0) ←→ 知觉 (+1.0)
    pub j_p: f32,
    /// 人格稳定度（随年龄/经历增长，越高越难改变）
    pub stability: f32,
    /// 人格变化历史
    pub shift_history: Vec<PersonalityShift>,
}

/// 人格微调记录 — 支持 M5 研究分析的轨迹回溯
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PersonalityShift {
    pub timestamp: SimulationTime,
    pub trigger_event_id: EventId,
    pub dimension: PersonalityDimension,
    pub delta: f32,
    pub reason: String,
}
```

**依赖**：仅 serde、chrono、uuid、thiserror、schemars、async-trait 等基础库。不依赖任何业务 crate。

---

#### 2. `ai-school-llm` — LLM 集成层

**职责**：实现 `LlmProvider` trait，封装所有 LLM API 交互逻辑，管理 Prompt 模板渲染。

```
ai-school-llm/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── providers/
    │   ├── mod.rs
    │   ├── openai.rs          # OpenAI / OpenAI-compatible (Azure, vLLM, Ollama)
    │   ├── anthropic.rs       # Anthropic Claude API
    │   └── mock.rs            # 测试用 mock provider
    │
    ├── prompt.rs              # Prompt 模板渲染（minijinja 集成）
    ├── structured.rs          # JSON Schema 约束输出 + 验证
    └── retry.rs               # 重试策略（tower::retry 集成）
```

**设计要点**：

- `openai.rs` 通过 `async-openai` 实现，支持所有 OpenAI 兼容端点（Azure OpenAI、vLLM、Ollama）
- `anthropic.rs` 通过 `reqwest` 直接调用 Anthropic Messages API
- `structured.rs` 实现 `complete_structured<T>` 的核心逻辑：将 `schemars` 生成的 JSON Schema 注入 Prompt → 调用 LLM → 解析输出 → `jsonschema` 验证 → 反序列化
- `prompt.rs` 加载 `prompts/` 目录下的 `.j2` 模板文件，对应 ADR-0005 的四类模板
- `retry.rs` 基于 tower 的重试中间件，处理 LLM API 的 rate limit 和瞬时错误

**依赖**：`ai-school-core`、`async-openai`、`reqwest`、`minijinja`、`jsonschema`、`tower`

---

#### 3. `ai-school-agent` — M1 学生 Agent 系统

**职责**：定义学生 Agent 的领域模型——人格初始化、职业志向、认知行为框架。**不直接调用 LLM**，只定义"如何组装一个 Agent 的决策输入"。

```
ai-school-agent/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── personality.rs         # M1.1 人格初始化引擎
    │                          #   - 从 MBTI 4D 分数生成行为倾向权重
    │                          #   - 随机生成算法（保证分布多样性）
    │
    ├── career.rs              # M1.2 职业志向模型
    │                          #   - 职业与学科偏好的映射
    │                          #   - 职业志向对行为的影响系数
    │
    ├── cognition.rs           # M1.3 认知与行为框架
    │                          #   - 感知: 从情境中提取与人格相关的信息
    │                          #   - 思考: 组装 LLM 决策所需的完整上下文
    │                          #   - 行动: 解析 LLM 输出为 BehaviorIntent
    │
    └── builder.rs             # Agent 配置构建器（用于 F3 配置面板）
```

**设计要点**：

- `cognition.rs` 的 `think()` 方法不调用 LLM，而是返回一个 `CompletionRequest`——实际调用由 engine 执行。这保持了 agent crate 对 LLM 的零依赖
- 人格参数如何"调制"决策通过 `cognition.rs` 中的上下文组装逻辑体现（将人格维度映射为 Prompt 中的行为指导语）

**依赖**：仅 `ai-school-core`

---

#### 4. `ai-school-world` — M2 学校世界系统

**职责**：定义和管理结构化的校园环境——空间、时间、课程、社交网络。对应 ADR-0003 的"结构化世界状态管理器"。

```
ai-school-world/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── campus.rs              # 校园空间模型
    │                          #   - 功能区域定义（教室、图书馆、操场...）
    │                          #   - Agent 位置追踪
    │                          #   - 空间邻接关系（用于行为合理性验证）
    │
    ├── time.rs                # M2.3 环境与时间系统
    │                          #   - SimulationClock: 仿真时钟推进
    │                          #   - 学期/周/天/时段的层次时间
    │                          #   - 时间事件触发器（"09:00 数学课开始"）
    │
    ├── curriculum.rs          # M2.1 课程与学习活动
    │                          #   - 课程表定义
    │                          #   - 学科难度模型
    │                          #   - 学业反馈计算
    │
    ├── social.rs              # M2.2 社交与校园活动
    │                          #   - 社团模型
    │                          #   - 社交事件模板（聚餐、辩论、运动会...）
    │
    ├── relationships.rs       # 关系矩阵
    │                          #   - Agent 间亲密度
    │                          #   - 群组归属
    │                          #   - 关系变更应用（来自 GM 的 state_changes）
    │
    └── state.rs               # 世界状态管理器（核心）
                               #   - 聚合所有子系统的状态
                               #   - 状态变更的验证与应用
                               #   - 快照生成
                               #   - 情境描述生成（结构化 → 自然语言）
```

**设计要点**：

- `state.rs` 是 ADR-0003 图中"结构化世界状态管理器"的直接实现
- 所有状态变更通过 `apply_state_changes(changes: &[StateChange]) -> Result<()>` 单一入口——保证 ADR-0003 要求的"验证合法性 → 执行状态更新 → 记录事件日志"流程
- `time.rs` 的 `SimulationClock` 与 tokio 的 `Interval` 配合，实现仿真时间与现实时间的可配置映射（回应 D5）

**依赖**：仅 `ai-school-core`

---

#### 5. `ai-school-memory` — M4 记忆与成长系统

**职责**：实现 `MemoryStore` trait，管理多层记忆的存储、检索、反思、巩固和遗忘。集成 Qdrant。

```
ai-school-memory/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── store/
    │   ├── mod.rs
    │   ├── qdrant.rs          # Qdrant 实现 MemoryStore trait
    │   │                      #   - Collection 管理（per-simulation）
    │   │                      #   - Point CRUD（带 payload 和向量）
    │   │                      #   - 批量操作
    │   │
    │   └── in_memory.rs       # 内存实现（测试用 + 感知记忆/短期缓冲）
    │
    ├── retrieval.rs           # 三维检索评分
    │                          #   - relevance: 向量余弦相似度 (Qdrant)
    │                          #   - recency: 时间衰减函数
    │                          #   - importance: 预计算重要性分数
    │                          #   - 综合评分 = α×relevance + β×recency + γ×importance
    │
    ├── reflection.rs          # 反思机制
    │                          #   - 累积经历评估 → 触发反思（阈值判断）
    │                          #   - 反思 Prompt 构建
    │                          #   - 反思结果 → 语义记忆
    │
    ├── consolidation.rs       # 记忆巩固与遗忘
    │                          #   - 短期 → 长期的巩固策略
    │                          #   - 重要性评估
    │                          #   - 遗忘策略（基于 MaRS 的 6 种遗忘机制）
    │
    ├── development.rs         # M4.2 身心发展追踪
    │                          #   - 心理状态指标
    │                          #   - 社交能力指标
    │                          #   - 学业能力指标
    │                          #   - 指标变化历史
    │
    └── evolution.rs           # M4.3 人格动态演变
                               #   - ADR-0002 的演化速率控制
                               #   - ΔΘ_actual = ΔΘ_signal × (1/stability) × decay
                               #   - 反思结论 → 人格微调信号
                               #   - PersonalityShift 记录
```

**Qdrant Collection 设计**：

```
Collection: {simulation_id}_memories
├── Vectors: 1536-dim (OpenAI text-embedding-3-small) 或模型适配
├── Payload Schema:
│   ├── agent_id: keyword          # Agent 隔离
│   ├── layer: keyword             # "sensory" | "shortterm" | "longterm" | "semantic"
│   ├── timestamp: integer         # 仿真时间戳（用于 recency 计算）
│   ├── importance: float          # 预计算重要性分数
│   ├── content: text              # 记忆文本内容
│   ├── event_id: keyword          # 关联的 SimulationEvent ID
│   ├── emotion_valence: float     # 情绪效价（正/负）
│   └── tags: keyword[]            # 标签（"academic", "social", "conflict" ...）
└── Indexes:
    ├── agent_id: exact match
    ├── layer: exact match
    ├── timestamp: range
    └── importance: range
```

**依赖**：`ai-school-core`、`ai-school-llm`（用于 `embed()` 和反思 Prompt）、`qdrant-client`

---

#### 6. `ai-school-engine` — M3 演化引擎

**职责**：核心编排层。实现仿真主循环，协调 Agent 决策、GM 仲裁、状态更新、记忆写入的完整流程。这是"胶水层"——将所有领域 crate 连接成可运行的仿真。

```
ai-school-engine/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── simulation.rs          # M3.1 自主演化循环（仿真主循环）
    │                          #
    │                          #   SimulationRunner {
    │                          #     1. 时间推进 → 触发时间事件
    │                          #     2. 为每个活跃 Agent 构建 SituationContext
    │                          #     3. 并发执行 Agent 决策（tokio::JoinSet）
    │                          #     4. 收集 BehaviorIntent → GM 仲裁
    │                          #     5. 应用 StateChanges → 世界状态更新
    │                          #     6. 写入记忆 → 检查反思触发
    │                          #     7. 广播更新 → WebSocket 推送
    │                          #   }
    │
    ├── game_master.rs         # Game Master 仲裁层
    │                          #   - 对应 ADR-0003 LLM 调用点 #2
    │                          #   - 行为合理性验证
    │                          #   - 多 Agent 交互仲裁
    │                          #   - 自然语言 → 结构化 StateChange 翻译
    │                          #   - JSON Schema 验证 GM 输出
    │
    ├── intervention.rs        # M3.2 用户干预机制
    │                          #   - 参数调整（课程难度、社交密度等）
    │                          #   - 事件触发（预设 + 自定义）
    │                          #   - 对话干预（以角色身份与 Agent 对话）
    │                          #   - 干预日志记录
    │
    ├── event_gen.rs           # M3.3 事件与冲突生成
    │                          #   - 基于世界状态的自动事件检测
    │                          #   - 阈值触发（关系破裂、学业危机等）
    │                          #   - 叙事性事件生成
    │
    ├── consistency.rs         # ADR-0004 一致性检测
    │                          #   - 叙事 vs 数据变更的一致性校验
    │                          #   - 情感强度匹配、因果检测、时间矛盾
    │                          #   - R3 噪声放大回路断路器
    │
    ├── snapshot.rs            # 状态快照与回放
    │                          #   - PostgreSQL 事务快照 + Qdrant Collection 快照
    │                          #   - 快照恢复
    │                          #   - 事件回放
    │
    └── broadcast.rs           # 状态广播
                               #   - tokio::sync::broadcast channel
                               #   - 仿真事件流（供 WebSocket 消费）
```

**仿真主循环设计**：

```rust
/// 仿真主循环 — 对应 ADR-0003 数据流的完整实现
pub struct SimulationRunner<L, M, W>
where
    L: LlmProvider,
    M: MemoryStore,
{
    llm: Arc<L>,
    memory: Arc<M>,
    world: WorldState,        // M2 世界状态
    agents: Vec<StudentAgent>, // M1 Agent 集合
    game_master: GameMaster<L>,
    clock: SimulationClock,
    event_tx: broadcast::Sender<SimulationEvent>,
    config: SimulationConfig,
}

impl<L: LlmProvider, M: MemoryStore> SimulationRunner<L, M> {
    pub async fn step(&mut self) -> Result<StepResult> {
        // 1. 时间推进
        let time_events = self.clock.advance();

        // 2. 为每个 Agent 构建情境
        let contexts = self.world.build_contexts(&self.agents, &time_events);

        // 3. 并发 Agent 决策（ADR-0003 LLM 调用点 #1）
        let intents = self.parallel_agent_decisions(&contexts).await?;

        // 4. Game Master 仲裁（ADR-0003 LLM 调用点 #2）
        let gm_output = self.game_master.arbitrate(&intents, &self.world).await?;

        // 5. 一致性检测（ADR-0004）
        let warnings = self.check_consistency(&gm_output);

        // 6. 应用状态变更
        self.world.apply_state_changes(&gm_output.state_changes)?;

        // 7. 写入记忆 + 检查反思
        self.update_memories(&gm_output).await?;

        // 8. 检查人格演化（ADR-0002 闭环）
        self.check_personality_evolution().await?;

        // 9. 广播更新
        let event = SimulationEvent::from_step(/*...*/);
        self.event_tx.send(event)?;

        Ok(StepResult { warnings, /*...*/ })
    }
}
```

**依赖**：`ai-school-core`、`ai-school-llm`、`ai-school-agent`、`ai-school-world`、`ai-school-memory`

---

#### 7. `ai-school-api` — HTTP/WebSocket API 层

**职责**：为前端提供 REST API 和 WebSocket 实时通信。

```
ai-school-api/
├── Cargo.toml
└── src/
    ├── main.rs                # 应用入口（tokio::main, 初始化所有依赖）
    ├── state.rs               # AppState（Arc 包装的共享状态）
    ├── routes/
    │   ├── mod.rs
    │   ├── simulation.rs      # POST /simulations, PUT /simulations/:id/speed, ...
    │   ├── agents.rs          # POST /agents, GET /agents/:id, PATCH /agents/:id
    │   ├── intervention.rs    # POST /interventions/event, PATCH /interventions/params
    │   ├── chat.rs            # POST /agents/:id/chat
    │   └── analysis.rs        # GET /analysis/timeline, GET /analysis/export
    │
    ├── ws.rs                  # WebSocket 端点（/ws/simulation/:id）
    │                          #   - 订阅 engine 的 broadcast channel
    │                          #   - 推送 SimulationEvent 到前端
    │
    ├── dto.rs                 # Request/Response DTO（与 core types 的转换）
    ├── error.rs               # API 错误到 HTTP 响应的映射
    └── middleware.rs           # 请求日志、CORS 等
```

**依赖**：`ai-school-core`、`ai-school-engine`、`axum`、`tower-http`

---

#### 8. `ai-school-cli` — CLI 工具

**职责**：提供命令行界面，用于批量实验、数据导出和调试。

```
ai-school-cli/
├── Cargo.toml
└── src/
    ├── main.rs
    └── commands/
        ├── run.rs             # 运行仿真（无 UI，纯数据输出）
        ├── export.rs          # 导出仿真数据（JSON/CSV）
        ├── replay.rs          # 从快照回放仿真
        └── inspect.rs         # 查看 Agent 状态、记忆、人格轨迹
```

**依赖**：`ai-school-core`、`ai-school-engine`、`clap`（CLI 参数解析）

---

### Prompt 模板目录

Prompt 模板独立于 crate 代码，放在项目根目录，方便非程序员参与维护：

```
prompts/
├── agent/
│   ├── decision.j2                  # Agent 决策 Prompt（调用点 #1）
│   ├── decision_with_career.j2      # 带职业志向的决策变体
│   └── perceive.j2                  # 感知阶段 Prompt
│
├── game_master/
│   ├── arbitrate.j2                 # GM 仲裁 Prompt（调用点 #2）
│   ├── validate_action.j2           # 行为合理性验证
│   └── resolve_conflict.j2          # 多 Agent 冲突仲裁
│
├── memory/
│   ├── reflection.j2                # 反思机制 Prompt
│   ├── importance_scoring.j2        # 记忆重要性评分
│   └── consolidation.j2             # 记忆巩固摘要
│
├── consistency/
│   └── narrative_data_check.j2      # ADR-0004 叙事-数据一致性检测
│
└── system/
    ├── base_personality.j2          # 人格基础 System Prompt（供所有 Agent 模板继承）
    └── game_master_role.j2          # GM 角色定义 System Prompt
```

---

### 测试策略

| 层级 | 位置 | 关注点 | 工具 |
|------|------|--------|------|
| **单元测试** | 各 crate 的 `#[cfg(test)]` module | 单个函数/方法的正确性 | `#[test]`, `mockall` |
| **集成测试** | 各 crate 的 `tests/` 目录 | crate 内部多组件协作 | `tokio::test`, `wiremock` |
| **端到端测试** | 根目录 `tests/` | 完整仿真循环、API 端到端 | `testcontainers`（PostgreSQL + Qdrant） |

**关键 Mock 边界**：

| 被 Mock 的 trait | 在哪里 Mock | 目的 |
|-----------------|-----------|------|
| `LlmProvider` | agent、engine 的测试 | 不依赖真实 LLM API |
| `MemoryStore` | engine 的测试 | 不依赖真实 Qdrant |
| `WorldStateReader` | 需要读取世界状态的测试 | 隔离世界状态依赖 |

---

### Docker Compose 开发环境

```yaml
# docker-compose.yml
services:
  postgres:
    image: postgres:17
    ports: ["5432:5432"]
    environment:
      POSTGRES_DB: ai_school
      POSTGRES_USER: ai_school
      POSTGRES_PASSWORD: dev_password
    volumes:
      - pgdata:/var/lib/postgresql/data

  qdrant:
    image: qdrant/qdrant:v1.13.0
    ports:
      - "6333:6333"    # REST API
      - "6334:6334"    # gRPC
    volumes:
      - qdrant_data:/qdrant/storage

volumes:
  pgdata:
  qdrant_data:
```

---

### 与 P0-P4 优先级的对应

| 优先级 | 需要的 crate | 最小可编译/可测试子集 |
|--------|------------|-------------------|
| **P0** | core + llm + agent(部分) + memory(部分) | 人格初始化 + 认知框架 + 记忆存储/检索 |
| **P1** | + world + engine(部分) | 最小仿真循环：Agent 决策 → GM 仲裁 → 状态更新 |
| **P2** | + engine(完整) | 用户干预、事件生成、人格演化闭环 |
| **P3** | + api + cli | Web 界面接入、数据导出 |
| **P4** | 新 crate（ai-school-companion） | Phase 2 陪伴机器人 |

P0 阶段只需 4 个 crate 即可独立运行和测试，验证核心假设。

---

## Consequences

### Positive

- **crate 结构直接映射业务架构**——新团队成员看 crate 名称就知道对应哪个业务模块
- **编译隔离**——修改 memory crate 不会触发 agent crate 重编译（通过 trait 解耦）
- **测试隔离**——每个 crate 可以独立测试，mock 外部依赖，CI 可以并行测试各 crate
- **依赖隔离**——Qdrant 依赖仅存在于 memory crate，sqlx 主要在 engine/api，不会全局污染
- **渐进式构建**——P0 只需 4 个 crate，可以快速达到第一个可运行的里程碑
- **ADR-0002 闭环在 engine 中实现**——人格演化的跨模块协调有单一清晰的所有者

### Negative

- **前期设计成本**——core 中的类型和 trait 设计是关键路径，需要仔细推敲
- **跨 crate 重构成本**——如果 core 中的类型定义需要大改，所有下游 crate 都需要适配
- **8 个 crate 的维护开销**——版本管理、CI 配置、依赖更新的复杂度比单 crate 高

### Risks

- **core 类型过度设计**
  - 缓解：P0 阶段先定义最小类型集，随需扩展，不要试图预见所有需求
  - 缓解：利用 Rust 的 `#[non_exhaustive]` 属性为将来的扩展预留空间

- **Prompt 模板与代码脱节**
  - 缓解：集成测试中包含 Prompt 渲染验证，确保模板变量与代码一致

- **engine crate 成为"上帝模块"**
  - 缓解：engine 内部按职责拆分为多个清晰的模块（simulation/game_master/intervention/...），每个模块有独立的文件和测试

## Implementation Notes

### Workspace Cargo.toml 配置

```toml
[workspace]
members = ["crates/*"]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2024"
rust-version = "1.85"
license = "MIT"

[workspace.dependencies]
# 在此统一管理版本，各 crate 通过 workspace = true 引用
# （具体版本见 ADR-0005 完整依赖清单）
```

### 启动顺序

1. 创建 workspace + core crate（定义最小类型集）
2. 创建 llm crate（实现 OpenAI provider + mock provider）
3. 创建 agent crate（人格初始化 + 认知框架）
4. 创建 memory crate（Qdrant 集成 + 基础检索）
5. 创建 engine crate（最小仿真循环）
6. 创建 api crate（基础 REST + WebSocket）
7. 创建 cli crate（批量运行工具）
8. 初始化前端项目

## Related Decisions

- [ADR-0001](0001-system-architecture-overview.md): 整体架构（6 模块 → 8 crate 的映射）
- [ADR-0002](0002-personality-evolution-mechanism.md): 人格演化闭环（通过 engine crate 协调）
- [ADR-0003](0003-world-system-llm-interaction.md): 双层架构（world crate = 结构化层，engine 中 game_master = 仲裁层）
- [ADR-0004](0004-narrative-data-dual-view.md): 叙事-数据双视图（engine 中 consistency.rs 实现）
- [ADR-0005](0005-rust-tech-stack.md): 技术栈选型（本 ADR 将选型落地为 crate 结构）

## References

- [Cargo Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Domain-Driven Design: crate 边界对应 Bounded Context
- Hexagonal Architecture: core 中的 trait 对应 Port，llm/memory 中的实现对应 Adapter

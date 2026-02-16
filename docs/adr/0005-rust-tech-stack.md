# ADR-0005: Rust 技术栈与核心依赖选型

**Status**: Proposed
**Date**: 2026-02-16
**Deciders**: Product & Architecture Team

## Context

ADR-0001 定义了 AI School 仿真平台的 6 大模块 18 子模块架构，同时列出了 7 个待决技术决策点（D1-D7）。本 ADR 解决其中与技术栈直接相关的决策：

| 待决编号 | 问题 | 本 ADR 覆盖 |
|---------|------|------------|
| D1 | LLM 选型：API 调用 vs 本地部署 vs 混合 | ✅ LLM 集成策略 |
| D3 | 记忆架构：向量数据库选型与分层策略 | ✅ Qdrant 选型 |
| D4 | 仿真规模：初始 Agent 数量与并发策略 | ✅ 并发模型 |
| D7 | 前端形态 | ✅ 前后端分离策略 |

### 系统的技术特征

从已有架构决策中提炼，仿真引擎有以下显著技术特征：

1. **I/O 密集**：每个仿真步进涉及多次 LLM API 调用（ADR-0003 定义的两个调用点），单次调用延迟 1-10s
2. **并发密集**：多 Agent 决策可并行（5-10 MVP，远期 100+），共享世界状态需安全读写
3. **长期运行**：仿真可能持续数小时甚至数天，内存安全和稳定性至关重要
4. **类型复杂**：18 个子模块间的数据流需要严格的接口契约（ADR-0001 Negative 中明确指出的挑战）
5. **状态密集**：结构化世界状态需支持快照、回放、复杂查询（ADR-0003、ADR-0004）
6. **双数据库**：结构化数据（世界状态、关系矩阵）+ 向量数据（记忆检索）需要不同的存储引擎

### 核心设计约束

来自已有 ADR 的刚性约束：

- LLM **仅在两个明确点**被调用（ADR-0003），其余为传统计算 → 编排层性能直接影响吞吐
- Game Master 输出必须符合 **JSON Schema**（ADR-0003 Implementation Notes）→ 需强结构化输出能力
- 人格演化闭环跨越 M1↔M3↔M4 三大模块（ADR-0002）→ 接口契约是生命线
- 记忆系统是**核心研究对象**（四层架构来自 34 篇论文的研究综合）→ 不能被框架抽象掉

## Decision Drivers

- **并发安全**：共享世界状态的多 Agent 读写需要编译期保证，而非运行时祈祷
- **类型即契约**：18 子模块的协作依赖精确的类型定义，类型系统的强度直接决定重构成本
- **性能不浪费**：LLM API 已是延迟瓶颈，编排层必须高效到"透明"
- **长期可靠**：仿真是科学研究工具，崩溃 = 丢失实验数据
- **研究级掌控**：记忆系统、认知循环、人格演化是核心研究领域，需要对每一层完全掌控
- **生态可用性**：所选技术栈在 Web、数据库、LLM 集成方面需有成熟可用的库

## Considered Options — 语言选型

### Option 1: Python

- **Pros**: LLM 生态最丰富（LangChain、LlamaIndex、OpenAI SDK），原型开发最快，数据科学工具链完善
- **Cons**: GIL 限制真并发（多 Agent 并行需要 multiprocessing 或 asyncio 绕行），类型系统可选且不严格，运行时错误在复杂系统中难以追踪，长期运行的内存管理不可控

### Option 2: Rust（推荐）

- **Pros**: 零成本抽象，所有权系统在编译期消除数据竞争，async/await + tokio 提供真正的异步并发，枚举 + 模式匹配天然适合建模复杂状态机，无 GC 的确定性内存管理适合长期运行
- **Cons**: 学习曲线陡峭，LLM 生态不如 Python 丰富，初期开发速度较慢

### Option 3: Go

- **Pros**: 良好的并发模型（goroutine），编译快，部署简单
- **Cons**: 类型系统表达力不足（无 sum type / ADT），泛型支持较新且有限，错误处理冗长，不适合建模复杂领域对象（人格参数空间、多层记忆、事件类型）

### Option 4: TypeScript (Node.js)

- **Pros**: 可全栈复用（与前端统一），生态大
- **Cons**: 单线程事件循环，类型安全仍弱于 Rust/Go，不适合计算密集型编排

## Decision

采用 **Rust (2024 Edition)** 作为仿真引擎的实现语言，以下逐一明确核心技术选型。

---

## 选型一：异步运行时 — tokio

**决策**：采用 `tokio` 作为唯一异步运行时。

```toml
tokio = { version = "1", features = ["full"] }
```

**理由**：

- ADR-0003 定义的两个 LLM 调用点（Agent 决策 + GM 仲裁）是典型的异步 I/O 任务
- 多 Agent 决策可以作为独立 tokio task 并发执行
- 仿真步进可以用 `tokio::time::interval` 精确控制
- `tokio::sync` 提供适合共享世界状态的并发原语（`RwLock`、`mpsc`、`watch`）
- Rust 异步生态的事实标准，axum / sqlx / qdrant-client 等核心依赖均基于 tokio

**并发模型映射**（回应 D4）：

```
仿真步进循环 (tokio::interval)
    ├── Agent 决策阶段 (tokio::JoinSet — 并行)
    │   ├── Agent A: 感知 → 记忆检索 → LLM 决策    ← tokio task
    │   ├── Agent B: 感知 → 记忆检索 → LLM 决策    ← tokio task
    │   └── Agent C: ...                            ← tokio task
    │
    ├── GM 仲裁阶段 (顺序或分组并行)
    │   └── 收集行为意图 → LLM 仲裁 → 结构化状态变更
    │
    ├── 状态更新阶段 (单线程写入 — 通过 mpsc channel)
    │   └── 世界状态管理器应用所有 state_changes
    │
    └── 广播阶段 (tokio::sync::broadcast)
        └── WebSocket 推送更新到前端
```

---

## 选型二：Web 框架 — axum

**决策**：采用 `axum` 作为 HTTP/WebSocket 框架。

```toml
axum = { version = "0.8", features = ["ws"] }
tower = "0.5"
tower-http = { version = "0.6", features = ["cors", "compression-gzip", "trace"] }
```

**比选**：

| 维度 | axum | actix-web |
|------|------|-----------|
| tokio 集成 | 原生（同团队） | 兼容但有自有运行时 |
| WebSocket | 内置，与 HTTP 路由统一 | 内置 |
| 中间件 | tower 生态（可跨 HTTP/非 HTTP 复用） | 自有系统 |
| 类型提取 | 编译期检查的 Extractor 模式 | 类似 |
| 生态趋势 | 2024-2026 新项目首选 | 成熟稳定 |

**选择 axum 的关键理由**：

1. 与 tokio 同源——仿真引擎深度依赖 tokio 的 task/channel/timer，axum 零摩擦集成
2. tower 中间件可复用于仿真引擎内部的 service 抽象（如 LLM 调用的重试/限流/追踪）
3. 内置 WebSocket 满足 ADR-0004 实时状态推送需求
4. Extractor 模式的编译期类型检查与项目对"类型即契约"的要求一致

---

## 选型三：关系型数据库 — PostgreSQL + sqlx

**决策**：采用 `PostgreSQL` 存储结构化世界状态，通过 `sqlx` 异步访问。

```toml
sqlx = { version = "0.8", features = [
    "runtime-tokio",
    "postgres",
    "json",
    "chrono",
    "uuid",
    "migrate"
] }
```

**PostgreSQL 存储的数据**（对应 ADR-0003 结构化世界状态管理器）：

| 数据类型 | 示例 | 查询特征 |
|---------|------|---------|
| Agent 配置与状态 | 人格参数 Θ、情绪、能力值 | 按 ID 查询 + 批量读取 |
| 关系矩阵 | Agent 间亲密度、群组归属 | 矩阵查询、图遍历 |
| 时间与课程表 | 仿真时钟、学期日历、课程安排 | 时间范围查询 |
| 事件日志 | 所有 SimulationEvent（含 narrative + state_changes）| 时间序列查询、全文搜索 |
| 仿真快照 | 世界状态快照（支持回放） | 按时间戳检索 |
| 实验配置 | Agent 群体配置方案、实验参数 | CRUD |

**比选**：

| 维度 | sqlx | sea-orm | diesel |
|------|------|---------|--------|
| 查询方式 | 原生 SQL + 编译期检查 | ORM 抽象 | DSL |
| 异步 | 原生 tokio | 原生 | 同步（wrapper） |
| 灵活性 | 最高 | 中等 | 中等 |
| JSONB | 完美支持 | 支持 | 有限 |

**选择 sqlx 的关键理由**：

1. ADR-0003 的世界状态管理器需要**复杂查询**（关系矩阵图遍历、事件时间序列、带条件的 Agent 状态聚合）——原生 SQL 的灵活性不可替代
2. 编译期 SQL 查询检查确保 schema 一致性——对 18 子模块的复杂数据模型是关键安全网
3. JSONB 支持存储半结构化数据（如 Game Master 的 state_changes 指令）
4. 原生 async + tokio 无缝配合

---

## 选型四：向量数据库 — Qdrant（回应 D3）

**决策**：采用 `Qdrant` 作为向量数据库，承载 M4 记忆系统的向量检索能力。

```toml
qdrant-client = "1"
```

**比选**：

| 维度 | Qdrant | pgvector | Milvus | Weaviate |
|------|--------|----------|--------|----------|
| 实现语言 | **Rust** | C (PG 扩展) | Go | Go |
| 部署复杂度 | 单二进制 / Docker | 依附 PostgreSQL | 较重 | 较重 |
| 过滤能力 | 丰富 payload 过滤 + 向量搜索组合 | SQL WHERE + 向量 | 有限 | GraphQL |
| Rust 客户端 | 官方 gRPC 客户端 | 通过 sqlx | 社区 | 社区 |
| 快照/备份 | 原生 Collection 快照 | PG 备份 | 有 | 有 |
| 性能（高维） | 优秀（HNSW + 量化） | 中等 | 优秀 | 良好 |
| 适合规模 | 百万级向量 | 十万级 | 亿级 | 千万级 |

**选择 Qdrant 的关键理由**：

### 理由一：与 M4 四层记忆架构的精确映射

ADR-0001 中 M4.1 定义了"感知→短期→长期→语义"的四层记忆，Qdrant 在其中的角色：

```
┌──────────────────────────────────────────────────────────────┐
│  Layer 4: 语义记忆 (Semantic Memory)                         │
│  "我不擅长和强势的人合作" "数学比语文更让我有成就感"           │
│  存储: Qdrant collection: {sim_id}_semantic                  │
│  特征: 高度抽象，由反思机制生成，长期保留                      │
├──────────────────────────────────────────────────────────────┤
│  Layer 3: 长期记忆 (Long-term Memory)                        │
│  "上学期和小红做数学项目时发生了争执"                          │
│  存储: Qdrant collection: {sim_id}_longterm                  │
│  特征: 从短期记忆筛选重要事件巩固而来，向量检索主要目标         │
├──────────────────────────────────────────────────────────────┤
│  Layer 2: 短期记忆 (Short-term Memory)                       │
│  "刚才老师布置了分组课题，我和小红被分到一组"                  │
│  存储: 内存缓冲 → 定期 flush 到 Qdrant {sim_id}_shortterm    │
│  特征: 近期事件，可能被遗忘或巩固为长期记忆                    │
├──────────────────────────────────────────────────────────────┤
│  Layer 1: 感知记忆 (Sensory Memory)                          │
│  "看到小红坐在旁边" "听到老师说要分组"                        │
│  存储: 仅内存（tokio task 局部变量），当前仿真步结束即丢弃     │
│  特征: 瞬时感知，不持久化                                     │
└──────────────────────────────────────────────────────────────┘
```

### 理由二：支持 Generative Agents 的三维检索评分

研究综合文档（第四章）明确了记忆检索的评分公式：

```
Score(memory) = α × relevance(query, memory)       // 语义相关性
              + β × recency(now, memory.timestamp)  // 时间衰减
              + γ × importance(memory)              // 重要性权重
```

Qdrant 精确支持这一公式：

| 评分维度 | Qdrant 实现方式 |
|---------|---------------|
| **relevance** | 向量余弦相似度（核心能力） |
| **recency** | payload 中存储 `timestamp`，检索时通过自定义 scoring 函数计算时间衰减 |
| **importance** | payload 中存储预计算的 `importance_score`，纳入评分 |
| **agent 隔离** | payload filter: `agent_id = "xiaoming"` |
| **层级过滤** | payload filter: `layer = "longterm"` |

### 理由三：Rust 原生亲和

Qdrant 本身用 Rust 实现，其官方客户端基于 gRPC（tonic），与 tokio 运行时完美兼容：

- gRPC 调用天然 async
- Rust 类型安全贯穿——从构建查询到解析结果
- 如未来需要嵌入式部署，Qdrant 的核心引擎可以作为 Rust library 直接集成

### 理由四：快照与仿真对齐

ADR-0003/0004 要求状态可快照、可回放。Qdrant 的 Collection Snapshot 功能可以与 PostgreSQL 的事务快照配合，实现完整的仿真状态保存：

```
仿真快照 = PostgreSQL 事务快照（世界状态）
         + Qdrant Collection 快照（记忆向量）
         + 内存状态序列化（当前仿真步上下文）
```

### 不选 pgvector 的理由

虽然 pgvector 作为 PostgreSQL 扩展可以减少基础设施，但对于本项目：

1. M4 是核心研究模块，记忆检索的性能和灵活性不能是"附带功能"
2. pgvector 的过滤 + 向量组合查询不如 Qdrant 灵活（需要复杂 SQL + CTE）
3. 随着仿真规模扩大（100+ Agent × 数千条记忆），pgvector 的性能衰减比专用向量数据库更早
4. Qdrant 的 Collection 概念天然支持按仿真实例隔离数据

---

## 选型五：LLM 集成策略（回应 D1）

**决策**：
- 采用 **API 调用优先**策略（MVP 阶段通过 HTTP 调用云端 LLM）
- **主力 LLM：DeepSeek**（deepseek-chat / deepseek-reasoner），通过 OpenAI 兼容 API 对接
- 通过自定义 `LlmProvider` trait 抽象厂商差异，**不使用 LangChain 或类似框架**
- 使用 `async-openai` 对接 DeepSeek 及其他 OpenAI 兼容 API，`reqwest` 对接非兼容厂商

```toml
async-openai = "0.27"
reqwest = { version = "0.12", features = ["json", "stream"] }
```

### DeepSeek 作为主力 LLM

DeepSeek API 完全兼容 OpenAI 的 Chat Completions 协议（`/v1/chat/completions`），`async-openai` 通过配置 base URL 即可直接对接：

```rust
let config = OpenAIConfig::new()
    .with_api_key(deepseek_api_key)
    .with_api_base("https://api.deepseek.com");

let client = Client::with_config(config);
```

**DeepSeek 兼容性矩阵**：

| 能力 | OpenAI 端点 | DeepSeek 支持 | AI School 用途 |
|------|-----------|-------------|---------------|
| Chat Completions | `/v1/chat/completions` | ✅ 完全兼容 | Agent 决策（调用点 #1）、GM 仲裁（调用点 #2） |
| Function Calling | `tools` 参数 | ✅ 兼容 | GM 结构化输出的备选通道 |
| JSON Mode | `response_format: { type: "json_object" }` | ✅ 兼容 | GM 结构化输出的主要方式 |
| Streaming | `stream: true` | ✅ 兼容 | 对话面板的流式回复（F4） |
| **Embeddings** | `/v1/embeddings` | ❌ **不提供** | 记忆向量化（M4.1）需要替代方案 |

### Embedding 供应商分离策略

DeepSeek 不提供 Embedding API，因此记忆向量化需要独立的 Embedding 供应商。这恰好验证了 `LlmProvider` trait 中将 `embed()` 独立定义的设计价值——补全和嵌入可以使用不同的供应商：

```
LLM 调用路由：
├── complete()              → DeepSeek API (deepseek-chat)
├── complete_structured()   → DeepSeek API (JSON Mode / Function Calling)
└── embed()                 → 独立 Embedding 供应商
```

**Embedding 供应商比选**：

| 方案 | 优势 | 劣势 | 适合阶段 |
|------|------|------|---------|
| **智谱 AI embedding-3** | 质量高，2048 维，OpenAI 兼容格式，国内访问稳定，成本低（¥0.5/1M tokens） | 依赖外部 API | **MVP 首选** |
| OpenAI text-embedding-3-small | 质量高，1536 维，async-openai 直接支持 | 有成本（$0.02/1M tokens），依赖海外 API | 备选 |
| **本地模型（BGE / GTE via Ollama）** | 零调用成本，无网络依赖 | 需部署 Ollama，初始化慢 | 大规模仿真阶段 |

**MVP 阶段推荐**：DeepSeek（补全）+ 智谱 AI（嵌入）的组合。两者都兼容 OpenAI API 格式，通过 `async-openai` 对接，只是 base URL 和 API key 不同，实现简洁：

```rust
pub struct DeepSeekProvider {
    chat_client: Client<OpenAIConfig>,       // base_url = "https://api.deepseek.com"
    embedding_client: Client<OpenAIConfig>,  // base_url = "https://open.bigmodel.cn/api/paas/v4"
}

#[async_trait]
impl LlmProvider for DeepSeekProvider {
    async fn complete(&self, request: &CompletionRequest) -> Result<CompletionResponse> {
        // → DeepSeek
        self.chat_client.chat().create(/*...*/).await
    }

    async fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        // → 智谱 AI Embedding
        self.embedding_client.embeddings().create(/*...*/).await
    }
}
```

未来如果 DeepSeek 推出 Embedding API，或者切换到本地 Embedding 模型，只需替换 `embedding_client` 的实现，不影响其他任何代码。

### 为什么不用 LangChain（或 Rust 版替代品）

| LangChain 能力 | AI School 需求 | 匹配度 |
|---------------|---------------|-------|
| 对话 Memory | M4 是认知科学级四层记忆架构 | ❌ 范式完全不同 |
| RAG 管线 | 记忆检索是 `relevance + recency + importance` 三维评分 | ❌ 远超通用 RAG |
| Agent (ReAct) | Agent 是人格驱动的认知循环，不是 tool-use | ❌ 范式完全不同 |
| Chain 编排 | 仿真循环是 `Agent→GM→State→Memory` 高度定制闭环 | ❌ 框架成为束缚 |
| LLM 调用抽象 | 一个 trait + 2-3 个 impl | ⚠️ 杀鸡用牛刀 |

**核心论点**：AI School 的三大核心领域——记忆系统、认知循环、人格演化——全部是研究级定制实现。LangChain 的抽象层恰好覆盖在这些需要完全掌控的地方。引入它等于在核心研究对象上加了一层不透明的封装。

**替代方案——薄封装 + 精选工具库**：

```rust
/// 定义在 ai-school-core 中的 LLM 抽象
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// 自然语言补全（Agent 决策、Game Master 仲裁）
    async fn complete(&self, request: &CompletionRequest) -> Result<CompletionResponse>;

    /// 结构化输出（Game Master 的 JSON Schema 约束输出）
    async fn complete_structured<T: DeserializeOwned>(
        &self,
        request: &CompletionRequest,
        schema: &serde_json::Value,
    ) -> Result<T>;

    /// 文本嵌入（记忆向量化）
    async fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>>;
}
```

这个 trait 的三个方法精确覆盖 ADR-0003 的两个 LLM 调用点 + M4 的嵌入需求：

| trait 方法 | 对应场景 | ADR 来源 |
|-----------|---------|---------|
| `complete` | Agent 决策（调用点 #1） | ADR-0003 |
| `complete_structured` | GM 仲裁（调用点 #2，输出 JSON Schema） | ADR-0003 |
| `embed` | 记忆向量化（写入 Qdrant） | ADR-0001 M4.1 |

---

## 选型六：Prompt 模板 — minijinja

**决策**：采用 `minijinja` 作为 Prompt 模板引擎。

```toml
minijinja = { version = "2", features = ["loader"] }
```

**理由**：

- Jinja2 语法是 Prompt 工程的事实标准（OpenAI、HuggingFace 均使用）
- `minijinja` 是纯 Rust 实现，无外部依赖，编译快
- 支持模板继承、过滤器、条件逻辑——满足复杂 Prompt 的组合需求
- 比 `tera`（另一个 Jinja2 实现）更轻量，API 更简洁

**使用场景**：

| 模板 | 输入 | 用于 |
|------|------|------|
| `agent_decision.j2` | 人格参数 Θ + 检索记忆 + 情境描述 | LLM 调用点 #1 |
| `game_master_arbitrate.j2` | Agent 行为意图 + 世界状态 + 规则约束 | LLM 调用点 #2 |
| `memory_reflection.j2` | 累积经历 + 当前人格 | M4 反思机制 |
| `consistency_check.j2` | 叙事文本 + 数据变更 | ADR-0004 一致性检测 |

---

## 选型七：序列化与 Schema 验证

**决策**：采用 serde 生态 + JSON Schema 验证。

```toml
serde = { version = "1", features = ["derive"] }
serde_json = "1"
schemars = "0.8"         # 从 Rust struct 自动生成 JSON Schema
jsonschema = "0.28"      # 运行时 JSON Schema 验证
```

**关键应用**：ADR-0003 要求 Game Master 的输出符合预定义 JSON Schema。`schemars` 可以从 Rust 类型定义自动生成 schema，`jsonschema` 在运行时验证 LLM 输出——两者配合实现"类型定义即 schema"的单一事实源：

```rust
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GameMasterOutput {
    pub event_type: EventType,
    pub intensity: f32,
    pub state_changes: Vec<StateChange>,
    pub narrative: String,
}

// 自动生成的 JSON Schema 既用于 LLM 提示约束，也用于输出验证
```

---

## 选型八：时间与标识符

```toml
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v7", "serde"] }
```

- `chrono`：仿真时间的精确表示（`DateTime<Utc>`）和时间运算
- `uuid` v7：带时间排序的唯一标识符——事件 ID 和记忆 ID 使用 v7 可以按时间自然排序，对应 ADR-0004 的时间对齐需求

---

## 选型九：可观测性 — tracing

```toml
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
```

**理由**：`tracing` 的 span 模型天然适合追踪仿真因果链：

```
[simulation tick=42]
  └── [agent_decision agent="小明"]
        ├── [memory_retrieval query="数学课分组" results=5]
        ├── [llm_call provider="deepseek" model="deepseek-chat" latency_ms=2340]
        └── [behavior_intent "我想按照我的方案来"]
  └── [game_master_arbitrate agents=["小明","小红"]]
        ├── [llm_call provider="deepseek" model="deepseek-chat" latency_ms=1820]
        └── [state_changes count=3]
  └── [state_update changes=3 conflicts=0]
  └── [ws_broadcast subscribers=2]
```

每一级 span 都可以附加结构化字段（agent_id、latency、event_type），为调试和性能分析提供全链路可见性。

---

## 选型十：错误处理

```toml
thiserror = "2"    # 库/模块级 typed error（各 crate 定义自己的 error enum）
anyhow = "1"       # 应用入口级 error propagation（API 层、CLI）
```

**策略**：

- 各业务 crate（agent、world、memory、engine）使用 `thiserror` 定义精确的错误枚举——调用者可以模式匹配处理特定错误
- API 层使用 `anyhow` 收集所有下游错误并统一转换为 HTTP 响应
- 错误链保持完整，不丢失上下文

---

## 选型十一：前端技术栈（回应 D7）

**决策**：前后端分离。前端采用 **React + TypeScript** 独立项目，通过 REST API + WebSocket 与 Rust 后端通信。

| 组件 | 选型 | 理由 |
|------|------|------|
| 框架 | React 19 + TypeScript | 组件生态最丰富，可视化库支持最好 |
| 构建 | Vite | 快速 HMR，开发体验好 |
| 2D 地图 | Pixi.js (Canvas) | Phase1 PRD 的 2D 校园地图，5-10 Agent 规模 Canvas 性能充足且交互灵活 |
| 数据可视化 | D3.js / ECharts | ADR-0004 双视图的数据面板（人格雷达图、情绪曲线、关系图谱） |
| 实时通信 | 原生 WebSocket | 接收 axum 后端的状态推送 |
| 状态管理 | Zustand | 轻量级，适合仿真状态的高频更新 |
| UI 组件 | shadcn/ui + Tailwind CSS | 现代设计系统，满足 PRD 对非技术用户可用性的要求 |

**不选 Rust WASM 的理由**：Phase1 PRD 要求快速迭代（2D 地图、对话面板、数据可视化），React 生态在这些场景的成熟度远超 Rust WASM 方案。

---

## 完整依赖清单

### 核心仿真引擎依赖

```toml
[workspace.dependencies]
# Async Runtime
tokio = { version = "1", features = ["full"] }

# Web Framework
axum = { version = "0.8", features = ["ws"] }
tower = "0.5"
tower-http = { version = "0.6", features = ["cors", "compression-gzip", "trace"] }

# Database — Relational
sqlx = { version = "0.8", features = [
    "runtime-tokio", "postgres", "json", "chrono", "uuid", "migrate"
] }

# Database — Vector
qdrant-client = "1"

# LLM Integration
async-openai = "0.27"
reqwest = { version = "0.12", features = ["json", "stream"] }

# Serialization & Validation
serde = { version = "1", features = ["derive"] }
serde_json = "1"
schemars = "0.8"
jsonschema = "0.28"

# Prompt Templates
minijinja = { version = "2", features = ["loader"] }

# Time & IDs
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v7", "serde"] }

# Observability
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }

# Error Handling
thiserror = "2"
anyhow = "1"

# Utilities
async-trait = "0.1"

# Config
config = "0.15"
dotenvy = "0.15"
```

### 测试依赖

```toml
[workspace.dependencies]
# Testing
mockall = "0.13"           # Mock trait（LlmProvider、MemoryStore 等）
wiremock = "0.6"           # HTTP mock（LLM API 测试，不依赖真实 API）
testcontainers = "0.23"    # Docker 容器化测试（PostgreSQL、Qdrant）
tokio-test = "0.4"         # async 测试工具
```

---

## Consequences

### Positive

- **并发安全由编译器保证**——共享世界状态的读写安全不依赖人的纪律，Rust 所有权系统杜绝数据竞争
- **类型系统作为模块契约**——18 子模块间的接口由类型定义精确描述，重构时编译器即时报错
- **性能天花板高**——LLM API 是唯一瓶颈，编排层本身接近零开销，为未来 100+ Agent 扩展预留空间
- **双数据库精确分工**——PostgreSQL 承载结构化查询（世界状态、事件日志），Qdrant 承载语义检索（记忆），各自发挥所长
- **研究级掌控力**——无框架遮蔽，记忆系统/认知循环/人格演化的每一行逻辑都是自研代码
- **Qdrant 的 Rust 原生亲和**——从编译到运行的完整 Rust 链路，减少跨语言边界的不确定性

### Negative

- **初期开发速度较慢**——Rust 的学习曲线和编译期检查会在前几周降低迭代速度
- **LLM SDK 生态较薄**——非 OpenAI 兼容 API（如 Anthropic）需要用 reqwest 自行封装
- **Embedding 供应商分离**——DeepSeek 不提供 Embedding API，需要额外的供应商（OpenAI 或本地模型），增加了一个外部依赖
- **双数据库运维**——PostgreSQL + Qdrant 两套系统需要独立部署、备份、监控
- **前后端分离的协调成本**——API 契约需要明确定义和同步维护

### Risks

- **Rust 学习曲线影响交付**
  - 缓解：P0 模块（M1.1、M1.3、M4.1）的核心类型定义先行，团队在有限范围内熟悉 Rust
  - 缓解：通过 ADR-0006 的 crate 划分，每个 crate 的复杂度可控

- **LLM 厂商 API 变动**
  - 缓解：`LlmProvider` trait 抽象隔离了厂商细节，切换只需添加新 impl

- **Qdrant 版本兼容性**
  - 缓解：使用 Qdrant 官方 Rust 客户端，跟随 Qdrant 版本节奏
  - 缓解：Collection 快照机制提供数据迁移安全网

- **async-openai crate 维护不活跃的可能**
  - 缓解：该 crate 本质是 HTTP 调用的类型封装，最坏情况下可切换到纯 reqwest 实现

## Implementation Notes

- Rust 版本：推荐 1.85+（2024 Edition），启用 `edition = "2024"`
- MSRV（最低支持版本）策略：跟随 workspace 中最新依赖的 MSRV
- CI 工具链：`rustfmt`（格式化）+ `clippy`（lint）+ `cargo test`（测试）+ `cargo deny`（依赖审计）
- 开发环境：推荐 `cargo-watch`（热重载）+ `bacon`（后台检查）+ `cargo-nextest`（并行测试）
- Docker Compose 开发环境：PostgreSQL + Qdrant + 前端 dev server

## Related Decisions

- [ADR-0001](0001-system-architecture-overview.md): 整体架构（本 ADR 回应其 D1, D3, D4, D7）
- [ADR-0002](0002-personality-evolution-mechanism.md): 人格演化闭环（LlmProvider trait 的设计约束来源）
- [ADR-0003](0003-world-system-llm-interaction.md): 双层架构（两个 LLM 调用点 → LlmProvider 的 complete / complete_structured）
- [ADR-0004](0004-narrative-data-dual-view.md): 叙事-数据双视图（Qdrant + PostgreSQL 的双视图数据基础）
- [ADR-0006](0006-project-structure.md): 项目工程结构（本 ADR 的选型落地为具体 crate 组织）

## References

- [tokio 官方文档](https://tokio.rs/)
- [axum 官方文档](https://docs.rs/axum/latest/axum/)
- [sqlx 官方文档](https://docs.rs/sqlx/latest/sqlx/)
- [Qdrant 官方文档](https://qdrant.tech/documentation/)
- [Qdrant Rust Client](https://github.com/qdrant/rust-client)
- [async-openai](https://github.com/64bit/async-openai)
- [DeepSeek API 文档](https://platform.deepseek.com/api-docs/) — OpenAI 兼容 API
- [minijinja](https://github.com/mitsuhiko/minijinja)
- Generative Agents (Park et al., 2023) — 记忆检索评分公式
- Concordia (Google DeepMind, 2023) — Game Master 机制
- AgentSociety (2025) — 10,000+ Agent 大规模仿真经验

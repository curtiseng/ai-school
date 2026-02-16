# AI School — 从零开始的环境配置与启动指南

本文档面向一台全新的 macOS / Linux 机器，记录从裸机到项目完整运行的每一步。

---

## 目录

1. [系统依赖](#1-系统依赖)
2. [Rust 工具链](#2-rust-工具链)
3. [Docker 与基础设施](#3-docker-与基础设施)
4. [克隆项目](#4-克隆项目)
5. [环境变量配置](#5-环境变量配置)
6. [编译与测试](#6-编译与测试)
7. [运行项目](#7-运行项目)
8. [项目结构速览](#8-项目结构速览)
9. [常用命令速查](#9-常用命令速查)
10. [常见问题](#10-常见问题)

---

## 1. 系统依赖

### macOS

```bash
# 安装 Homebrew（如果没有）
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# 安装基础工具
brew install git curl pkg-config openssl
```

### Ubuntu / Debian

```bash
sudo apt update
sudo apt install -y git curl build-essential pkg-config libssl-dev
```

---

## 2. Rust 工具链

### 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

按提示选择默认安装（选 1），安装完成后激活环境：

```bash
source "$HOME/.cargo/env"
```

### 验证安装

```bash
rustc --version   # 需要 >= 1.85
cargo --version
```

> 项目根目录的 `rust-toolchain.toml` 会自动管理工具链版本，首次编译时 rustup 会自动安装对应版本。

### 推荐开发工具（可选）

```bash
# 后台持续检查编译错误（类似 watch 模式）
cargo install bacon

# 更快的测试运行器（支持并行）
cargo install cargo-nextest

# 文件变更自动重编译
cargo install cargo-watch
```

---

## 3. Docker 与基础设施

项目依赖两个外部服务：**PostgreSQL**（结构化数据）和 **Qdrant**（向量记忆检索）。

### 安装 Docker

**macOS：**

```bash
brew install --cask docker
```

安装后打开 Docker Desktop 并确保 Docker Engine 正在运行。

**Linux：**

```bash
# 参照官方文档：https://docs.docker.com/engine/install/
curl -fsSL https://get.docker.com | sh
sudo systemctl start docker
sudo systemctl enable docker
sudo usermod -aG docker $USER  # 免 sudo（需重新登录生效）
```

### 启动基础设施

```bash
cd ai-school
docker compose up -d
```

这会启动：

| 服务 | 端口 | 用途 |
|------|------|------|
| PostgreSQL 17 | `15432` | 世界状态、事件日志、快照存储 |
| Qdrant v1.13 | `16333` (REST) / `16334` (gRPC) | 记忆向量检索 |

### 验证服务

```bash
# PostgreSQL
docker compose exec postgres pg_isready
# 预期输出: /var/run/postgresql:5432 - accepting connections

# Qdrant
curl http://localhost:16333/healthz
# 预期输出: 空白（HTTP 200）
```

### 停止 / 清理

```bash
# 停止服务（保留数据）
docker compose stop

# 停止并删除容器 + 数据
docker compose down -v
```

---

## 4. 克隆项目

```bash
git clone https://github.com/your-org/ai-school.git
cd ai-school
```

---

## 5. 环境变量配置

```bash
cp .env.example .env
```

编辑 `.env` 文件，填入你的 API Key：

```ini
# LLM Configuration — DeepSeek 用于 Chat 补全
DEEPSEEK_API_KEY=sk-xxxxxxxxxxxxxxxx        # ← 必填
DEEPSEEK_BASE_URL=https://api.deepseek.com
DEEPSEEK_MODEL=deepseek-chat

# Embedding Configuration — 智谱 AI 用于记忆向量化
ZHIPU_API_KEY=xxxxxxxxxxxxxxxx               # ← 必填
ZHIPU_EMBEDDING_BASE_URL=https://open.bigmodel.cn/api/paas/v4
ZHIPU_EMBEDDING_MODEL=embedding-3

# PostgreSQL（与 docker-compose.yml 一致即可）
DATABASE_URL=postgres://ai_school:dev_password@localhost:15432/ai_school

# Qdrant
QDRANT_URL=http://localhost:16333

# Server
API_HOST=0.0.0.0
API_PORT=3000

# Logging（开发环境推荐 debug 级别）
RUST_LOG=ai_school=debug,tower_http=debug
```

> **注意：** 当前 MVP 阶段默认使用 Mock LLM Provider，不填 API Key 也能运行 CLI 和 API 服务器（使用模拟数据）。如需真实 LLM 调用，必须填入有效的 Key。

### API Key 获取方式

| 服务 | 获取地址 | 用途 |
|------|---------|------|
| DeepSeek | https://platform.deepseek.com/ | Agent 决策 + GM 仲裁 |
| 智谱 AI | https://open.bigmodel.cn/ | 记忆向量化（Embedding） |

---

## 6. 编译与测试

### 首次编译

```bash
cargo build
```

> 首次编译需要下载并编译所有依赖，预计 2-5 分钟（取决于网络和机器性能）。后续增量编译通常在数秒内完成。

### 运行测试

```bash
cargo test
```

预期输出：所有 15 个测试通过。

### 代码质量检查

```bash
# 格式化
cargo fmt --check

# Lint 检查
cargo clippy -- -D warnings
```

---

## 7. 运行项目

### 方式一：CLI 工具（快速体验）

无需启动任何服务即可运行，使用 Mock LLM。

```bash
# 查看 Agent 人格分析
cargo run --bin ai-school-cli -- inspect --agents 5

# 运行批量仿真（5 个 Agent，50 步）
cargo run --bin ai-school-cli -- run --agents 5 --steps 50

# 运行仿真并导出数据到文件
cargo run --bin ai-school-cli -- run --agents 5 --steps 100 --output result.json
```

### 方式二：API 服务器

```bash
# 确保 Docker 基础设施已启动
docker compose up -d

# 启动 API 服务
cargo run --bin ai-school-api
```

服务启动后监听 `http://localhost:3000`。

#### API 端点一览

| 方法 | 路径 | 说明 |
|------|------|------|
| `GET` | `/api/simulation/status` | 获取仿真状态 |
| `POST` | `/api/simulation/start` | 启动仿真 |
| `POST` | `/api/simulation/stop` | 停止仿真 |
| `POST` | `/api/simulation/step` | 手动执行一步 |
| `PUT` | `/api/simulation/speed` | 设置仿真速度 |
| `GET` | `/api/agents` | 获取所有 Agent |
| `POST` | `/api/agents` | 创建 Agent |
| `POST` | `/api/agents/generate` | 批量生成随机 Agent |
| `GET` | `/api/agents/{id}` | 获取 Agent 详情 |
| `POST` | `/api/interventions/event` | 触发事件 |
| `GET` | `/api/analysis/snapshot` | 获取世界快照 |
| `GET` | `/api/analysis/events` | 获取事件日志 |
| `GET` | `/api/analysis/export` | 导出全量数据 |
| `WebSocket` | `/ws/simulation` | 实时仿真状态推送 |

#### 快速体验流程

```bash
# 1. 生成 5 个随机 Agent
curl -X POST http://localhost:3000/api/agents/generate \
  -H "Content-Type: application/json" \
  -d '{"count": 5}'

# 2. 查看 Agent 列表
curl http://localhost:3000/api/agents | python3 -m json.tool

# 3. 手动执行一步仿真
curl -X POST http://localhost:3000/api/simulation/step | python3 -m json.tool

# 4. 获取世界快照
curl http://localhost:3000/api/analysis/snapshot | python3 -m json.tool

# 5. 导出全量数据
curl http://localhost:3000/api/analysis/export > export.json
```

---

## 8. 项目结构速览

```
ai-school/
├── Cargo.toml                  # Workspace 根配置
├── rust-toolchain.toml         # Rust 版本锁定
├── docker-compose.yml          # PostgreSQL + Qdrant
├── .env.example                # 环境变量模板
│
├── crates/                     # 8 个 Rust Crate
│   ├── ai-school-core/         # 共享类型 + Trait 定义
│   ├── ai-school-llm/          # LLM 集成（DeepSeek + Mock）
│   ├── ai-school-agent/        # M1: 人格 + 认知 + 职业
│   ├── ai-school-world/        # M2: 校园 + 时间 + 课程 + 社交
│   ├── ai-school-memory/       # M4: 记忆存储/检索/反思/演变
│   ├── ai-school-engine/       # M3: 仿真循环 + GM + 干预
│   ├── ai-school-api/          # HTTP/WebSocket API 层
│   └── ai-school-cli/          # CLI 工具
│
├── prompts/                    # Prompt 模板（.j2 文件）
│   ├── agent/                  # Agent 决策 Prompt
│   ├── game_master/            # GM 仲裁 Prompt
│   ├── memory/                 # 反思 + 重要性评分
│   ├── consistency/            # 一致性检测
│   └── system/                 # 系统角色定义
│
├── migrations/                 # PostgreSQL 迁移脚本
├── tests/                      # 集成测试
│
└── docs/                       # 文档
    ├── SETUP.md                # ← 本文档
    ├── adr/                    # 架构决策记录
    ├── prd/                    # 产品需求文档
    └── research/               # 研究论文综述
```

### Crate 依赖关系

```
core → llm, agent, world
         ↓
       memory (依赖 core + llm)
         ↓
       engine (依赖全部)
         ↓
     api / cli (依赖 engine)
```

---

## 9. 常用命令速查

```bash
# 编译
cargo build                          # Debug 编译
cargo build --release                # Release 编译

# 测试
cargo test                           # 运行所有测试
cargo test -p ai-school-agent        # 只测某个 crate
cargo nextest run                    # 并行测试（需安装 cargo-nextest）

# 运行
cargo run --bin ai-school-api        # 启动 API 服务
cargo run --bin ai-school-cli -- -h  # CLI 帮助

# 代码质量
cargo fmt                            # 格式化代码
cargo clippy                         # Lint 检查
cargo doc --open                     # 生成并打开文档

# Docker
docker compose up -d                 # 启动基础设施
docker compose down                  # 停止基础设施
docker compose logs -f postgres      # 查看 PG 日志
docker compose logs -f qdrant        # 查看 Qdrant 日志

# 持续开发
bacon                                # 后台持续编译检查
cargo watch -x test                  # 文件变更自动测试
```

---

## 10. 常见问题

### Q: 编译报错 `linker 'cc' not found`

缺少 C 编译器。macOS 运行 `xcode-select --install`；Linux 运行 `sudo apt install build-essential`。

### Q: Docker 端口冲突

如果 15432 或 16333 端口已被占用，修改 `docker-compose.yml` 中的端口映射，并同步更新 `.env` 文件。

### Q: `cargo build` 非常慢

首次编译需要下载和编译所有依赖（约 200+ crate），这是正常的。后续增量编译会快很多。建议：
- 确保网络畅通（依赖从 crates.io 下载）
- 如在中国大陆，配置 crates.io 镜像源（在 `~/.cargo/config.toml` 中添加）

### Q: 如何切换到真实 LLM？

当前默认使用 Mock Provider。要切换到真实 LLM，需要在 `ai-school-api` 的 `main.rs` 中将 `MockLlmProvider` 替换为 `DeepSeekProvider`，并确保 `.env` 中的 API Key 有效。

### Q: Qdrant 连接失败

确认 Qdrant 容器正在运行：`docker compose ps`。如果容器未启动，检查 Docker Desktop 是否在运行。当前 MVP 阶段使用 `InMemoryStore`，不依赖 Qdrant 也能运行。

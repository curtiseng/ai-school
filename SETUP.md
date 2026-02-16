# AI School — 从零开始的环境配置与启动指南

本文档面向一台全新的 macOS / Linux 机器，记录从裸机到项目完整运行的每一步。

---

## 目录

1. [系统依赖](#1-系统依赖)
2. [Rust 工具链](#2-rust-工具链)
3. [Node.js 工具链](#3-nodejs-工具链)
4. [Docker 与基础设施](#4-docker-与基础设施)
5. [克隆项目](#5-克隆项目)
6. [环境变量配置](#6-环境变量配置)
7. [编译与构建](#7-编译与构建)
8. [运行项目](#8-运行项目)
9. [项目结构速览](#9-项目结构速览)
10. [常用命令速查](#10-常用命令速查)
11. [常见问题](#11-常见问题)

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
cargo install bacon           # 后台持续检查编译错误
cargo install cargo-nextest   # 更快的并行测试运行器
cargo install cargo-watch     # 文件变更自动重编译
```

---

## 3. Node.js 工具链

前端 Web UI 基于 React + Vite 构建，需要 Node.js 环境。

### 安装 Node.js

**macOS：**

```bash
brew install node
```

**Linux：**

```bash
curl -fsSL https://deb.nodesource.com/setup_22.x | sudo -E bash -
sudo apt install -y nodejs
```

### 验证安装

```bash
node --version   # 需要 >= 18
npm --version
```

---

## 4. Docker 与基础设施

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

## 5. 克隆项目

```bash
git clone https://github.com/your-org/ai-school.git
cd ai-school
```

---

## 6. 环境变量配置

```bash
cp .env.example .env
```

编辑 `.env` 文件，填入你的 API Key：

```ini
# LLM Configuration — DeepSeek 用于 Agent 决策与 GM 仲裁
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

> **注意：** API 服务器默认使用真实 LLM（DeepSeek 补全 + 智谱嵌入）。必须填入有效的 API Key 才能使 Agent 决策和对话功能正常工作。CLI 工具仍使用 Mock LLM，无需 Key。

### API Key 获取方式

| 服务 | 获取地址 | 用途 |
|------|---------|------|
| DeepSeek | https://platform.deepseek.com/ | Agent 决策 + GM 仲裁 + 角色对话 |
| 智谱 AI | https://open.bigmodel.cn/ | 记忆向量化（Embedding） |

---

## 7. 编译与构建

### 后端编译

```bash
cargo build
```

> 首次编译需要下载并编译所有依赖，预计 2-5 分钟。后续增量编译通常在数秒内完成。

### 前端构建

```bash
cd frontend
npm install
npx vite build
cd ..
```

> 前端构建产物输出到 `frontend/dist/`，API 服务器会自动提供静态文件服务。

### 运行测试

```bash
cargo test
```

### 代码质量检查

```bash
cargo fmt --check     # 格式化检查
cargo clippy          # Lint 检查
```

---

## 8. 运行项目

### 方式一：Web UI（推荐）

**一体化部署** — 后端 API 服务器同时提供前端界面和 API：

```bash
# 1. 确保前端已构建（见第 7 步）
# 2. 启动 Docker 基础设施
docker compose up -d

# 3. 启动服务
cargo run --bin ai-school-api
```

打开浏览器访问 **http://localhost:3000**，即可看到 2D 校园仿真界面。

**界面功能：**
- **2D 校园地图**：实时展示 Agent 在校园中的位置和活动
- **Agent 列表**：查看所有 Agent 的人格、状态
- **仿真控制**：启动/暂停/步进/速度调节
- **详情面板**：查看 Agent 人格、情绪、能力详情
- **对话面板**：以老师/校长/辅导员身份与 Agent 对话（真实 LLM 驱动）
- **干预工具**：调整环境参数、触发预设事件
- **数据导出**：导出全量仿真数据（JSON）

**前端开发模式**（支持热更新）：

```bash
# 终端 1：启动后端
cargo run --bin ai-school-api

# 终端 2：启动 Vite dev server（自动代理 API）
cd frontend && npm run dev
```

访问 http://localhost:5173 即可，前端修改实时生效。

### 方式二：CLI 工具（快速实验）

无需启动任何服务即可运行，使用 Mock LLM。

```bash
# 查看 Agent 人格分析
cargo run --bin ai-school-cli -- inspect --agents 5

# 运行批量仿真（5 个 Agent，50 步）
cargo run --bin ai-school-cli -- run --agents 5 --steps 50

# 运行仿真并导出数据到文件
cargo run --bin ai-school-cli -- run --agents 5 --steps 100 --output result.json
```

### 方式三：纯 API 调用

```bash
# 启动 API 服务
cargo run --bin ai-school-api
```

#### API 端点一览

| 方法 | 路径 | 说明 |
|------|------|------|
| `GET` | `/api/simulation/status` | 获取仿真状态 |
| `POST` | `/api/simulation/start` | 启动仿真 |
| `POST` | `/api/simulation/stop` | 停止仿真 |
| `POST` | `/api/simulation/step` | 手动执行一步 |
| `PUT` | `/api/simulation/speed` | 设置仿真速度 |
| `PUT` | `/api/simulation/params` | 调整环境参数 |
| `GET` | `/api/agents` | 获取所有 Agent |
| `POST` | `/api/agents` | 创建 Agent |
| `POST` | `/api/agents/generate` | 批量生成随机 Agent |
| `GET` | `/api/agents/{id}` | 获取 Agent 详情 |
| `POST` | `/api/agents/{id}/chat` | 与 Agent 对话（LLM 驱动） |
| `POST` | `/api/interventions/event` | 触发事件 |
| `GET` | `/api/analysis/snapshot` | 获取世界快照 |
| `GET` | `/api/analysis/events` | 获取事件日志 |
| `GET` | `/api/analysis/export` | 导出全量数据 |
| `WebSocket` | `/ws/simulation` | 实时仿真状态推送（JSON） |

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

# 4. 与 Agent 对话（需要有效的 DEEPSEEK_API_KEY）
curl -X POST http://localhost:3000/api/agents/{id}/chat \
  -H "Content-Type: application/json" \
  -d '{"role": "teacher", "message": "你最近学习怎么样？"}'

# 5. 导出全量数据
curl http://localhost:3000/api/analysis/export > export.json
```

---

## 9. 项目结构速览

```
ai-school/
├── Cargo.toml                  # Workspace 根配置
├── rust-toolchain.toml         # Rust 版本锁定
├── docker-compose.yml          # PostgreSQL + Qdrant
├── .env.example                # 环境变量模板
│
├── crates/                     # 8 个 Rust Crate（后端）
│   ├── ai-school-core/         # 共享类型 + Trait 定义
│   ├── ai-school-llm/          # LLM 集成（DeepSeek + 智谱 + Mock）
│   ├── ai-school-agent/        # M1: 人格 + 认知 + 职业
│   ├── ai-school-world/        # M2: 校园 + 时间 + 课程 + 社交
│   ├── ai-school-memory/       # M4: 记忆存储/检索/反思/演变
│   ├── ai-school-engine/       # M3: 仿真循环 + GM + 干预
│   ├── ai-school-api/          # HTTP/WebSocket API + 静态文件服务
│   └── ai-school-cli/          # CLI 工具
│
├── frontend/                   # Web UI（React + Vite + Tailwind）
│   ├── src/
│   │   ├── components/         # TopBar, AgentList, CampusMap, DetailPanel, BottomBar
│   │   ├── stores/             # zustand 状态管理
│   │   ├── api/                # REST + WebSocket 客户端
│   │   └── types/              # TypeScript 类型定义
│   ├── dist/                   # 构建产物（git ignored）
│   └── package.json
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

## 10. 常用命令速查

```bash
# === 后端 ===
cargo build                          # Debug 编译
cargo build --release                # Release 编译
cargo test                           # 运行所有测试
cargo test -p ai-school-agent        # 只测某个 crate
cargo run --bin ai-school-api        # 启动 API 服务（含 Web UI）
cargo run --bin ai-school-cli -- -h  # CLI 帮助
cargo fmt                            # 格式化代码
cargo clippy                         # Lint 检查
cargo doc --open                     # 生成并打开文档

# === 前端 ===
cd frontend
npm install                          # 安装依赖
npm run dev                          # 开发模式（热更新，端口 5173）
npx vite build                       # 生产构建（输出到 dist/）
npx tsc --noEmit                     # TypeScript 类型检查

# === Docker ===
docker compose up -d                 # 启动基础设施
docker compose down                  # 停止基础设施
docker compose logs -f postgres      # 查看 PG 日志
docker compose logs -f qdrant        # 查看 Qdrant 日志

# === 持续开发 ===
bacon                                # 后台持续编译检查
cargo watch -x test                  # 文件变更自动测试
```

---

## 11. 常见问题

### Q: 编译报错 `linker 'cc' not found`

缺少 C 编译器。macOS 运行 `xcode-select --install`；Linux 运行 `sudo apt install build-essential`。

### Q: Docker 端口冲突

如果 15432 或 16333 端口已被占用，修改 `docker-compose.yml` 中的端口映射，并同步更新 `.env` 文件。

### Q: `cargo build` 非常慢

首次编译需要下载和编译所有依赖（约 200+ crate），这是正常的。后续增量编译会快很多。建议：
- 确保网络畅通（依赖从 crates.io 下载）
- 如在中国大陆，配置 crates.io 镜像源（在 `~/.cargo/config.toml` 中添加）

### Q: LLM 调用失败

API 服务器默认使用真实 LLM。确保 `.env` 中的 `DEEPSEEK_API_KEY` 和 `ZHIPU_API_KEY` 有效。可通过以下方式验证：

```bash
# 启动服务后，生成 Agent 并执行一步仿真
curl -X POST http://localhost:3000/api/agents/generate -H "Content-Type: application/json" -d '{"count": 3}'
curl -X POST http://localhost:3000/api/simulation/step | python3 -m json.tool
```

如果看到 `"success": true`，说明 LLM 调用正常。

### Q: 前端页面空白

确保已执行前端构建：`cd frontend && npm install && npx vite build`。构建后重启 API 服务器即可在 http://localhost:3000 看到界面。

### Q: Qdrant 连接失败

确认 Qdrant 容器正在运行：`docker compose ps`。如果容器未启动，检查 Docker Desktop 是否在运行。当前 MVP 阶段使用 `InMemoryStore`，不依赖 Qdrant 也能运行。

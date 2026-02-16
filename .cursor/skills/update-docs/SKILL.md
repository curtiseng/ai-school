---
name: update-docs
description: 基于 arrow-simple 源码更新 astra-faber-docs 文档站点。更新主页产品介绍（Vera、Arca、Anima、Cosmo），维护 SDK 使用文档。Use when the user asks to update documentation, sync docs with code, or generate product pages.
---

# 更新 AstraFaber 文档

## 概览

文档项目路径：`/Users/curtis/workspace/astra-faber/astra-faber-docs`
源码项目路径：`/Users/curtis/workspace/github/arrow-simple`

文档框架：VitePress 1.6.3，语言 zh-CN，品牌色 #7c3aed（紫色系）。

## 产品线定义

| 产品 | 定位 | 状态 | 面向用户描述 |
|------|------|------|------------|
| **Vera** | 物模型平台 | 已发布 | 设备建模与管理、属性同步、设备孪生 |
| **Arca** | 文件服务 | 已发布 | 文件上传与管理、MCAP 录制、日志存储 |
| **Anima** | 数字孪生引擎 | 建设中 | 3D 可视化、实时数字孪生、场景回放 |
| **Cosmo** | 仿真平台 | 建设中 | 物理仿真、场景模拟、自动化测试 |

**重要原则：**
- 不暴露内部技术细节（Raft、VNode、WAL、MemTable、DataFusion 等）
- 不暴露内部 crate 结构（store-types、store-meta 等）
- 侧重用户价值和使用场景
- SDK 使用文档可以详细，包含完整代码示例
- Anima 和 Cosmo 标注"即将推出"

## 工作流程

### Step 1: 读取源码获取最新信息

从源码项目读取以下内容，了解最新变化：

```
# 产品架构（了解产品线和定位）
docs/PLATFORM_ARCHITECTURE.md

# astra-faber SDK 源码（统一 SDK，按 feature 拆分 vera / arca 模块）
sdks/astra-faber/
├── Cargo.toml             → features: vera, arca（默认全部启用）
├── vera/                  → ThingsClient, sync, schema, conflict
└── arca/                  → FileClient, McapRecorder, Uploader, RotatePolicy

# Vera API Proto（了解接口定义）
crates/vera-api/proto/

# Arca API Proto
crates/arca-api/proto/

# 物模型定义
crates/vera-things/        → ThingModel, Slot, Property
```

### Step 2: 读取现有文档

```
index.md                   # 主页
.vitepress/config.ts       # 导航和侧边栏配置
sdk/                       # SDK 文档目录（按 vera / arca 模块组织）
```

### Step 3: 更新主页 (index.md)

主页结构应包含：

1. **Hero 区域** — AstraFaber 平台简介（不只是数据库，而是完整的 IoT 平台）
2. **产品卡片** — 四个产品的介绍卡片（Vera、Arca、Anima、Cosmo）
3. **Features** — 平台核心能力（保持现有 6 个 feature 但调整措辞，减少技术术语）
4. **快速开始** — 保留简洁的代码示例

#### 主页产品介绍写作指南

**Vera（物模型平台）：**
- 强调：设备管理、物模型定义、属性同步、设备孪生
- 可提及：支持海量设备接入、毫秒级属性同步、离线缓存与自动恢复
- 不要提：Raft、VNode、store-engine、WAL

**Arca（文件服务）：**
- 强调：文件上传管理、机器人日志录制、MCAP 格式支持
- 可提及：预签名上传、自动轮转、断点续传
- 不要提：SealHandler、GcCoordinator、OpenDAL 细节

**Anima（数字孪生引擎）— 即将推出：**
- 强调：3D 可视化、实时数字孪生、场景记录与回放
- 可提及：支持 WASM 在浏览器运行
- 标注"Coming Soon"状态

**Cosmo（仿真平台）— 即将推出：**
- 强调：物理仿真、自动化测试、场景模拟
- 可提及：与 Anima 深度集成
- 标注"Coming Soon"状态

#### 主页产品卡片 HTML 模板

在 features 之前或之后插入产品卡片区域，使用自定义 HTML + CSS，与现有 `custom.css` 风格一致（紫色渐变、圆角卡片、hover 动画）。

### Step 4: 更新 SDK 文档

SDK 统一为 `astra-faber`，按 Vera 和 Arca 两个模块拆分文档。SDK 文档是核心内容，可以详细。

#### 文档目录结构

```
sdk/
├── index.md               # astra-faber SDK 总览（安装、feature 配置、架构简介）
├── vera/
│   ├── client.md          # Vera Client（数据读写、Schema、表/设备操作）
│   └── things.md          # Vera Things（设备孪生、属性同步、离线队列）
└── arca/
    ├── file.md            # Arca File（文件上传、预签名 URL）
    └── recorder.md        # Arca Recorder（MCAP 录制、自动轮转、上传）
```

#### SDK 总览页 (sdk/index.md)

介绍 astra-faber SDK 整体：
- 一句话定位：面向边缘设备的统一 SDK，集成物模型同步与文件管理
- 安装方式：`Cargo.toml` 依赖配置，说明 feature flags
- 模块一览：vera 模块做什么、arca 模块做什么
- 快速导航：链接到各子模块文档

```toml
# 全部功能（默认）
[dependencies]
astra-faber = { version = "x.x", features = ["vera", "arca"] }

# 仅物模型
astra-faber = { version = "x.x", features = ["vera"] }

# 仅文件服务
astra-faber = { version = "x.x", features = ["arca"] }
```

#### 各模块文档模板

```markdown
# [模块名称]

[一句话描述]

## 快速上手

[最小可运行代码示例]

## 核心 API

### [主要类/结构体]
[API 说明 + 代码示例]

## 进阶用法

[高级功能、配置选项]

## 错误处理

[Error 类型说明]
```

#### Vera 模块文档要点

**vera/client.md** — 数据读写客户端：
- Client 连接与配置
- SchemaBuilder 定义数据结构
- 表操作（创建、插入、流式插入）
- 设备操作（创建设备、设备数据写入）
- 查询（最新值、范围查询、批量查询、流式查询）
- 类型系统（int32、string、timestamp、enum8 等）
- HLC 混合逻辑时钟

**vera/things.md** — 设备孪生（Things）：
- ThingsClient 配置与连接
- 属性上报（report / report_batch）
- 期望值读取（get_desired）
- Slot 操作（report_slot / get_slot_desired）
- Schema 校验
- 离线队列与自动同步
- 文件持久化（可选 feature）
- 冲突解决策略

#### Arca 模块文档要点

**arca/file.md** — 文件上传：
- FileClient 配置与连接
- 文件上传流程（获取预签名 URL → 上传 → 确认）
- 文件元数据管理
- 上传策略配置

**arca/recorder.md** — MCAP 录制：
- McapRecorder 配置与初始化
- 录制启停控制
- 自动轮转策略（RotatePolicy：按大小/时间）
- 自动上传到 Arca
- Uploader 配置

#### 从源码同步 SDK 文档

1. 读取 `sdks/astra-faber/Cargo.toml` 获取 feature 定义和版本
2. 读取 `sdks/astra-faber/src/lib.rs` 和各模块 `mod.rs` 获取公开 API
3. 读取各模块的 doc comment 获取说明
4. 读取 `examples/` 目录获取用法示例
5. 对比现有文档，更新新增/变更的 API

### Step 5: 更新 VitePress 配置

更新 `.vitepress/config.ts`，确保导航和侧边栏包含所有产品和 SDK 页面：

```typescript
nav: [
  { text: '首页', link: '/' },
  {
    text: '产品',
    items: [
      { text: 'Vera 物模型', link: '/products/vera' },
      { text: 'Arca 文件服务', link: '/products/arca' },
      { text: 'Anima 数字孪生', link: '/products/anima' },
      { text: 'Cosmo 仿真', link: '/products/cosmo' },
    ],
  },
  {
    text: 'SDK',
    items: [
      { text: '总览', link: '/sdk/' },
      { text: 'Vera Client', link: '/sdk/vera/client' },
      { text: 'Vera Things', link: '/sdk/vera/things' },
      { text: 'Arca File', link: '/sdk/arca/file' },
      { text: 'Arca Recorder', link: '/sdk/arca/recorder' },
    ],
  },
],

sidebar: {
  '/sdk/': [
    {
      text: 'astra-faber SDK',
      items: [
        { text: '总览', link: '/sdk/' },
      ],
    },
    {
      text: 'Vera 模块',
      items: [
        { text: 'Client 数据读写', link: '/sdk/vera/client' },
        { text: 'Things 万物模型', link: '/sdk/vera/things' },
      ],
    },
    {
      text: 'Arca 模块',
      items: [
        { text: 'File 文件上传', link: '/sdk/arca/file' },
        { text: 'Recorder MCAP 录制', link: '/sdk/arca/recorder' },
      ],
    },
  ],
},
```

### Step 6: 添加产品详情页（可选）

如需要独立产品页，创建 `products/` 目录：

```
products/
├── vera.md            # Vera 产品介绍 + 架构概览 + 快速开始
├── arca.md            # Arca 产品介绍 + 使用场景
├── anima.md           # Anima 预览页（Coming Soon）
└── cosmo.md           # Cosmo 预览页（Coming Soon）
```

产品页侧重使用场景和价值，不涉及内部实现。

## 写作风格

- 语言：简体中文
- 语气：专业但友好，面向开发者
- 代码示例：完整可运行，包含必要 import
- 避免：过度使用形容词、营销腔调
- 技术术语：使用 gRPC、SDK、API 等开发者熟悉的词汇，但不使用内部架构术语
- Anima/Cosmo 相关内容用虚线框或淡色样式表示"建设中"

## 自定义 CSS 约定

- 品牌渐变：`linear-gradient(135deg, #7c3aed, #2563eb)`
- 卡片圆角：`12px`
- hover 效果：`translateY(-4px)` + `box-shadow`
- 暗色模式：使用 `.dark` 选择器覆盖
- 响应式：640px 和 960px 断点

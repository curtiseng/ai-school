# Generative Agents 论文研究综述

> 最后更新：2026-02-16
>
> 本文档系统梳理了 Generative Agents（生成式智能体）领域的核心论文，涵盖奠基性工作、架构演进、记忆系统、涌现行为、框架平台、评估基准等方向。

---

## 目录

- [一、奠基性论文](#一奠基性论文)
- [二、大规模仿真与后续工作](#二大规模仿真与后续工作)
- [三、框架与平台](#三框架与平台)
- [四、记忆架构前沿](#四记忆架构前沿)
- [五、涌现行为研究](#五涌现行为研究)
- [六、综述论文](#六综述论文)
- [七、评估与基准](#七评估与基准)
- [八、技术演进脉络](#八技术演进脉络)
- [九、论文速查索引](#九论文速查索引)

---

## 一、奠基性论文

### 1.1 Generative Agents: Interactive Simulacra of Human Behavior

| 字段 | 内容 |
|------|------|
| **作者** | Joon Sung Park, Joseph C. O'Brien, Carrie Jun Cai, Meredith Ringel Morris, Percy Liang, Michael S. Bernstein |
| **机构** | Stanford University, Google Research |
| **发表** | UIST 2023 (ACM Symposium on User Interface Software and Technology) |
| **日期** | 2023年4月 (arXiv), 2023年10月 (会议) |
| **链接** | [arXiv:2304.03442](https://arxiv.org/abs/2304.03442) · [ACM DL](https://dl.acm.org/doi/10.1145/3586183.3606763) · [OpenReview](https://openreview.net/forum?id=9hj38qPQAt) · [GitHub (20k+ Stars)](https://github.com/joonspk-research/generative_agents) |

**核心贡献：** 这是 Generative Agents 领域的**开山之作**，首次提出了基于大语言模型（LLM）的生成式智能体架构，能够模拟可信的人类行为。

**架构设计 — 三大核心模块：**

| 模块 | 功能 | 机制 |
|------|------|------|
| **观察 (Observation)** | 感知环境信息 | 将环境事件转化为自然语言记忆流 |
| **规划 (Planning)** | 制定行动计划 | 基于记忆生成层次化的日程计划（日→小时→分钟） |
| **反思 (Reflection)** | 高层次推理 | 周期性综合记忆，生成抽象认知（如"我重视与家人的时间"） |

**记忆系统：**

- 所有经历以自然语言存储在**记忆流 (Memory Stream)** 中
- 检索基于三个权重：
  - **时效性 (Recency)** — 近期事件权重更高
  - **重要性 (Importance)** — LLM 对事件打分（1-10）
  - **相关性 (Relevance)** — 与当前情境的语义相似度
- 当重要性分数累积超过阈值时触发**反思 (Reflection)** 机制，生成更高层次的认知

**实验设置：**

- 在名为 **Smallville** 的虚拟小镇中部署 **25 个智能体**
- 环境灵感来自 The Sims，用户可通过自然语言与智能体交互
- 仅给一个智能体"想办一场情人节派对"的初始指令，智能体们便**自主地**：
  - 传播邀请
  - 结识新朋友
  - 约会
  - 协调出席时间
- 展现了**涌现性社会行为**

**消融实验结果：** 观察、规划和反思三个组件各自对可信行为有**关键贡献**，移除任一组件都会显著降低行为可信度。

**影响力：** 开启了整个 LLM 驱动的智能体仿真研究方向，GitHub 仓库获得 20,000+ Stars，被引次数极高。

---

## 二、大规模仿真与后续工作

### 2.1 Generative Agent Simulations of 1,000 People

| 字段 | 内容 |
|------|------|
| **作者** | Joon Sung Park, Carolyn Q. Zou, Aaron Shaw, Benjamin Mako Hill, Carrie Cai, Meredith Ringel Morris, Robb Willer, Percy Liang, Michael S. Bernstein |
| **机构** | Stanford University, Google DeepMind, Northwestern University |
| **日期** | 2024年11月 |
| **链接** | [arXiv:2411.10109](https://arxiv.org/abs/2411.10109) · [Papers With Code](https://paperswithcode.com/paper/generative-agent-simulations-of-1000-people) · [GitHub](https://github.com/joonspk-research/genagents) |

**核心突破：** 从"虚构角色仿真"迈向"真实个体仿真"。

**方法：**
- 基于对 **1,052 名真实个体** 的深度访谈（总计 2,000+ 小时），构建每个人的生成式智能体
- 应用 LLM 处理定性访谈数据，生成个体化的智能体架构

**关键发现：**
- 智能体在 General Social Survey 上的回答与真人一致率达 **85%**
- 该准确率与参与者两周后自己回答的一致率**持平**（人类自身的测试-重测信度）
- 在人格特质预测和实验复现中表现同样出色
- **关键创新**：相比仅使用人口统计描述的智能体，该架构**显著降低了跨种族和意识形态群体的准确率偏差**

**数据与 API：**
- 提供基于 2,000 小时访谈的 1,000 人智能体（受限 API）
- 开源人口统计智能体库（3,000+ 智能体）
- 聚合数据开放访问，个体数据需伦理审查

**意义：** 为政策制定和社会科学研究提供了"数字孪生"工具。

### 2.2 AgentSociety: Large-Scale Simulation of LLM-Driven Generative Agents

| 字段 | 内容 |
|------|------|
| **作者** | （多作者） |
| **日期** | 2025年2月 |
| **链接** | [arXiv:2502.08691](https://arxiv.org/abs/2502.08691) |

**规模突破：**
- **10,000+** LLM 驱动的智能体
- **500 万次**交互
- 测试场景：
  - 社会极化
  - 虚假信息传播
  - 全民基本收入 (UBI) 政策影响
  - 飓风等外部冲击
- 仿真结果与**真实世界实验结论高度一致**，验证了其作为社会科学研究平台的潜力

---

## 三、框架与平台

### 3.1 Concordia: Generative Agent-Based Modeling

| 字段 | 内容 |
|------|------|
| **作者** | Alexander Sasha Vezhnevets 等 |
| **机构** | Google DeepMind, Google Research |
| **日期** | 2023年12月 |
| **链接** | [arXiv:2312.03664](https://arxiv.org/abs/2312.03664) · [Google DeepMind](https://deepmind.google/research/publications/64717/) · [GitHub (1.2k+ Stars)](https://github.com/google-deepmind/concordia) |

**核心设计：**
- 构建 **Generative Agent-Based Models (GABMs)** 的通用框架
- 基于两个基本操作：**LLM 调用** + **联想记忆检索**
- 使用灵活的组件系统 (Component System) 在两者间进行中介

**Game Master (GM) 机制：**
- 借鉴桌游中的"游戏主持人"概念
- GM 负责模拟智能体交互的环境
- 智能体以自然语言描述意图，GM 翻译为具体实现：
  - 物理世界 → 验证物理合理性，描述效果
  - 数字环境 → 处理 API 调用（日历、邮件、搜索等）

**应用场景：**
- 合成用户研究
- 数据生成与服务评估
- 序贯社会博弈实验

**后续发展：**
- 2024 年 **NeurIPS Concordia Contest** — 挑战开发具有合作智能的语言智能体
- 涉及谈判、互惠、声誉管理、惩罚等社会博弈场景
- 2025 年研究发现 LLM 智能体在说服和规范执行任务上存在显著泛化差距

---

## 四、记忆架构前沿

### 4.1 A-MEM: Agentic Memory for LLM Agents

| 字段 | 内容 |
|------|------|
| **作者** | Wujiang Xu, Yongfeng Zhang 等 |
| **日期** | 2025年2月 |
| **发表** | NeurIPS 2025 Poster |
| **链接** | [arXiv:2502.12110](https://arxiv.org/abs/2502.12110) · [OpenReview](https://openreview.net/forum?id=FiM0M8gcct) · [GitHub](https://github.com/agiresearch/A-mem) |

**核心创新：** 借鉴 **Zettelkasten（卡片笔记法）** 原则，动态组织互联知识网络。

**工作流程：**
1. 新记忆添加时，生成包含上下文描述、关键词、标签的**结构化笔记**
2. 分析历史记忆，识别**有意义的连接**
3. 在相似记忆之间建立链接
4. **记忆演化** — 新记忆可触发对已有记忆属性的更新，使网络持续精炼理解

**结果：** 在 6 个基础模型上的实验表明，性能优于现有 SOTA 基线。

### 4.2 Hindsight is 20/20: Building Agent Memory that Retains, Recalls, and Reflects

| 字段 | 内容 |
|------|------|
| **日期** | 2025年12月 |
| **链接** | [arXiv:2512.12818](https://arxiv.org/abs/2512.12818) · [GitHub](https://github.com/vectorize-io/hindsight) |

**四层记忆网络：**

| 网络 | 描述 |
|------|------|
| **Opinion（信念）** | 主观信念，附带演化的置信度分数 |
| **Observation（观察）** | 综合的实体摘要 |
| **Experience（经历）** | 智能体自身的行为和交互 |
| **World（世界）** | 客观的外部事实 |

**三步核心操作：**
- **Retain（保留）** — 从对话流中提取"叙事事实"，保留因果/语义链接
- **Recall（回忆）** — 多策略检索（时间过滤 + 图遍历 + BM25 + 语义向量搜索）
- **Reflect（反思）** — 基于可配置行为画像，推理生成回答并可追溯地更新信念

**性能：**
- LongMemEval 基准：**83.6% 准确率**（基线 39%）
- 扩展模型后达到 **91.4%**，超越 GPT-4o 和现有记忆系统

### 4.3 Forgetful but Faithful (MaRS)

| 字段 | 内容 |
|------|------|
| **作者** | Saad Alqithami |
| **日期** | 2025年12月 |
| **链接** | [arXiv:2512.12856](https://arxiv.org/abs/2512.12856) |

**核心贡献：**
- **MaRS (Memory-Aware Retention Schema)** — 认知启发式记忆架构
- 管理四类记忆：情景记忆、语义记忆、社交记忆、任务记忆
- 提出 **6 种理论基础的遗忘策略** + 可选差分隐私保障
- 提出 **FiFA (Forgetful but Faithful Agent) 基准**

**评估维度：** 叙事连贯性、目标完成度、社交回忆准确率、隐私保护、成本效率

**结果：** 混合遗忘策略在 300 次评估中达到综合分数 **0.911**。

### 4.4 MemGen: Weaving Generative Latent Memory for Self-Evolving Agents

| 字段 | 内容 |
|------|------|
| **作者** | Guibin Zhang, Muxin Fu, Shuicheng Yan 等 |
| **机构** | National University of Singapore |
| **发表** | ICLR 2026 |
| **链接** | [arXiv:2509.24704](https://arxiv.org/abs/2509.24704) · [ICLR 2026 Poster](https://iclr.cc/virtual/2026/poster/10006821) · [GitHub](https://github.com/KANABOON1/MemGen) |

**核心创新：** 将记忆**编织进推理过程**，而非外部存储/检索。

**架构：**
- **Memory Trigger（记忆触发器）** — 监控推理状态，决定何时调用记忆
- **Memory Weaver（记忆编织器）** — 生成潜在 token 序列作为"机器原生记忆"，融入推理流

**涌现属性：** 无需显式监督，MemGen **自发发展出**：
- 规划记忆 (Planning Memory)
- 程序性记忆 (Procedural Memory)
- 工作记忆 (Working Memory)

**性能：** 8 个基准上超越 ExpeL 和 AWM 达 **38.22%**，超越 GRPO 达 **13.44%**。

### 记忆架构对比总结

| 系统 | 核心范式 | 记忆组织 | 关键特点 |
|------|----------|----------|----------|
| **原始 Generative Agents** | 记忆流 + 反思 | 扁平流式 | 时效性/重要性/相关性三权重检索 |
| **A-MEM** | Zettelkasten 网络 | 互联笔记图 | 动态链接、记忆演化 |
| **Hindsight** | 四层逻辑网络 | 结构化分层 | retain-recall-reflect，多策略检索 |
| **MaRS** | 认知类型化 | 类型化节点 | 6种遗忘策略，差分隐私 |
| **MemGen** | 生成式潜在记忆 | 潜在token流 | 编织进推理，涌现记忆类型 |

---

## 五、涌现行为研究

### 5.1 Spontaneous Emergence of Agent Individuality through Social Interactions in LLM-Based Communities

| 字段 | 内容 |
|------|------|
| **作者** | Ryosuke Takata, Atsushi Masumori, Takashi Ikegami |
| **机构** | University of Tokyo |
| **发表** | Entropy, 2024年12月 |
| **链接** | [arXiv:2411.03252](https://arxiv.org/abs/2411.03252) · [PubMed Central](https://pmc.ncbi.nlm.nih.gov/articles/PMC11675631/) |

**核心发现：**
- LLM 智能体在社区交互中**自发涌现出**个性差异、合作模式和社会规范
- 无需预设任何行为规则
- 智能体自主生成"幻觉"和"标签"来维持交流
- 词汇多样性随交互提高，个性在社区形成过程中分化

**方法论意义：** 提供了分析集体人工智能的新计算建模方法。

### 5.2 Shall We Team Up: Exploring Spontaneous Cooperation of Competing LLM Agents

| 字段 | 内容 |
|------|------|
| **发表** | EMNLP 2024 Findings, ICML 2024 Workshop |
| **链接** | [arXiv:2402.12327](https://arxiv.org/abs/2402.12327) · [ACL Anthology](https://aclanthology.org/2024.findings-emnlp.297/) |

**核心发现：**
- 在竞争场景中，LLM 智能体能**自发产生合作行为**
- 成功模拟了合作的渐进涌现过程
- 结果与人类行为数据**高度一致**
- 说明智能体通过上下文理解做出适应性决策，而非依赖预编程规则

**应用：** 识别代理市场中的潜在共谋风险，为 LLM 智能体的监管提供参考。

### 5.3 Learning to Make Friends: Coaching LLM Agents toward Emergent Social Ties

| 字段 | 内容 |
|------|------|
| **作者** | Philipp J. Schneider, Lin Tian, Marian-Andrei Rizoiu |
| **发表** | NeurIPS 2025 |
| **链接** | [arXiv:2510.19299](https://arxiv.org/abs/2510.19299) · [OpenReview](https://openreview.net/forum?id=dylEDdHOyY) · [NeurIPS 2025](https://neurips.cc/virtual/2025/124535) |

**核心发现：**
- 引导式 LLM 智能体发展出稳定交互模式并形成**涌现性社交纽带**
- 产生的网络结构**镜像真实在线社区**的属性
- 行为奖励函数捕捉了在线互动的核心驱动力：
  - 社交互动
  - 信息寻求
  - 自我展示
  - 协调配合
  - 情感支持

---

## 六、综述论文

### 6.1 A Survey on Multi-Generative Agent System: Recent Advances and New Frontiers

| 字段 | 内容 |
|------|------|
| **作者** | Chen, Liu, Han, Zhang, Liu |
| **机构** | Harbin Institute of Technology |
| **日期** | 2024年12月 |
| **链接** | [arXiv:2412.17481](https://arxiv.org/abs/2412.17481) · [GitHub](https://github.com/bianhua-12/multi-generative_agent_system_survey) |

**覆盖范围：** 综述了 **125 篇**来自 ACL、NeurIPS、AAAI、ICLR 等顶会的论文（2023-2024）

**三大应用类别：**
1. **解决复杂任务** — 协作编程、推理、问答
2. **模拟特定场景** — 社会仿真、经济模拟、游戏环境
3. **评估生成式智能体** — 基准测试与性能评价

### 6.2 Agent-based Modeling Meets Generative AI (LLM-Empowered ABM)

| 字段 | 内容 |
|------|------|
| **日期** | 2023年12月 |
| **链接** | [arXiv:2312.11970](https://arxiv.org/abs/2312.11970) |

**覆盖范围：** 整合 LLM 到 Agent-Based Modeling and Simulation (ABMS) 的全面综述

**四大挑战领域：**
- 环境感知 (Environment Perception)
- 人类对齐 (Human Alignment)
- 行为生成 (Action Generation)
- 评估方法 (Evaluation)

**四大应用领域：** 网络空间、物理环境、社会环境、混合环境

### 6.3 A Survey on LLM-based Multi-Agent Systems: Workflow, Infrastructure, and Challenges

| 字段 | 内容 |
|------|------|
| **日期** | 2024年 |
| **链接** | [Springer](https://link.springer.com/article/10.1007/s44336-024-00009-2) |

**系统框架 — 五大关键组件：**
1. Profile（角色画像）
2. Perception（感知）
3. Self-Action（自主行为）
4. Mutual Interaction（相互交互）
5. Evolution（演化）

---

## 七、评估与基准

### 7.1 BALROG: Benchmarking Agentic LLM and VLM Reasoning On Games

| 字段 | 内容 |
|------|------|
| **日期** | 2024年11月 |
| **链接** | [arXiv:2411.13543](https://arxiv.org/abs/2411.13543) · [官网](http://balrogai.com/) · [GitHub](https://github.com/balrog-ai/BALROG) |

**评估维度：** 游戏环境中的智能体推理能力（从简单到 NetHack 级复杂度）

**关键发现：**
- 当前模型仅在简单任务上部分成功
- 复杂任务表现显著不足
- 视觉决策能力严重欠缺（部分模型加入视觉后反而更差）

### 7.2 TeamCraft: A Benchmark for Multi-Modal Multi-Agent Systems in Minecraft

| 字段 | 内容 |
|------|------|
| **日期** | 2024年12月 |
| **链接** | [arXiv:2412.05255](https://arxiv.org/abs/2412.05255) |

**特点：** 55,000 个任务变体，评估协作智能体在目标/场景/人数三个维度的泛化能力

**关键发现：** 现有模型在多维泛化上面临重大挑战。

### 7.3 RoleAgent / RoleAgentBench

| 字段 | 内容 |
|------|------|
| **发表** | NeurIPS 2024 (Datasets & Benchmarks Track) |
| **链接** | [NeurIPS Proceedings](https://proceedings.neurips.cc/paper_files/paper/2024/hash/5875aca1ef70285a35940afbbce0f9fb-Abstract-Datasets_and_Benchmarks_Track.html) |

**特点：**
- 从剧本自动提取角色（无需人工标注）
- **100 英文角色**（20 部剧本）+ **28 中文角色**（5 部剧本）
- 两阶段框架：Building（层次化记忆提取）+ Interacting（四步交互机制）

### 7.4 SOTOPIA / LIFELONG-SOTOPIA

| 字段 | 内容 |
|------|------|
| **SOTOPIA-π** | [arXiv:2403.08715](https://arxiv.org/abs/2403.08715) · ACL 2024 |
| **LIFELONG-SOTOPIA** | [arXiv:2506.12666](https://arxiv.org/abs/2506.12666) |

**SOTOPIA-π：** 通过交互学习提升社交智能，7B 模型达到 GPT-4 专家级社交目标完成率

**LIFELONG-SOTOPIA：**
- 评估跨 **40 个连续社交回合** 的智能体表现
- 关键发现：所有模型的目标达成率和可信度随交互增加而**下降**
- 高级记忆方法有所改善但无法完全解决问题

---

## 八、技术演进脉络

```
2023.04 ── Generative Agents (Smallville, 25个智能体)
   │         奠定"观察-规划-反思"架构
   │         记忆流 + 三权重检索 + 反思触发
   │
2023.12 ── Concordia (Google DeepMind)
   │         通用 GABM 框架 + Game Master 机制
   │         支持物理/社会/数字三种环境
   │
2024 ──── 记忆架构百花齐放
   │         A-MEM (Zettelkasten 知识网络)
   │         Hindsight (四层逻辑网络)
   │         MaRS (认知类型化 + 隐私保护)
   │         MemGen (生成式潜在记忆)
   │
   │       涌现行为研究兴起
   │         自发个性分化 · 竞争中的合作涌现 · 社交纽带形成
   │
   │       评估基准建立
   │         BALROG · TeamCraft · RoleAgentBench · SOTOPIA
   │
2024.11 ── Generative Agent Simulations of 1,000 People
   │         真实个体仿真, 85%准确率
   │         降低跨群体偏差
   │
2025.02 ── AgentSociety (10,000+智能体, 500万交互)
              大规模社会科学仿真平台
              验证结果与真实实验一致
```

### 关键趋势洞察

1. **架构演进**：从简单的"记忆流+反思"发展到多层次认知记忆网络，逐渐逼近人类认知模型
2. **规模跃迁**：25 → 1,000 → 10,000+ 智能体，仿真规模呈指数增长
3. **从虚构到真实**：从虚构小镇角色到基于真实访谈的个体仿真，可信度不断提高
4. **涌现性行为**：个性、合作、社会规范等复杂社会现象无需预编程即可自发产生
5. **评估体系化**：从定性评估到系统化基准（游戏推理、长期社交、角色扮演、合作博弈）
6. **应用拓展**：社会科学研究、政策模拟、游戏 NPC、用户测试、教育训练、数字孪生

---

## 九、论文速查索引

| # | 论文 | 年份 | 类别 | 链接 |
|---|------|------|------|------|
| 1 | Generative Agents: Interactive Simulacra of Human Behavior | 2023 | 奠基 | [arXiv:2304.03442](https://arxiv.org/abs/2304.03442) |
| 2 | Generative Agent Simulations of 1,000 People | 2024 | 大规模仿真 | [arXiv:2411.10109](https://arxiv.org/abs/2411.10109) |
| 3 | AgentSociety | 2025 | 大规模仿真 | [arXiv:2502.08691](https://arxiv.org/abs/2502.08691) |
| 4 | Concordia (Google DeepMind) | 2023 | 框架 | [arXiv:2312.03664](https://arxiv.org/abs/2312.03664) |
| 5 | A-MEM: Agentic Memory | 2025 | 记忆架构 | [arXiv:2502.12110](https://arxiv.org/abs/2502.12110) |
| 6 | Hindsight: Retain, Recall, Reflect | 2025 | 记忆架构 | [arXiv:2512.12818](https://arxiv.org/abs/2512.12818) |
| 7 | MaRS / Forgetful but Faithful | 2025 | 记忆架构 | [arXiv:2512.12856](https://arxiv.org/abs/2512.12856) |
| 8 | MemGen: Generative Latent Memory | 2024 | 记忆架构 | [arXiv:2509.24704](https://arxiv.org/abs/2509.24704) · [ICLR 2026](https://iclr.cc/virtual/2026/poster/10006821) |
| 9 | Spontaneous Emergence of Individuality | 2024 | 涌现行为 | [arXiv:2411.03252](https://arxiv.org/abs/2411.03252) |
| 10 | Shall We Team Up: Spontaneous Cooperation | 2024 | 涌现行为 | [arXiv:2402.12327](https://arxiv.org/abs/2402.12327) |
| 11 | Learning to Make Friends | 2025 | 涌现行为 | [arXiv:2510.19299](https://arxiv.org/abs/2510.19299) · [OpenReview](https://openreview.net/forum?id=dylEDdHOyY) |
| 12 | Survey on Multi-Generative Agent System | 2024 | 综述 | [arXiv:2412.17481](https://arxiv.org/abs/2412.17481) |
| 13 | LLM-Empowered Agent-Based Modeling | 2023 | 综述 | [arXiv:2312.11970](https://arxiv.org/abs/2312.11970) |
| 14 | LLM-based Multi-Agent Workflow Survey | 2024 | 综述 | [Springer](https://link.springer.com/article/10.1007/s44336-024-00009-2) |
| 15 | BALROG | 2024 | 评估基准 | [arXiv:2411.13543](https://arxiv.org/abs/2411.13543) |
| 16 | TeamCraft | 2024 | 评估基准 | [arXiv:2412.05255](https://arxiv.org/abs/2412.05255) |
| 17 | RoleAgent / RoleAgentBench | 2024 | 评估基准 | [NeurIPS 2024](https://proceedings.neurips.cc/paper_files/paper/2024/hash/5875aca1ef70285a35940afbbce0f9fb-Abstract-Datasets_and_Benchmarks_Track.html) |
| 18 | SOTOPIA-π | 2024 | 评估基准 | [arXiv:2403.08715](https://arxiv.org/abs/2403.08715) |
| 19 | LIFELONG-SOTOPIA | 2025 | 评估基准 | [arXiv:2506.12666](https://arxiv.org/abs/2506.12666) |

---

> **关键 GitHub 仓库：**
>
> - [joonspk-research/generative_agents](https://github.com/joonspk-research/generative_agents) — 原始 Smallville 实现
> - [joonspk-research/genagents](https://github.com/joonspk-research/genagents) — 1,000 People 仿真
> - [google-deepmind/concordia](https://github.com/google-deepmind/concordia) — Concordia 框架
> - [agiresearch/A-mem](https://github.com/agiresearch/A-mem) — A-MEM 记忆系统
> - [vectorize-io/hindsight](https://github.com/vectorize-io/hindsight) — Hindsight 记忆架构
> - [KANABOON1/MemGen](https://github.com/KANABOON1/MemGen) — MemGen 生成式记忆
> - [balrog-ai/BALROG](https://github.com/balrog-ai/BALROG) — BALROG 基准

---

## 附录：论文 PDF 本地文件索引

所有论文 PDF 已下载至 `docs/research/paper/` 目录，共 19 篇。

| # | 论文 | 本地文件 | 大小 |
|---|------|----------|------|
| 1 | Generative Agents: Interactive Simulacra of Human Behavior | [`2304.03442.pdf`](paper/2304.03442.pdf) | 11 MB |
| 2 | Generative Agent Simulations of 1,000 People | [`2411.10109.pdf`](paper/2411.10109.pdf) | 2.9 MB |
| 3 | AgentSociety | [`2502.08691.pdf`](paper/2502.08691.pdf) | 45 MB |
| 4 | Concordia (Google DeepMind) | [`2312.03664.pdf`](paper/2312.03664.pdf) | 974 KB |
| 5 | A-MEM: Agentic Memory | [`2502.12110.pdf`](paper/2502.12110.pdf) | 991 KB |
| 6 | Hindsight: Retain, Recall, Reflect | [`2512.12818.pdf`](paper/2512.12818.pdf) | 2.3 MB |
| 7 | MaRS / Forgetful but Faithful | [`2512.12856.pdf`](paper/2512.12856.pdf) | 847 KB |
| 8 | MemGen: Generative Latent Memory | [`2509.24704.pdf`](paper/2509.24704.pdf) | 10 MB |
| 9 | Spontaneous Emergence of Individuality | [`2411.03252.pdf`](paper/2411.03252.pdf) | 14 MB |
| 10 | Shall We Team Up: Spontaneous Cooperation | [`2402.12327.pdf`](paper/2402.12327.pdf) | 3.7 MB |
| 11 | Learning to Make Friends | [`2510.19299.pdf`](paper/2510.19299.pdf) | 768 KB |
| 12 | Survey on Multi-Generative Agent System | [`2412.17481.pdf`](paper/2412.17481.pdf) | 395 KB |
| 13 | LLM-Empowered Agent-Based Modeling | [`2312.11970.pdf`](paper/2312.11970.pdf) | 1.4 MB |
| 14 | LLM-based Multi-Agent Workflow Survey | [`LLM-MAS-Workflow-Survey_Springer2024.pdf`](paper/LLM-MAS-Workflow-Survey_Springer2024.pdf) | 3.3 MB |
| 15 | BALROG | [`2411.13543.pdf`](paper/2411.13543.pdf) | 4.4 MB |
| 16 | TeamCraft | [`2412.05255.pdf`](paper/2412.05255.pdf) | 42 MB |
| 17 | RoleAgent / RoleAgentBench | [`RoleAgent_NeurIPS2024.pdf`](paper/RoleAgent_NeurIPS2024.pdf) | 1.7 MB |
| 18 | SOTOPIA-π | [`2403.08715.pdf`](paper/2403.08715.pdf) | 2.3 MB |
| 19 | LIFELONG-SOTOPIA | [`2506.12666.pdf`](paper/2506.12666.pdf) | 1.9 MB |

> **总计**: 19 篇论文, 约 149 MB

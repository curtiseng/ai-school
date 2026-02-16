# 人格与 Generative Agents 论文研究综述

> 最后更新：2026-02-16
>
> 本文档系统梳理了**人格心理学（MBTI / Big Five 等）与生成式智能体（Generative Agents）行为交互**领域的核心论文，涵盖性格赋予框架、行为对齐实验、性格一致性评估、角色扮演对话、博弈与社会仿真等方向。

---

## 目录

- [一、MBTI 性格赋予与行为影响](#一mbti-性格赋予与行为影响)
- [二、Big Five 性格模型方法](#二big-five-性格模型方法)
- [三、行为对齐与博弈实验](#三行为对齐与博弈实验)
- [四、角色扮演与性格对话](#四角色扮演与性格对话)
- [五、性格评估与基准测试](#五性格评估与基准测试)
- [六、研究全景与关键洞察](#六研究全景与关键洞察)
- [七、论文速查索引](#七论文速查索引)

---

## 一、MBTI 性格赋予与行为影响

### 1.1 MBTI-in-Thoughts: Psychologically Enhanced AI Agents

| 字段 | 内容 |
|------|------|
| **作者** | Maciej Besta 等 |
| **机构** | ETH Zurich, BASF SE |
| **日期** | 2025年9月 |
| **链接** | [arXiv:2509.04343](https://arxiv.org/abs/2509.04343) · [GitHub](https://github.com/spcl/MBTI-in-Thoughts) |

**核心贡献：** 提出基于 MBTI 的 LLM 智能体性格调制框架，是**最直接将 MBTI 16 型人格应用于生成式智能体行为控制**的工作。

**方法设计：**
- 通过 **Prompt Engineering** 将不同 MBTI 人格原型注入智能体，无需模型微调
- 控制行为沿两个核心心理学维度：**认知 (Cognition)** 和 **情感 (Affect)**
- 使用官方 **16Personalities 测试** 进行自动化人格验证，确保特质持久性
- 在交互前引入 **自我反思 (Self-reflection)** 步骤

**关键发现：**

| 智能体类型 | 优势领域 |
|-----------|----------|
| 情感表达型 (如 ENFP/INFP) | 叙事生成任务表现更优 |
| 分析理性型 (如 INTJ/ISTJ) | 博弈场景中策略更稳定 |
| 自我反思增强型 | 合作与推理质量均提升 |

**泛化性：** 框架不限于 MBTI，可扩展至 **Big Five、HEXACO、九型人格 (Enneagram)** 等心理学模型。

**意义：** 首次系统验证了 MBTI 人格调制对 LLM 智能体在多种任务（叙事生成、博弈论、合作推理）中行为的**可控且一致的影响**。

---

### 1.2 NetworkGames: MBTI 人格 × 网络拓扑 × 合作博弈

| 字段 | 内容 |
|------|------|
| **日期** | 2025年11月 |
| **链接** | [arXiv:2511.21783](https://arxiv.org/abs/2511.21783) |

**核心贡献：** 构建仿真框架研究**网络拓扑结构与 MBTI 人格分布如何共同决定合作行为的宏观涌现**。

**方法设计：**
- 为 LLM 智能体群体赋予 **MBTI 16 型人格**
- 部署在不同网络结构中：**小世界网络 (Small-World)**、**无标度网络 (Scale-Free)** 等
- 使用**迭代囚徒困境 (Iterated Prisoner's Dilemma)** 建立人格对之间的基线交互模式

**关键发现：**

| 发现 | 说明 |
|------|------|
| **宏观不可还原** | 宏观合作水平**无法**仅从双人交互模式预测——网络连接性与人格分布共同决定结果 |
| **小世界网络不利合作** | 小世界网络拓扑对合作行为有**抑制效应** |
| **关键节点策略** | 在无标度网络的 **Hub 节点** 战略性部署亲社会人格（如 ESFJ/ENFJ），可**显著促进**整体合作水平 |

**应用前景：** 为设计更健康的在线社交环境、预测集体行为提供了理论框架。

**意义：** 首次将 **MBTI 人格 + 网络科学 + 博弈论 + 多智能体 LLM 系统** 四大领域融合，揭示了人格分布如何在网络结构中产生涌现合作。

---

## 二、Big Five 性格模型方法

### 2.1 Designing AI-Agents with Personalities: A Psychometric Approach

| 字段 | 内容 |
|------|------|
| **日期** | 2024年10月 |
| **链接** | [arXiv:2410.19238](https://arxiv.org/abs/2410.19238) |

**核心贡献：** 提出基于 **Big Five Inventory-2 (BFI-2)** 的心理测量方法，为 AI 智能体赋予**定量化**的 Big Five 人格特质。

**三阶段研究设计：**
1. **阶段一** — 验证 LLM 能否捕捉 Big Five 量表之间的语义相似性
2. **阶段二** — 以 BFI-2 格式 Prompt 智能体，评估人格评估的对齐度
3. **阶段三** — 与人类数据进行全面对比分析

**关键发现：**
- LLM **能够**捕捉 Big Five 量表间的语义相似性
- BFI-2 格式的 Prompt 使智能体在人格评估上**更接近人类数据**
- 但在**更细粒度的回答模式**上仍与人类存在不一致
- **结论：AI 智能体目前还不能完全替代人类参与高精度研究**

---

### 2.2 BIG5-CHAT: Shaping LLM Personalities Through Training on Human-Grounded Data

| 字段 | 内容 |
|------|------|
| **日期** | 2024年10月 |
| **链接** | [arXiv:2410.16491](https://arxiv.org/abs/2410.16491) |

**核心贡献：** 创建 **100,000 条人类对话数据集**，通过训练（而非 Prompt）来塑造 LLM 的人格特质。

**方法：**
- 数据集基于**人类如何在语言中表达人格特质**
- 使用 **Supervised Fine-Tuning (SFT)** + **Direct Preference Optimization (DPO)** 训练
- 在 BFI 和 IPIP-NEO 人格评估量表上验证

**关键发现：**

| 发现 | 说明 |
|------|------|
| **训练 > Prompt** | 训练方法在人格评估上**显著优于**Prompt 方法 |
| **人格-推理关联** | 高尽责性 + 高宜人性 + 低外倾性 + 低神经质 → 推理能力更强 |
| **与心理学一致** | 上述关联与人类心理学研究结论**高度吻合** |

**意义：** 证明了人格特质可以通过**数据驱动训练**深度植入模型，且人格与认知能力之间存在可迁移的关联。

---

### 2.3 Driving Generative Agents With Their Personality

| 字段 | 内容 |
|------|------|
| **作者** | Klinkert, Buongiorno, Clark |
| **日期** | 2024年2月 |
| **链接** | [arXiv:2402.14879](https://arxiv.org/abs/2402.14879) |

**核心贡献：** 将 **IPIP 心理测量问卷** 应用于游戏 NPC 的人格驱动行为生成。

**方法：**
- 利用 **情感计算 (Affective Computing)** 系统量化 NPC 的心理特质
- 将 IPIP 问卷结果转化为 LLM Prompt 参数
- 评估 LLM（特别是 GPT-4）能否根据人格画像生成一致的角色行为

**关键发现：**
- GPT-4 能**一致地**解读和体现给定的人格画像
- 基于心理测量值驱动的 NPC 行为比通用回答**更像真人**
- 为**游戏 AI 角色开发**提供了桥接情感计算与 LLM 的有效路径

---

### 2.4 Personality-Driven Decision-Making in LLM-Based Autonomous Agents

| 字段 | 内容 |
|------|------|
| **作者** | Lewis Newsham, Daniel Prince |
| **机构** | Lancaster University |
| **发表** | AAMAS 2025 |
| **日期** | 2025年4月 |
| **链接** | [arXiv:2504.00727](https://arxiv.org/abs/2504.00727) · [AAMAS 2025](https://www.ifaamas.org/Proceedings/aamas2025/pdfs/p1538.pdf) |

**核心贡献：** 研究 **OCEAN 人格特质如何影响 LLM 智能体的任务选择与决策过程**。

**方法：**
- 基于 SANDMAN 架构，为智能体注入 Five-Factor OCEAN 人格
- 不仅观察日程生成，更关注智能体如何在**执行阶段**基于人格重新优先排序任务
- 分析不同人格配置下活动被转化执行的差异

**关键发现：**
- 人格调制**显著影响**智能体的任务选择模式
- 不同 OCEAN 配置产生了**可区分的**行为偏好
- 高尽责性智能体更倾向完成计划中的任务，高开放性智能体更容易偏离计划

**应用场景：** 设计"欺骗型智能体"用于**主动网络防御**——在蜜罐环境中复刻可信的人类行为模式以欺骗攻击者。

---

### 2.5 LLMs Simulate Big Five Personality Traits: Further Evidence

| 字段 | 内容 |
|------|------|
| **日期** | 2024年2月 |
| **链接** | [arXiv:2402.01765](https://arxiv.org/abs/2402.01765) |

**核心贡献：** 对多个主流 LLM 的 Big Five 人格模拟能力进行**实证验证**。

**测试模型：** Llama2, GPT-4, Mixtral 等

**关键发现：**
- 主流 LLM **均能**模拟 Big Five 人格特质
- 人格表征在**多轮交互中保持稳定**
- 为后续人格驱动智能体研究提供了**基础能力验证**

---

## 三、行为对齐与博弈实验

### 3.1 Assessing Behavioral Alignment of Personality-Driven Generative Agents in Social Dilemma Games

| 字段 | 内容 |
|------|------|
| **发表** | NeurIPS 2024 Workshop |
| **链接** | [OpenReview](https://openreview.net/forum?id=WCa25ExtbJ) · [NeurIPS Virtual](https://neurips.cc/virtual/2024/102131) |

**核心贡献：** 在**社会困境博弈**中系统评估人格驱动的生成式智能体的行为对齐程度。

**方法：**
- 以 Big Five 特质量值控制智能体行为
- 部署在经典社会困境场景（囚徒困境、公共品博弈等）
- 将智能体行为结果与**人类行为研究数据**对比

**关键发现：**
- 行为结果**可以**通过人格特质量值来调控
- 整体趋势与人类行为研究**基本一致**
- 但在**精确行为匹配**上仍存在差距
- AI 智能体与人类的行为对齐是**近似的、方向性的**，而非精确复刻

---

### 3.2 Assessing Social Alignment: Do Personality-Prompted LLMs Behave Like Humans?

| 字段 | 内容 |
|------|------|
| **日期** | 2024年12月 |
| **链接** | [arXiv:2412.16772](https://arxiv.org/abs/2412.16772) |

**核心贡献：** 使用经典心理学实验测试人格 Prompt 的 LLM 是否真的"像人类一样行为"。

**实验设计：**
- 使用 **Milgram 服从实验** 和 **最后通牒博弈 (Ultimatum Game)** 等经典范式
- 测试多个厂商的 LLM 在人格 Prompt 下的行为

**关键发现：**
- 所有测试模型都表现出**共同的失败模式**
- Prompt 方式的人格调制产生的行为结果**不一致**
- **警示性结论**：对 Prompt 方式人格调制的乐观预期需要审慎看待
- 人格 Prompt 可能改变语言风格，但未必改变**深层决策行为**

---

## 四、角色扮演与性格对话

### 4.1 PsyPlay: Personality-Infused Role-Playing Conversational Agents

| 字段 | 内容 |
|------|------|
| **日期** | 2025年2月 |
| **链接** | [arXiv:2502.03821](https://arxiv.org/abs/2502.03821) · [OpenReview](https://openreview.net/forum?id=S5BFq0uwix) |

**核心贡献：** 构建**人格注入的角色扮演对话系统**，使 LLM 智能体在对话中持续展现指定人格特质。

**三阶段框架：**
1. **角色卡创建 (Role Card Creation)** — 基于 Big Five 模型，附带从"有一点"到"极其"的特质强度标度，并生成个人经历细节
2. **话题提取 (Topic Extraction)** — 确保对话内容相关性
3. **对话生成 (Dialogue Generation)** — 在对话中持续体现人格特质

**性能验证：**

| 模型 | 人格准确呈现率 |
|------|-------------|
| GPT-3.5 | **80.31%** |
| GPT-4o | **89.99%** |

**重要发现：** 经过价值观对齐的 LLM 在扮演**正面人格**时成功率更高，扮演**负面人格**时表现下降。

**数据集：** 发布 **PsyPlay-Bench**，包含 4,745 条准确人格对话实例。

---

### 4.2 LLM Agents in Interaction: Measuring Personality Consistency and Linguistic Alignment

| 字段 | 内容 |
|------|------|
| **作者** | Ivar Frisch, Mario Giulianelli |
| **发表** | PERSONALIZE 2024 Workshop, EACL 2024 |
| **日期** | 2024年2月 |
| **链接** | [arXiv:2402.02896](https://arxiv.org/abs/2402.02896) · [ACL Anthology](https://aclanthology.org/2024.personalize-1.9.pdf) |

**核心贡献：** 研究**人格一致性与语言对齐在多智能体对话中的表现**。

**方法：**
- 用人格画像 Prompt GPT-3.5，创建两组 LLM 智能体群体
- 使用**变异诱导采样算法**增加个体差异
- 管理人格测试 + 协作写作任务

**关键发现：**
- 不同人格画像展现出**不同程度的**人格一致性
- 智能体会对对话伙伴产生**语言风格对齐**（语言适应现象）
- 人格一致性与语言对齐之间存在**张力**
- 需要开发新方法来平衡交互中的特质稳定性和自然交流

---

### 4.3 Capturing Minds, Not Just Words: Enhancing Role-Playing LMs with Personality-Indicative Data

| 字段 | 内容 |
|------|------|
| **日期** | 2024年6月 |
| **链接** | [arXiv:2406.18921](https://arxiv.org/abs/2406.18921) |

**核心贡献：** 提出使用**人格指示性数据**增强角色扮演语言模型，捕捉角色的"内心"而非仅模仿措辞。

**关键思路：**
- 传统角色扮演 LM 关注说话风格和背景知识
- 本研究关注角色的**深层人格特质**如何影响其回答
- 使用心理学量表数据训练模型理解人格特质与行为表达的映射

---

## 五、性格评估与基准测试

### 5.1 PersonaGym: Evaluating Persona Agents and LLMs

| 字段 | 内容 |
|------|------|
| **日期** | 2024年7月 |
| **链接** | [arXiv:2407.18416](https://arxiv.org/abs/2407.18416) |

**核心贡献：** 首个专为 **Persona Agent** 设计的动态评估框架。

**特点：**
- 提出 **PersonaScore** — 基于决策理论的人类对齐自动评分指标
- 10 个主流 LLM × 200 个人格画像 × 10,000 个测试问题
- 大规模评估人格忠实度

**关键发现：**
- 模型规模和复杂度增加**不一定**提升人格一致性
- GPT-4.1 在 PersonaScore 上的得分与 **LLaMA-3-8b 持平**
- 小模型在人格保持方面**并不逊色**于大模型

---

### 5.2 MBTIBench: A Comprehensive Benchmark for MBTI Personality Detection

| 字段 | 内容 |
|------|------|
| **日期** | 2024年12月 |
| **链接** | [arXiv:2412.12510](https://arxiv.org/abs/2412.12510) |

**核心贡献：** 首个**人工标注的 MBTI 人格检测基准数据集**，使用软标签。

**关键发现：**
- 现有数据集中 **29.58%** 的自报告标签包含**错误标注**
- 硬标签（如"你是 INTJ"）**无法**捕捉人群中人格分布的完整范围
- 提出使用**软标签**（概率分布）更准确地表示 MBTI 特质

---

### 5.3 TRAIT: Personality Testbed for LLMs

| 字段 | 内容 |
|------|------|
| **日期** | 2024年6月 |
| **链接** | [arXiv:2406.14703](https://arxiv.org/abs/2406.14703) |

**核心贡献：** 基于 BFI 和 Short Dark Triad 的 **8,000 道多选题基准**，评估 LLM 内在人格特质。

**关键发现：**

| 发现 | 说明 |
|------|------|
| **内在人格** | LLM 展现出**稳定且一致**的人格特质，深受训练数据和对齐训练影响 |
| **Prompt 局限** | 当前 Prompt 技术**难以有效诱导**某些特质（如高精神病态或低尽责性） |
| **对齐偏差** | 安全对齐训练使模型倾向于"好性格"，压制"坏性格"的表达 |

---

## 六、研究全景与关键洞察

### 6.1 研究主题分布

```
人格赋予方法
├── MBTI 框架
│   ├── MBTI-in-Thoughts (Prompt Engineering, 16型人格)
│   ├── NetworkGames (MBTI × 网络拓扑 × 合作博弈)
│   └── MBTIBench (MBTI 检测基准)
├── Big Five / OCEAN 框架
│   ├── Designing AI-Agents with Personalities (心理测量方法)
│   ├── BIG5-CHAT (训练方法 > Prompt 方法)
│   ├── LLMs Simulate Big Five (基础能力验证)
│   ├── Personality-Driven Decision-Making (任务选择影响)
│   └── Driving Generative Agents (游戏 NPC 应用)
└── 通用心理学
    ├── PsyPlay (角色扮演对话)
    ├── Capturing Minds (人格指示性数据)
    └── TRAIT (内在人格评估)

行为验证与对齐
├── Social Dilemma Games (博弈行为对齐)
├── Social Alignment (经典心理学实验)
├── LLM Agents in Interaction (对话中的一致性)
└── PersonaGym (人格忠实度评估)
```

### 6.2 两大技术路线对比

| 维度 | Prompt 方法 | 训练方法 |
|------|-----------|---------|
| **代表工作** | MBTI-in-Thoughts, PersonaGym | BIG5-CHAT |
| **实现成本** | 低（无需微调） | 高（需要数据集+训练） |
| **人格评估得分** | 中等 | 更高 |
| **行为一致性** | 语言风格改变明显，决策行为改变有限 | 更深层的行为一致性 |
| **灵活性** | 高（可即时切换人格） | 低（需重新训练） |
| **局限** | 难以诱导与对齐方向矛盾的特质 | 数据收集成本高 |

### 6.3 MBTI vs Big Five 在 Agent 领域的应用对比

| 维度 | MBTI | Big Five |
|------|------|----------|
| **类型 vs 维度** | 16种离散类型 | 5个连续维度 |
| **Agent 应用优势** | 直观的角色设定，适合叙事和游戏 | 更精细的行为控制，适合科学研究 |
| **代表论文数量** | 较少（3篇核心） | 较多（8篇以上） |
| **行为预测准确度** | 定性方向一致 | 定量趋势更可靠 |
| **主流学术倾向** | 游戏/NPC/社会仿真 | 心理测量/行为科学 |

### 6.4 关键洞察与开放问题

**已确立的发现：**
1. LLM **能够**模拟和表达人格特质（MBTI 和 Big Five 均验证）
2. 人格调制**确实影响**智能体的行为模式（任务选择、合作倾向、叙事风格）
3. 训练方法在人格深度上**优于** Prompt 方法
4. 人格效应在**博弈和社会仿真**场景中表现最为显著
5. 网络拓扑与人格分布**共同决定**宏观合作行为（而非简单叠加）

**关键局限与开放问题：**
1. **对齐偏差**：安全对齐训练压制负面人格表达，导致人格空间覆盖不完整
2. **表层 vs 深层**：Prompt 可能只改变语言风格，未必改变决策逻辑（Milgram 实验证据）
3. **精度差距**：AI 智能体与人类行为在宏观趋势上一致，但微观模式仍有差距
4. **评估难题**：缺乏统一的人格行为对齐评估标准
5. **长期稳定性**：人格一致性在长时间交互中是否会衰减（参考 LIFELONG-SOTOPIA 的发现）
6. **跨文化适用性**：现有研究主要基于英文 LLM，中文等其他语言的人格表达特性尚不清楚

---

## 七、论文速查索引

| # | 论文 | 年份 | 类别 | 链接 |
|---|------|------|------|------|
| 1 | MBTI-in-Thoughts: Psychologically Enhanced AI Agents | 2025 | MBTI 框架 | [arXiv:2509.04343](https://arxiv.org/abs/2509.04343) |
| 2 | NetworkGames (MBTI × 网络拓扑 × 合作) | 2025 | MBTI 博弈 | [arXiv:2511.21783](https://arxiv.org/abs/2511.21783) |
| 3 | Designing AI-Agents with Personalities | 2024 | Big Five 框架 | [arXiv:2410.19238](https://arxiv.org/abs/2410.19238) |
| 4 | BIG5-CHAT: Shaping LLM Personalities | 2024 | Big Five 训练 | [arXiv:2410.16491](https://arxiv.org/abs/2410.16491) |
| 5 | Driving Generative Agents With Their Personality | 2024 | 游戏 NPC | [arXiv:2402.14879](https://arxiv.org/abs/2402.14879) |
| 6 | Personality-Driven Decision-Making | 2025 | OCEAN 决策 | [arXiv:2504.00727](https://arxiv.org/abs/2504.00727) |
| 7 | LLMs Simulate Big Five Personality Traits | 2024 | 能力验证 | [arXiv:2402.01765](https://arxiv.org/abs/2402.01765) |
| 8 | Behavioral Alignment in Social Dilemma Games | 2024 | 博弈对齐 | [OpenReview](https://openreview.net/forum?id=WCa25ExtbJ) |
| 9 | Social Alignment: Personality-Prompted LLMs | 2024 | 行为对齐 | [arXiv:2412.16772](https://arxiv.org/abs/2412.16772) |
| 10 | PsyPlay: Personality-Infused Role-Playing | 2025 | 角色扮演 | [arXiv:2502.03821](https://arxiv.org/abs/2502.03821) |
| 11 | LLM Agents in Interaction (Personality Consistency) | 2024 | 对话一致性 | [arXiv:2402.02896](https://arxiv.org/abs/2402.02896) |
| 12 | Capturing Minds, Not Just Words | 2024 | 角色扮演 | [arXiv:2406.18921](https://arxiv.org/abs/2406.18921) |
| 13 | PersonaGym: Evaluating Persona Agents | 2024 | 评估基准 | [arXiv:2407.18416](https://arxiv.org/abs/2407.18416) |
| 14 | MBTIBench: MBTI Detection Benchmark | 2024 | MBTI 基准 | [arXiv:2412.12510](https://arxiv.org/abs/2412.12510) |
| 15 | TRAIT: Personality Testbed for LLMs | 2024 | 人格评估 | [arXiv:2406.14703](https://arxiv.org/abs/2406.14703) |

---

## 附录：论文 PDF 本地文件索引

所有论文 PDF 已下载至 `docs/research/paper/` 目录。

| # | 论文 | 本地文件 |
|---|------|----------|
| 1 | MBTI-in-Thoughts | [`2509.04343.pdf`](paper/2509.04343.pdf) |
| 2 | NetworkGames | [`2511.21783.pdf`](paper/2511.21783.pdf) |
| 3 | Designing AI-Agents with Personalities | [`2410.19238.pdf`](paper/2410.19238.pdf) |
| 4 | BIG5-CHAT | [`2410.16491.pdf`](paper/2410.16491.pdf) |
| 5 | Driving Generative Agents With Their Personality | [`2402.14879.pdf`](paper/2402.14879.pdf) |
| 6 | Personality-Driven Decision-Making | [`2504.00727.pdf`](paper/2504.00727.pdf) |
| 7 | PsyPlay | [`2502.03821.pdf`](paper/2502.03821.pdf) |
| 8 | LLM Agents in Interaction | [`2402.02896.pdf`](paper/2402.02896.pdf) |
| 9 | Capturing Minds, Not Just Words | [`2406.18921.pdf`](paper/2406.18921.pdf) |
| 10 | PersonaGym | [`2407.18416.pdf`](paper/2407.18416.pdf) |
| 11 | MBTIBench | [`2412.12510.pdf`](paper/2412.12510.pdf) |
| 12 | TRAIT | [`2406.14703.pdf`](paper/2406.14703.pdf) |

---

> **相关文档：** [Generative Agents 论文研究综述](generative-agents-papers.md) — 生成式智能体的奠基工作、架构演进、记忆系统等

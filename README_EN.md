# AI School

English | [中文](README.md)

> Simulate a school with AI, and observe how students grow.

## Vision

Every child is unique. Yet our education system rarely has the chance to truly understand — in what kind of social environment an introverted child will open up, or at which turning point a curious student finds their life direction.

**What AI School does is simple: let AI students "live" in a virtual campus, then observe them.**

Each AI student has an independent personality, memory, and goals. They attend classes, make friends, debate, get confused, and grow — none of this is pre-programmed. Instead, it **emerges** naturally from individual interactions. By observing these emergent behaviors, we study the patterns of student psycho-social development and career alignment.

This is not an educational game — it's a **research tool**. We believe that understanding the growth patterns of virtual students will help us better accompany real children.

## Quick Start

```bash
# 1. Clone the repo
git clone https://github.com/your-org/ai-school.git && cd ai-school

# 2. Configure environment variables
cp .env.example .env
# Edit .env, fill in your DeepSeek and Zhipu API keys

# 3. Start infrastructure
docker compose up -d

# 4. Build frontend
cd frontend && npm install && npx vite build && cd ..

# 5. Start the server
cargo run --bin ai-school-api

# 6. Open browser
open http://localhost:3000
```

For detailed setup instructions, see [SETUP.md](SETUP.md).

## Demo Video

<video src="assert/demo.mp4" controls width="100%"></video>

## UI Preview

```
┌──────────────────────────────────────────────────────────┐
│  [⚡] AI SCHOOL    [▶ Start] [⏸ Pause] [1x ▾]    Day 15 │
├──────────┬──────────────────────────┬────────────────────┤
│          │                          │                    │
│  Agent   │    2D Campus Map         │   Detail / Chat    │
│  List    │    (React-Konva)         │   Panel            │
│          │                          │                    │
│  Ming    │  ┌─────┐  ┌─────┐       │  [MBTI] ESTP       │
│  ESTP    │  │Class │  │Libr.│       │  [Mood] ██████░    │
│  Study   │  └─────┘  └─────┘       │  [Skill] ████░░    │
│          │    @  @  @               │                    │
│  Hong    │  ┌─────┐  ┌─────┐       │  "Hi teacher, I've │
│  INFJ    │  │Field │  │Cafe │       │   been thinking.." │
│  Social  │  └─────┘  └─────┘       │  [Send message...] │
│          │                          │                    │
├──────────┴──────────────────────────┴────────────────────┤
│  [Intervention]  Diff:████░░  Social:██████░  [Events ▾] │
└──────────────────────────────────────────────────────────┘
```

## Two Phases

| Phase | Goal | Form |
|-------|------|------|
| **Phase 1** | Study patterns of student psycho-social development and career alignment | AI school simulation platform (research tool) |
| **Phase 2** | Transform research insights into real student companionship | Student companion robot (user-facing product) |

## Architecture Overview

The system consists of 6 major modules. Phase 1 focuses on the first 5:

```
AI School
├── M1. Student Agent System       ← Personality, aspiration, cognitive-behavioral framework
├── M2. School World System        ← Curriculum, social dynamics, environment & time
├── M3. Evolution Engine           ← Autonomous evolution, user intervention, event generation
├── M4. Memory & Growth System     ← Multi-layer memory, psycho-social tracking, personality drift
├── M5. Research & Analysis        ← Trajectory visualization, career matching, controlled experiments
└── M6. Student Companion Robot    ← Phase 2: Profile modeling, wellness support, career guidance
```

## Tech Stack

### Backend

- **Language**: Rust (concurrency safety, long-running stability, strict type system)
- **Web Framework**: Axum (HTTP + WebSocket)
- **LLM**: DeepSeek (agent decisions + GM arbitration) + Zhipu AI (memory embedding)
- **Storage**: PostgreSQL (structured data) + Qdrant (vector memory retrieval)
- **Architecture**: Cargo Workspace with 8 crates, inter-module decoupling via trait abstractions

### Frontend

- **Framework**: React + TypeScript + Vite
- **2D Rendering**: React-Konva (Canvas campus map)
- **State Management**: Zustand + real-time WebSocket sync
- **Styling**: Tailwind CSS (dark laboratory research theme)
- **Charts**: Recharts

### Data Flow

```
User Action → REST API → Simulation Engine → LLM Call → State Update
                                                         ↓
                                         WebSocket ← Broadcast → Frontend Real-time Render
```

## Research Foundation

The architecture is grounded in a **systematic review of 34 papers**, spanning five directions:

### Generative Agent Architecture

Starting from Stanford's 2023 Smallville experiment (25 AI residents living autonomously in a virtual town), the research community has progressively validated three core axioms:

1. **Behavior = f(Memory, Context, Goals)** — Every action is jointly driven by accumulated memory, current perception, and intrinsic personality
2. **Social behavior is an emergent property** — Information diffusion, relationship formation, and activity coordination arise naturally from individual interactions, not pre-programming
3. **Memory is cognitive infrastructure** — Ablation studies show behavioral credibility drops by 8.16 standard deviations when memory is removed

Subsequent work — 1,000 People (2024) extended simulation to real individuals; AgentSociety (2025) scaled to 10,000+ agents / 5 million interactions — with results highly consistent with real-world experiments.

### Memory System Evolution

Memory architectures have undergone four paradigm shifts:

| Generation | Paradigm | Representative Work | Core Breakthrough |
|------------|----------|-------------------|-------------------|
| 1st | Flat memory stream | Generative Agents (2023) | Weighted retrieval by recency × importance × relevance |
| 2nd | Interconnected knowledge network | A-MEM (2025) | Memory as a continuously refined Zettelkasten network, not static archive |
| 3rd | Multi-layer logical network | Hindsight (2025) | Distinguishing fact / belief / observation / experience; accuracy from 39% → 91.4% |
| 4th | Generative latent memory | MemGen (ICLR 2026) | Memory woven into reasoning; planning memory and working memory emerge spontaneously |

AI School adopts a layered memory architecture (perception → short-term → long-term → semantic), incorporating core designs from the 2nd and 3rd generations.

### Personality & Behavior

Two technical approaches coexist: prompt-based methods (instant effect, but only alter surface linguistic style) and training-based methods (deep implantation; BIG5-CHAT proved significantly superior to prompting). MBTI suits intuitive role design while Big Five suits precise behavioral prediction — AI School uses MBTI 4-dimensional continuous scores as its base framework.

Key caveat: Social Alignment research using Milgram's obedience experiment demonstrates that prompts may only change an agent's wording, not its deep-level decision-making. This demands continuous verification of personality consistency.

### Emergent Behavior

Three categories of emergent phenomena have been observed, none pre-programmed:

- **Spontaneous personality differentiation** — Homogeneous agents autonomously develop differentiated personalities through community interaction
- **Spontaneous cooperation in competition** — Agents progressively "discover" cooperative strategies in competitive scenarios, highly consistent with human behavioral data
- **Social bond formation** — Network structures developed by agents mirror real online communities

NetworkGames (2025) further reveals: macro-level cooperation cannot be predicted from pairwise interactions — network topology and personality distribution **jointly determine** emergent outcomes. This means simulation scale itself brings qualitative change.

### Known Challenges

- **Long-term consistency decay** — LIFELONG-SOTOPIA found all models exhibit declining performance after 40+ interaction rounds; memory improvements help but are insufficient
- **Emergence unpredictability** — Large-scale simulation results are difficult to predict a priori
- **Alignment bias** — Safety training biases models toward "good personalities," suppressing negative personality expression and resulting in incomplete personality space coverage

See [`docs/research/`](docs/research/) for details.

## Documentation

| Document | Description |
|----------|-------------|
| [`SETUP.md`](SETUP.md) | Complete environment setup and launch guide |
| [`docs/research/`](docs/research/) | Research survey and deep analysis of 34 papers |
| [`docs/prd/`](docs/prd/) | Product requirements framework and per-module PRDs |
| [`docs/adr/`](docs/adr/) | Architecture Decision Records (ADRs) |

## Development Status

**Phase 1a MVP** — Core simulation engine + Web interaction layer complete:

- [x] M1: Student Agent System (personality, cognition, career)
- [x] M2: School World System (campus, time, curriculum, social)
- [x] M3: Evolution Engine (simulation loop, GM arbitration, intervention)
- [x] M4: Memory System (multi-layer memory, vector retrieval, reflection triggers)
- [x] LLM Integration (DeepSeek completion + Zhipu embedding)
- [x] HTTP/WebSocket API (15 endpoints)
- [x] Web UI (2D campus map, agent management, chat, intervention, data export)
- [x] M5: Research & Analysis Platform (chart visualization, controlled experiments)
- [x] Persistent Storage (PostgreSQL + Qdrant integration)
- [ ] Large-scale Simulation Optimization (50+ agents)

## License

MIT

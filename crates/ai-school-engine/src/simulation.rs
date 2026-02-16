//! M3.1 自主演化循环（仿真主循环）
//!
//! 核心编排层：协调 Agent 决策、GM 仲裁、状态更新、记忆写入。

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use tokio::sync::broadcast;
use tracing::{debug, error, info, instrument, warn};

use ai_school_core::config::SimulationConfig;
use ai_school_core::error::SimulationError;
use ai_school_core::traits::llm::LlmProvider;
use ai_school_core::traits::MemoryStore;
use ai_school_core::types::{
    AgentId, AgentState, BehaviorIntent, EventId, EventTrigger, Memory, MemoryId,
    MemoryLayer, MemoryQuery, SimulationEvent, SimulationSpeed, SimulationTime,
};

use ai_school_agent::cognition::CognitionProcessor;
use ai_school_memory::reflection::ReflectionTrigger;
use ai_school_world::state::WorldState;

use crate::broadcast::SimulationUpdate;
use crate::game_master::GameMaster;

/// 仿真步骤结果
#[derive(Debug)]
pub struct StepResult {
    pub tick: u64,
    pub events: Vec<SimulationEvent>,
    pub warnings: Vec<String>,
}

/// 仿真运行器 — ADR-0003 数据流的完整实现
pub struct SimulationRunner<L: LlmProvider, M: MemoryStore> {
    pub llm: Arc<L>,
    pub memory_store: Arc<M>,
    pub world: WorldState,
    pub config: SimulationConfig,
    pub speed: SimulationSpeed,
    pub game_master: GameMaster,
    pub reflection_trigger: ReflectionTrigger,
    pub event_tx: broadcast::Sender<SimulationUpdate>,
    /// Shared atomic flag — can be set from outside without holding the RwLock
    pub running: Arc<AtomicBool>,
}

impl<L: LlmProvider, M: MemoryStore> SimulationRunner<L, M> {
    pub fn new(
        llm: Arc<L>,
        memory_store: Arc<M>,
        config: SimulationConfig,
    ) -> Self {
        let (event_tx, _) = broadcast::channel(1024);

        Self {
            llm: llm.clone(),
            memory_store,
            world: WorldState::new(config.time_step_hours),
            config: config.clone(),
            speed: SimulationSpeed::Paused,
            game_master: GameMaster::new(),
            reflection_trigger: ReflectionTrigger::new(config.reflection_threshold),
            event_tx,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    /// 添加 Agent 到仿真
    pub fn add_agent(&mut self, agent: AgentState) {
        self.world.add_agent(agent);
    }

    /// 获取事件订阅
    pub fn subscribe(&self) -> broadcast::Receiver<SimulationUpdate> {
        self.event_tx.subscribe()
    }

    /// 执行一个仿真步骤
    #[instrument(skip(self), fields(tick = self.world.clock.current_time().tick))]
    pub async fn step(&mut self) -> Result<StepResult, SimulationError> {
        let mut events = Vec::new();
        let mut warnings = Vec::new();

        // 1. 时间推进 → 触发时间事件
        let time_events = self.world.clock.advance();
        let current_time = self.world.clock.current_time().clone();

        for te in &time_events {
            debug!(event = ?te, "Time event triggered");
        }

        // 1b. 处理时间事件 → 移动 Agent 到对应位置
        self.world.process_time_events(&time_events);

        // 2. 为每个活跃 Agent 构建 SituationContext 并执行决策
        let agent_ids: Vec<AgentId> = self.world.agents.keys().cloned().collect();
        let mut intents = Vec::new();

        for agent_id in &agent_ids {
            match self.agent_decision(agent_id, &current_time).await {
                Ok(intent) => intents.push(intent),
                Err(e) => {
                    warn!(agent = %agent_id, error = %e, "Agent decision failed");
                    warnings.push(format!("Agent {} decision failed: {e}", agent_id));
                }
            }
        }

        // 3. Game Master 仲裁
        let gm_output = self
            .game_master
            .arbitrate(&intents, &self.world, &*self.llm)
            .await?;

        // 4. 应用状态变更
        let change_warnings = self.world.apply_state_changes(&gm_output.state_changes)?;
        warnings.extend(change_warnings);

        // 5. 创建并记录事件
        let event = SimulationEvent {
            id: EventId::new(),
            event_type: gm_output.event_type,
            trigger: EventTrigger::AgentAction,
            timestamp: current_time.clone(),
            involved_agents: intents.iter().map(|i| i.agent_id.clone()).collect(),
            narrative: gm_output.narrative,
            state_changes: gm_output.state_changes,
            intensity: gm_output.intensity,
        };
        events.push(event.clone());
        self.world.event_log.push(event.clone());

        // 6. 写入记忆 + 检查反思
        for agent_id in &agent_ids {
            self.update_agent_memory(agent_id, &event).await;
        }

        // 7. 广播更新
        let snapshot = self.world.snapshot();
        let _ = self.event_tx.send(SimulationUpdate::Tick {
            time: current_time.clone(),
            snapshot,
            events: events.clone(),
        });

        let tick = current_time.tick;
        debug!(tick, agents = agent_ids.len(), "Step completed");

        Ok(StepResult {
            tick,
            events,
            warnings,
        })
    }

    /// 单个 Agent 的决策过程
    async fn agent_decision(
        &self,
        agent_id: &AgentId,
        current_time: &SimulationTime,
    ) -> Result<BehaviorIntent, SimulationError> {
        let agent = self.world.get_agent(agent_id)?;

        // 检索相关记忆
        let query = MemoryQuery {
            query_text: self.world.describe_situation(agent_id),
            layer_filter: None,
            tag_filter: vec![],
            since: None,
            limit: 5,
        };

        let dummy_embedding = vec![0.0; 2048]; // TODO: use actual embedding
        let memories = self
            .memory_store
            .retrieve(agent_id, &query, &dummy_embedding)
            .await
            .unwrap_or_default();

        let memory_texts: Vec<String> = memories.iter().map(|m| m.memory.content.clone()).collect();

        // 构建情境上下文
        let context = ai_school_core::types::SituationContext {
            agent_id: agent_id.clone(),
            time: current_time.clone(),
            perception: ai_school_core::types::Perception {
                nearby_agents: self
                    .world
                    .agents_at_location(&agent.location)
                    .iter()
                    .filter(|a| a.id != *agent_id)
                    .map(|a| a.id.clone())
                    .collect(),
                observable_activities: Vec::new(),
                environment_description: self.world.describe_situation(agent_id),
                recent_events: self
                    .world
                    .event_log
                    .iter()
                    .rev()
                    .take(3)
                    .map(|e| e.narrative.clone())
                    .collect(),
            },
            relevant_memories: memory_texts,
            emotional_summary: format!(
                "情绪: 效价={:.1}, 唤醒={:.1}, 压力={:.1}",
                agent.emotion.valence, agent.emotion.arousal, agent.emotion.stress
            ),
            personality_description: ai_school_agent::personality::personality_description(
                &agent.config.personality,
            ),
            career_summary: ai_school_agent::career::CareerDatabase::aspiration_description(
                &agent.config.career_aspiration,
            ),
        };

        // 认知处理：组装 LLM 请求
        let request = CognitionProcessor::think(agent, &context);

        // 执行 LLM 调用（调用点 #1: Agent 决策）
        let response = self.llm.complete(&request).await?;

        // 解析行为意图
        let intent = CognitionProcessor::act(agent, &response.content);

        Ok(intent)
    }

    /// 更新 Agent 记忆
    async fn update_agent_memory(&mut self, agent_id: &AgentId, event: &SimulationEvent) {
        if !event.involved_agents.contains(agent_id) {
            return;
        }

        let current_time = self.world.clock.current_time().clone();

        let memory = Memory {
            id: MemoryId::new(),
            agent_id: agent_id.clone(),
            layer: MemoryLayer::ShortTerm,
            content: event.narrative.clone(),
            timestamp: current_time.clone(),
            importance: event.intensity,
            emotion_valence: 0.0,
            event_id: Some(event.id.clone()),
            tags: vec![format!("{:?}", event.event_type)],
            access_count: 0,
            last_accessed: current_time,
        };

        let dummy_embedding = vec![0.0; 2048]; // TODO: use actual embedding
        if let Err(e) = self.memory_store.store(agent_id, &memory, &dummy_embedding).await {
            error!(agent = %agent_id, error = %e, "Failed to store memory");
        }

        // Check reflection trigger
        if self.reflection_trigger.record_event(agent_id) {
            info!(agent = %agent_id, "Reflection triggered");
            // TODO: Execute reflection process
        }
    }

    /// Get a clone of the running flag (for the stop handler to use without the write lock)
    pub fn running_flag(&self) -> Arc<AtomicBool> {
        self.running.clone()
    }

    /// Check if running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    /// 运行仿真主循环
    pub async fn run(&mut self) -> Result<(), SimulationError> {
        self.running.store(true, Ordering::Relaxed);
        info!("Simulation started");

        while self.running.load(Ordering::Relaxed) {
            if self.speed == SimulationSpeed::Paused {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                continue;
            }

            let result = self.step().await?;
            debug!(tick = result.tick, events = result.events.len(), "Simulation step");

            // Wait based on speed
            if let Some(interval_ms) = self.speed.step_interval_ms() {
                if interval_ms > 0 {
                    tokio::time::sleep(tokio::time::Duration::from_millis(interval_ms)).await;
                }
            }
        }

        info!("Simulation stopped");
        Ok(())
    }

    /// 停止仿真 (can be called with &self since AtomicBool)
    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    /// 设置仿真速度
    pub fn set_speed(&mut self, speed: SimulationSpeed) {
        self.speed = speed;
        let _ = self.event_tx.send(SimulationUpdate::SpeedChanged { speed });
    }
}

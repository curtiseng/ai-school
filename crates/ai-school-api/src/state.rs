use std::sync::Arc;

use tokio::sync::RwLock;

use ai_school_core::config::AppConfig;
use ai_school_engine::simulation::SimulationRunner;
use ai_school_llm::providers::mock::MockLlmProvider;
use ai_school_memory::store::in_memory::InMemoryStore;

/// Application shared state
#[derive(Clone)]
pub struct AppState {
    pub runner: Arc<RwLock<SimulationRunner<MockLlmProvider, InMemoryStore>>>,
    pub config: AppConfig,
}

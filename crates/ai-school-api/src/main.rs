use std::sync::Arc;

use anyhow::Result;
use axum::Router;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

use ai_school_core::config::AppConfig;
use ai_school_engine::simulation::SimulationRunner;
use ai_school_llm::providers::mock::MockLlmProvider;
use ai_school_memory::store::in_memory::InMemoryStore;

mod dto;
mod error;
mod routes;
mod state;
mod ws;

use state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ai_school=debug,tower_http=debug".into()),
        )
        .init();

    // Load env
    dotenvy::dotenv().ok();

    // Config
    let config = AppConfig::default();

    // Initialize LLM provider (use Mock for development)
    let llm = Arc::new(MockLlmProvider::default());
    let memory_store = Arc::new(InMemoryStore::new());

    // Create simulation runner
    let runner = SimulationRunner::new(
        llm.clone(),
        memory_store.clone(),
        config.simulation.clone(),
    );

    // Build app state
    let app_state = AppState {
        runner: Arc::new(RwLock::new(runner)),
        config: config.clone(),
    };

    // Build router
    let app = Router::new()
        .merge(routes::simulation::router())
        .merge(routes::agents::router())
        .merge(routes::intervention::router())
        .merge(routes::analysis::router())
        .merge(ws::router())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    info!("Starting AI School API server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

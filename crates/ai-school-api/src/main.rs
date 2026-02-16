use std::sync::Arc;

use anyhow::Result;
use axum::Router;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::info;

use ai_school_core::config::{AppConfig, LlmConfig, QdrantConfig, DatabaseConfig, ServerConfig};
use ai_school_engine::simulation::SimulationRunner;
use ai_school_llm::providers::deepseek::DeepSeekProvider;
use ai_school_memory::store::in_memory::InMemoryStore;

mod dto;
mod error;
mod routes;
mod state;
mod ws;

use state::AppState;

fn load_config() -> AppConfig {
    let llm = LlmConfig {
        chat_base_url: std::env::var("DEEPSEEK_BASE_URL")
            .unwrap_or_else(|_| "https://api.deepseek.com".into()),
        chat_api_key: std::env::var("DEEPSEEK_API_KEY").unwrap_or_default(),
        chat_model: std::env::var("DEEPSEEK_MODEL")
            .unwrap_or_else(|_| "deepseek-chat".into()),
        embedding_base_url: std::env::var("ZHIPU_EMBEDDING_BASE_URL")
            .unwrap_or_else(|_| "https://open.bigmodel.cn/api/paas/v4".into()),
        embedding_api_key: std::env::var("ZHIPU_API_KEY").unwrap_or_default(),
        embedding_model: std::env::var("ZHIPU_EMBEDDING_MODEL")
            .unwrap_or_else(|_| "embedding-3".into()),
        ..Default::default()
    };

    let qdrant = QdrantConfig {
        url: std::env::var("QDRANT_URL")
            .unwrap_or_else(|_| "http://localhost:16333".into()),
        ..Default::default()
    };

    let database = DatabaseConfig {
        url: std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://ai_school:dev_password@localhost:15432/ai_school".into()),
    };

    let server = ServerConfig {
        host: std::env::var("API_HOST").unwrap_or_else(|_| "0.0.0.0".into()),
        port: std::env::var("API_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000),
    };

    AppConfig {
        simulation: Default::default(),
        llm,
        qdrant,
        database,
        server,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ai_school=debug,tower_http=debug".into()),
        )
        .init();

    dotenvy::dotenv().ok();

    let config = load_config();

    info!(
        chat_model = %config.llm.chat_model,
        embedding_model = %config.llm.embedding_model,
        "Loaded LLM configuration"
    );

    let llm = Arc::new(DeepSeekProvider::new(&config.llm));
    let memory_store = Arc::new(InMemoryStore::new());

    let runner = SimulationRunner::new(
        llm.clone(),
        memory_store.clone(),
        config.simulation.clone(),
    );
    let running_flag = runner.running_flag();

    let app_state = AppState {
        runner: Arc::new(RwLock::new(runner)),
        config: config.clone(),
        running: running_flag,
    };

    // Serve frontend static files from frontend/dist/ (fallback to index.html for SPA)
    let frontend_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.join("frontend/dist"))
        .unwrap_or_else(|| std::path::PathBuf::from("frontend/dist"));

    let serve_frontend = ServeDir::new(&frontend_dir)
        .not_found_service(ServeFile::new(frontend_dir.join("index.html")));

    let app = Router::new()
        .merge(routes::simulation::router())
        .merge(routes::agents::router())
        .merge(routes::intervention::router())
        .merge(routes::analysis::router())
        .merge(ws::router())
        .fallback_service(serve_frontend)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    info!("Starting AI School API server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

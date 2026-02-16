use std::sync::atomic::Ordering;

use axum::extract::State;
use axum::routing::{get, post, put};
use axum::{Json, Router};

use ai_school_core::types::SimulationSpeed;

use crate::dto::{SetSpeedRequest, SimulationStatusResponse, SuccessResponse};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/simulation/status", get(get_status))
        .route("/api/simulation/start", post(start_simulation))
        .route("/api/simulation/stop", post(stop_simulation))
        .route("/api/simulation/step", post(step_simulation))
        .route("/api/simulation/speed", put(set_speed))
}

async fn get_status(State(state): State<AppState>) -> Json<SimulationStatusResponse> {
    // running flag is read lock-free via AtomicBool
    let is_running = state.running.load(Ordering::Relaxed);

    // Try to get read lock with timeout for other fields
    match tokio::time::timeout(
        tokio::time::Duration::from_millis(100),
        state.runner.read(),
    )
    .await
    {
        Ok(runner) => {
            let time = runner.world.clock.current_time();
            Json(SimulationStatusResponse {
                running: is_running,
                tick: time.tick,
                time_display: time.display(),
                agent_count: runner.world.agents.len(),
                speed: runner.speed,
            })
        }
        Err(_) => {
            // Runner is busy (write-locked by simulation loop), return partial status
            Json(SimulationStatusResponse {
                running: is_running,
                tick: 0,
                time_display: "运行中...".to_string(),
                agent_count: 0,
                speed: SimulationSpeed::Normal,
            })
        }
    }
}

async fn start_simulation(State(state): State<AppState>) -> Json<SuccessResponse> {
    // Spawn simulation loop in background
    let runner_clone = state.runner.clone();
    tokio::spawn(async move {
        let mut runner = runner_clone.write().await;
        runner.set_speed(SimulationSpeed::Normal);
        if let Err(e) = runner.run().await {
            tracing::error!(error = %e, "Simulation error");
        }
    });

    Json(SuccessResponse {
        success: true,
        message: "Simulation started".to_string(),
    })
}

async fn stop_simulation(State(state): State<AppState>) -> Json<SuccessResponse> {
    // Directly set the AtomicBool — no lock needed at all
    state.running.store(false, Ordering::Relaxed);
    tracing::info!("Stop signal sent");

    Json(SuccessResponse {
        success: true,
        message: "Simulation stopped".to_string(),
    })
}

async fn step_simulation(State(state): State<AppState>) -> Json<serde_json::Value> {
    let mut runner = state.runner.write().await;
    match runner.step().await {
        Ok(result) => Json(serde_json::json!({
            "success": true,
            "tick": result.tick,
            "events": result.events.len(),
            "warnings": result.warnings,
        })),
        Err(e) => Json(serde_json::json!({
            "success": false,
            "error": e.to_string(),
        })),
    }
}

async fn set_speed(
    State(state): State<AppState>,
    Json(req): Json<SetSpeedRequest>,
) -> Json<SuccessResponse> {
    let mut runner = state.runner.write().await;
    runner.set_speed(req.speed);

    Json(SuccessResponse {
        success: true,
        message: format!("Speed set to {:?}", req.speed),
    })
}

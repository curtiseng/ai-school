use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/analysis/snapshot", get(get_snapshot))
        .route("/api/analysis/events", get(get_events))
        .route("/api/analysis/export", get(export_data))
}

async fn get_snapshot(State(state): State<AppState>) -> Json<serde_json::Value> {
    let runner = state.runner.read().await;
    let snapshot = runner.world.snapshot();
    Json(serde_json::to_value(snapshot).unwrap_or_default())
}

async fn get_events(State(state): State<AppState>) -> Json<serde_json::Value> {
    let runner = state.runner.read().await;
    let events: Vec<serde_json::Value> = runner
        .world
        .event_log
        .iter()
        .rev()
        .take(50)
        .map(|e| {
            serde_json::json!({
                "id": e.id.0.to_string(),
                "type": format!("{:?}", e.event_type),
                "time": e.timestamp.display(),
                "narrative": e.narrative,
                "intensity": e.intensity,
                "involved_agents": e.involved_agents.len(),
            })
        })
        .collect();

    Json(serde_json::json!({ "events": events }))
}

async fn export_data(State(state): State<AppState>) -> Json<serde_json::Value> {
    let runner = state.runner.read().await;

    let agents: Vec<serde_json::Value> = runner
        .world
        .agents
        .values()
        .map(|a| serde_json::to_value(a).unwrap_or_default())
        .collect();

    let events: Vec<serde_json::Value> = runner
        .world
        .event_log
        .iter()
        .map(|e| serde_json::to_value(e).unwrap_or_default())
        .collect();

    Json(serde_json::json!({
        "simulation": {
            "time": runner.world.clock.current_time(),
            "agent_count": runner.world.agents.len(),
        },
        "agents": agents,
        "events": events,
    }))
}

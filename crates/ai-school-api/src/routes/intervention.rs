use axum::extract::State;
use axum::routing::{post, put};
use axum::{Json, Router};

use crate::dto::{AdjustParamsRequest, TriggerEventRequest, SuccessResponse};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/interventions/event", post(trigger_event))
        .route("/api/simulation/params", put(adjust_params))
}

async fn trigger_event(
    State(state): State<AppState>,
    Json(req): Json<TriggerEventRequest>,
) -> Json<SuccessResponse> {
    let mut runner = state.runner.write().await;
    let time = runner.world.clock.current_time().clone();

    let mut intervention_manager = ai_school_engine::intervention::InterventionManager::new();
    let event = intervention_manager.trigger_preset_event(&req.event, &time);

    runner.world.event_log.push(event);

    Json(SuccessResponse {
        success: true,
        message: "Event triggered".to_string(),
    })
}

async fn adjust_params(
    State(state): State<AppState>,
    Json(req): Json<AdjustParamsRequest>,
) -> Json<SuccessResponse> {
    let mut runner = state.runner.write().await;

    match req.parameter.as_str() {
        "CourseDifficulty" => {
            runner.config.random_event_frequency = req.value;
        }
        "RandomEventFrequency" => {
            runner.config.random_event_frequency = req.value;
        }
        _ => {
            return Json(SuccessResponse {
                success: false,
                message: format!("Unknown parameter: {}", req.parameter),
            });
        }
    }

    Json(SuccessResponse {
        success: true,
        message: format!("Parameter '{}' set to {}", req.parameter, req.value),
    })
}

use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};

use ai_school_core::types::{CareerAspiration, CareerCategory, PersonalityParams, SimulationTime};
use ai_school_agent::builder::{generate_random_agents, AgentBuilder};

use crate::dto::{CreateAgentRequest, GenerateAgentsRequest, SuccessResponse};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/agents", get(list_agents).post(create_agent))
        .route("/api/agents/generate", post(generate_agents))
        .route("/api/agents/{id}", get(get_agent))
}

async fn list_agents(State(state): State<AppState>) -> Json<serde_json::Value> {
    let runner = state.runner.read().await;
    let agents: Vec<serde_json::Value> = runner
        .world
        .agents
        .values()
        .map(|a| {
            serde_json::json!({
                "id": a.id.0.to_string(),
                "name": a.config.name,
                "mbti": a.config.personality.mbti_label(),
                "location": a.location.0,
                "activity": format!("{:?}", a.activity),
                "emotion": {
                    "valence": a.emotion.valence,
                    "arousal": a.emotion.arousal,
                    "stress": a.emotion.stress,
                },
                "career": a.config.career_aspiration.ideal_career,
            })
        })
        .collect();

    Json(serde_json::json!({ "agents": agents }))
}

async fn create_agent(
    State(state): State<AppState>,
    Json(req): Json<CreateAgentRequest>,
) -> Json<SuccessResponse> {
    let personality = PersonalityParams::new(req.e_i, req.s_n, req.t_f, req.j_p);
    let time = SimulationTime::new();

    let career = CareerAspiration {
        ideal_career: req.ideal_career.unwrap_or_else(|| "探索中".to_string()),
        category: CareerCategory::Other("未定".to_string()),
        subject_preferences: Vec::new(),
        clarity: 0.5,
    };

    let agent = AgentBuilder::new()
        .name(&req.name)
        .personality(personality)
        .career(career)
        .age(req.age.unwrap_or(16))
        .build(&time);

    let mut runner = state.runner.write().await;
    runner.add_agent(agent);

    Json(SuccessResponse {
        success: true,
        message: format!("Agent '{}' created", req.name),
    })
}

async fn generate_agents(
    State(state): State<AppState>,
    Json(req): Json<GenerateAgentsRequest>,
) -> Json<SuccessResponse> {
    let count = req.count.min(10); // MVP 限制最多 10 个
    let time = SimulationTime::new();
    let agents = generate_random_agents(count, &time);

    let mut runner = state.runner.write().await;
    for agent in agents {
        runner.add_agent(agent);
    }

    Json(SuccessResponse {
        success: true,
        message: format!("{count} agents generated"),
    })
}

async fn get_agent(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Json<serde_json::Value> {
    let runner = state.runner.read().await;

    let agent = runner.world.agents.values().find(|a| a.id.0.to_string() == id);

    match agent {
        Some(a) => Json(serde_json::json!({
            "id": a.id.0.to_string(),
            "name": a.config.name,
            "personality": {
                "e_i": a.config.personality.e_i,
                "s_n": a.config.personality.s_n,
                "t_f": a.config.personality.t_f,
                "j_p": a.config.personality.j_p,
                "mbti": a.config.personality.mbti_label(),
                "stability": a.config.personality.stability,
            },
            "career": {
                "ideal": a.config.career_aspiration.ideal_career,
                "clarity": a.config.career_aspiration.clarity,
            },
            "location": a.location.0,
            "activity": format!("{:?}", a.activity),
            "emotion": a.emotion,
            "abilities": a.abilities,
            "current_thought": a.current_thought,
        })),
        None => Json(serde_json::json!({ "error": "Agent not found" })),
    }
}

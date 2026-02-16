use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};

use ai_school_core::types::{CareerAspiration, CareerCategory, PersonalityParams, SimulationTime};
use ai_school_core::traits::llm::{ChatMessage, CompletionRequest, LlmProvider, MessageRole};
use ai_school_agent::builder::{generate_random_agents, AgentBuilder};
use ai_school_agent::personality::personality_description;
use ai_school_agent::career::CareerDatabase;

use crate::dto::{ChatRequest, CreateAgentRequest, GenerateAgentsRequest, SuccessResponse};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/agents", get(list_agents).post(create_agent))
        .route("/api/agents/generate", post(generate_agents))
        .route("/api/agents/{id}", get(get_agent))
        .route("/api/agents/{id}/chat", post(chat_with_agent))
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
                "personality": {
                    "e_i": a.config.personality.e_i,
                    "s_n": a.config.personality.s_n,
                    "t_f": a.config.personality.t_f,
                    "j_p": a.config.personality.j_p,
                },
                "abilities": a.abilities,
                "current_thought": a.current_thought,
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
    let count = req.count.min(10);
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

async fn chat_with_agent(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<ChatRequest>,
) -> Json<serde_json::Value> {
    let runner = state.runner.read().await;

    let agent = match runner.world.agents.values().find(|a| a.id.0.to_string() == id) {
        Some(a) => a.clone(),
        None => return Json(serde_json::json!({ "error": "Agent not found" })),
    };

    let role_desc = match req.role.as_str() {
        "teacher" => "班主任老师",
        "principal" => "校长",
        "counselor" => "心理辅导员",
        _ => &req.role,
    };

    let personality_desc = personality_description(&agent.config.personality);
    let career_desc = CareerDatabase::aspiration_description(&agent.config.career_aspiration);

    let system = format!(
        "你是一个名叫{}的高中生。以下是你的角色设定：\n\
        人格特征: {}\n\
        职业志向: {}\n\
        当前情绪: 效价={:.1}, 唤醒={:.1}, 压力={:.1}\n\
        当前活动: {:?}\n\
        当前位置: {}\n\n\
        现在有一位{}想和你对话。请根据你的人格特征、情绪状态自然地回应。\
        回应要简短（1-3句），像一个真实的高中生那样说话。用中文回答。",
        agent.config.name,
        personality_desc,
        career_desc,
        agent.emotion.valence,
        agent.emotion.arousal,
        agent.emotion.stress,
        agent.activity,
        agent.location.0,
        role_desc,
    );

    let request = CompletionRequest {
        system,
        messages: vec![ChatMessage {
            role: MessageRole::User,
            content: req.message.clone(),
        }],
        temperature: Some(0.8),
        max_tokens: Some(256),
    };

    let llm = runner.llm.clone();
    drop(runner);

    match llm.complete(&request).await {
        Ok(response) => Json(serde_json::json!({
            "reply": response.content,
            "impact": "对话内容已被记录到 Agent 记忆中",
        })),
        Err(e) => Json(serde_json::json!({
            "error": format!("LLM call failed: {}", e),
        })),
    }
}

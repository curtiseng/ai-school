use std::sync::Arc;

use anyhow::Result;
use tracing::info;

use ai_school_agent::builder::generate_random_agents;
use ai_school_core::config::SimulationConfig;
use ai_school_core::types::SimulationTime;
use ai_school_engine::simulation::SimulationRunner;
use ai_school_llm::providers::mock::MockLlmProvider;
use ai_school_memory::store::in_memory::InMemoryStore;

pub async fn execute(agent_count: usize, steps: usize, output: Option<String>) -> Result<()> {
    info!(agents = agent_count, steps, "Starting batch simulation");

    let llm = Arc::new(MockLlmProvider::default());
    let memory = Arc::new(InMemoryStore::new());
    let config = SimulationConfig::default();

    let mut runner = SimulationRunner::new(llm, memory, config);

    // Generate agents
    let time = SimulationTime::new();
    let agents = generate_random_agents(agent_count, &time);
    for agent in agents {
        info!(name = %agent.config.name, mbti = %agent.config.personality.mbti_label(), "Agent created");
        runner.add_agent(agent);
    }

    // Run simulation
    for i in 0..steps {
        match runner.step().await {
            Ok(result) => {
                if !result.events.is_empty() {
                    for event in &result.events {
                        info!(
                            tick = result.tick,
                            event_type = ?event.event_type,
                            narrative = %event.narrative,
                            "Event"
                        );
                    }
                }
            }
            Err(e) => {
                tracing::error!(step = i, error = %e, "Simulation step failed");
            }
        }
    }

    // Export
    let snapshot = runner.world.snapshot();
    let export_data = serde_json::json!({
        "simulation": {
            "steps": steps,
            "agent_count": agent_count,
            "final_time": snapshot.time,
        },
        "agents": snapshot.agents,
        "relationships": snapshot.relationships,
        "event_count": runner.world.event_log.len(),
    });

    if let Some(path) = output {
        std::fs::write(&path, serde_json::to_string_pretty(&export_data)?)?;
        info!(path = %path, "Data exported");
    } else {
        println!("{}", serde_json::to_string_pretty(&export_data)?);
    }

    info!("Simulation completed");
    Ok(())
}

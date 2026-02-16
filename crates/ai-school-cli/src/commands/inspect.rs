use ai_school_agent::builder::generate_random_agents;
use ai_school_agent::career::CareerDatabase;
use ai_school_agent::personality::personality_description;
use ai_school_core::types::SimulationTime;

pub fn execute(agent_count: usize) {
    let time = SimulationTime::new();
    let agents = generate_random_agents(agent_count, &time);

    println!("\n=== AI School Agent Inspection ===\n");

    for agent in &agents {
        println!("--- {} ---", agent.config.name);
        println!("MBTI: {}", agent.config.personality.mbti_label());
        println!("{}", personality_description(&agent.config.personality));
        println!("职业志向: {}", agent.config.career_aspiration.ideal_career);

        let matches = CareerDatabase::suggest_careers(&agent.config.personality);
        if !matches.is_empty() {
            println!("推荐职业:");
            for m in matches.iter().take(3) {
                println!("  - {} (匹配度: {:.0}%)", m.career, m.score * 100.0);
                for reason in &m.reasons {
                    println!("    · {reason}");
                }
            }
        }
        println!();
    }
}

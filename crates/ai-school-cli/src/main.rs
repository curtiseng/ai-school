use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(name = "ai-school", version, about = "AI School 仿真平台 CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 运行仿真（无 UI，纯数据输出）
    Run {
        /// Agent 数量
        #[arg(short, long, default_value_t = 5)]
        agents: usize,

        /// 仿真步数
        #[arg(short, long, default_value_t = 100)]
        steps: usize,

        /// 输出文件路径
        #[arg(short, long)]
        output: Option<String>,
    },

    /// 查看 Agent 人格匹配
    Inspect {
        /// Agent 数量
        #[arg(short, long, default_value_t = 5)]
        agents: usize,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("ai_school=info")
        .init();

    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        Commands::Run { agents, steps, output } => {
            commands::run::execute(agents, steps, output).await?;
        }
        Commands::Inspect { agents } => {
            commands::inspect::execute(agents);
        }
    }

    Ok(())
}

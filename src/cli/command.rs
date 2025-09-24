use crate::utils::config::Config;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {
    #[clap(about = "Configure LLVM client")]
    Set {
        #[arg(long, env = "OPENAI_URL")]
        open_ai_url: Option<String>,
        #[arg(long, env = "OPENAI_KEY")]
        open_ai_key: Option<String>,
    },
}

pub fn handle_subcommand(command: Command) {
    match command {
        Command::Set {
            open_ai_url,
            open_ai_key,
        } => {
            let config = Config {
                open_ai_url,
                open_ai_key,
            };
            config.save();
        }
    }
}

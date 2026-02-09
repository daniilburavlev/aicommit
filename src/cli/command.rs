use std::process::exit;

use crate::utils::config::Config;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {
    #[clap(about = "See LLVM client configuration")]
    Get,
    #[clap(about = "Configure LLVM client")]
    Set {
        #[arg(long, env = "OPENAI_URL", help = "OpenAI ChatGPT API supported url")]
        open_ai_url: Option<String>,
        #[arg(long, env = "OPENAI_KEY", help = "OpenAI API key")]
        open_ai_key: Option<String>,
        #[arg(long, help = "Comming language, i.e 'hebrew'")]
        lang: Option<String>,
    },
}

pub fn handle_subcommand(command: Command) {
    match command {
        Command::Get => {
            let config = Config::read();
            match serde_json::to_string_pretty(&config) {
                Ok(json) => println!("{}", json),
                Err(_) => {
                    eprintln!("Error reading config");
                    exit(-1);
                }
            }
        }
        Command::Set {
            open_ai_url,
            open_ai_key,
            lang,
        } => {
            let config = Config {
                open_ai_url,
                open_ai_key,
                lang,
            };
            config.save();
        }
    }
}

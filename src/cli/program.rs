use crate::cli::command::{Command, handle_subcommand};
use crate::cli::diff::run_diff;
use clap::Parser;

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = "A CLI that writes messages for your git commits with AI"
)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

pub fn run() {
    let cli = Cli::parse();
    if let Some(command) = cli.command {
        handle_subcommand(command);
        return;
    }
    run_diff();
}

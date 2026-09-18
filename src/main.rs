mod cli;
mod cmd_commit;
mod cmd_doctor;
mod cmd_tr;
mod config;
mod llm;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Cmd};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Tr {
            text,
            to,
            from,
            model,
        } => cmd_tr::run(text, to, from, model).await,
        Cmd::Commit {
            apply,
            staged,
            unstaged,
            model,
        } => cmd_commit::run(apply, staged, unstaged, model).await,
        Cmd::Doctor => cmd_doctor::run().await,
        Cmd::Models => cmd_doctor::models().await,
    }
}

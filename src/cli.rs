use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "aikit", about = "AI toolbox")]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    /// Translate text (args or piped stdin, auto Chinese-English by default)
    Tr {
        text: Vec<String>,
        #[arg(long)]
        to: Option<String>,
        #[arg(long)]
        from: Option<String>,
        #[arg(long)]
        model: Option<String>,
    },
    /// Generate a commit message from uncommitted changes
    Commit {
        #[arg(long)]
        apply: bool,
        #[arg(long)]
        staged: bool,
        #[arg(long)]
        unstaged: bool,
        #[arg(long)]
        model: Option<String>,
    },
    /// Show current config and check API connectivity
    Doctor,
    /// List available models
    Models,
}

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "jugaad-sync", version = "1.0", about = "Git-based 2-way sync tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Start,
    Stop,
    Status,
    Push,
    Pull,
    Logs,
    Config,
}

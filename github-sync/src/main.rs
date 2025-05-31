use clap::{Parser, Subcommand};
use colored::*;

mod commands;
mod config;
mod error;
mod git;
mod github;
mod logger;
mod watcher;
mod remote_watcher;

#[derive(Parser)]
#[command(name = "ghs")]
#[command(about = "Two-way file synchronization using Git", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the GitHub Sync service
    Start {
        /// Name of the repository to sync
        #[arg(short, long)]
        name: String,
    },
    
    /// Stop the GitHub Sync service
    Stop,
    
    /// Show current status
    Status,
    
    /// Configure GitHub Sync
    Config {
        /// Branch to sync with
        #[arg(short, long)]
        branch: Option<String>,
        
        /// Sync interval in seconds
        #[arg(short, long)]
        interval: Option<u64>,
    },

    /// Authenticate with GitHub
    Auth {
        /// GitHub personal access token
        token: String,
    },

    /// Manually push changes to remote
    Push,

    /// Manually pull changes from remote
    Pull,

    /// Show sync history
    Logs,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Start { name } => commands::start::handle(name).await,
        Commands::Stop => commands::stop::handle(),
        Commands::Status => commands::status::handle(),
        Commands::Config { branch, interval } => {
            commands::config::handle(branch, interval)
        }
        Commands::Auth { token } => {
            github::GitHub::save_token(&token)
        }
        Commands::Push => commands::push::handle(),
        Commands::Pull => commands::pull::handle(),
        Commands::Logs => commands::logs::handle(),
    };

    if let Err(e) = result {
        logger::error(&format!("Error: {}", e));
        std::process::exit(1);
    }
}

  
  
    // run auth first each run
    // entry point for the menu: ghs -> 
    /*
    🛠️  Jugaad Sync – Menu

[1] Start Sync
[2] Stop Sync
[3] Show Status
[4] Open Config
[5] Connected devices
[6] Exit
 */ 
    //    


/*
src/
├── main.rs        # Entry point
├── auth.rs        # All authentication logic
├── sync.rs        # All Git sync logic (commit, pull, push)
├── watcher.rs     # File watching and idle timer logic
├── logger.rs      # Logging with colored status labels
├── utils.rs       # General-purpose helpers
├── cli.rs         # CLI structure (clap)
├── commands.rs    # Declares command submodules
└── commands/      # Folder only for CLI subcommands
    ├── start.rs
    ├── stop.rs
    └── ...
*/
mod cli;
mod commands;

use cli::{Cli, Commands};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Start => commands::start::handle(),
        Commands::Stop => commands::stop::handle(),
        Commands::Status => commands::status::handle(),
        Commands::Push => commands::push::handle(),
        Commands::Pull => commands::pull::handle(),
        Commands::Logs => commands::logs::handle(),
        Commands::Config => commands::config::handle(),
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
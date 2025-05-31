// src/commands/start.rs
use std::path::PathBuf;
use std::fs;
use crate::config::Config;
use crate::error::Result;
use crate::git::GitSync;
use crate::github::GitHub;
use crate::watcher::FileWatcher;
use crate::logger;
use dirs;

const PID_FILE: &str = ".github-sync.pid";
const MAIN_REPO: &str = "github-sync";

pub async fn handle(folder_name: String) -> Result<()> {
    // Check if already running
    let pid_file = PathBuf::from(PID_FILE);
    if pid_file.exists() {
        logger::error("GitHub Sync is already running. Use 'ghs stop' first.");
        return Ok(());
    }

    // Write PID file
    fs::write(&pid_file, std::process::id().to_string())?;

    // Initialize GitHub client and ensure main repository exists
    let github = GitHub::new().await?;
    let remote_url = github.ensure_repository(MAIN_REPO).await?;

    // Create sync root in user's home directory
    let sync_root = dirs::home_dir()
        .expect("Could not find home directory")
        .join(".github-sync");
    let folder_path = sync_root.join(&folder_name);
    fs::create_dir_all(&folder_path)?;

    // Load or create config
    let config = Config::load().unwrap_or(Config {
        remote_url,
        branch: "main".to_string(),
        sync_paths: vec![sync_root.clone()],
        sync_interval: 300, // 5 minutes default
    });

    // Initialize Git sync for the main repository
    let git = GitSync::new(
        &sync_root,
        &config.remote_url,
        &config.branch
    )?;

    // Start remote watcher
    logger::info("Starting remote change watcher...");
    crate::remote_watcher::start_remote_watcher();

    // Initial sync
    logger::sync("Performing initial sync...");
    if let Err(e) = git.sync() {
        logger::error(&format!("Initial sync failed: {}", e));
        fs::remove_file(&pid_file)?;
        return Err(e);
    }

    // Initialize and start file watcher
    let mut watcher = FileWatcher::new(git)?;

    // Watch the entire sync directory
    if let Err(e) = watcher.watch(&sync_root) {
        logger::error(&format!("Failed to watch directory: {}", e));
        fs::remove_file(&pid_file)?;
        return Err(e);
    }

    // Set up signal handlers for graceful shutdown
    #[cfg(unix)]
    {
        use nix::sys::signal::{self, SigHandler, Signal};
        unsafe {
            signal::signal(Signal::SIGTERM, SigHandler::Handler(handle_signal))?;
            signal::signal(Signal::SIGINT, SigHandler::Handler(handle_signal))?;
        }
    }

    logger::success(&format!(
        "GitHub Sync started successfully!\nYour files will be synced in: {}\nAll files will be pushed to: {}", 
        folder_path.display(),
        config.remote_url
    ));

    // Run the watcher (this blocks indefinitely)
    let result = watcher.run();
    
    // Clean up PID file
    fs::remove_file(&pid_file)?;
    
    result
}

#[cfg(unix)]
extern "C" fn handle_signal(sig: i32) {
    let signal = nix::sys::signal::Signal::try_from(sig).unwrap();
    logger::info(&format!("Received signal: {:?}", signal));
    logger::info("Shutting down gracefully...");
    
    // Clean up PID file
    if let Err(e) = fs::remove_file(PID_FILE) {
        logger::error(&format!("Failed to clean up PID file: {}", e));
    }
    
    std::process::exit(0);
}

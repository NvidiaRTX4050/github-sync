use crate::error::Result;
use crate::git::GitSync;
use crate::config::Config;
use crate::logger;
use dirs;

pub fn handle() -> Result<()> {
    // Load config
    let config = Config::load()?;
    
    // Get sync root
    let sync_root = dirs::home_dir()
        .expect("Could not find home directory")
        .join(".github-sync");

    // Initialize Git sync
    let git = GitSync::new(
        &sync_root,
        &config.remote_url,
        &config.branch
    )?;

    // Pull changes
    logger::sync("Pulling changes from remote...");
    git.pull()?;
    logger::success("Changes pulled successfully");

    Ok(())
} 
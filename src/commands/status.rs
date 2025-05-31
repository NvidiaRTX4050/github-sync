use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;
use chrono::{DateTime, Local};
use crate::error::Result;
use crate::config::Config;
use crate::logger;

const PID_FILE: &str = ".github-sync.pid";
const STATUS_FILE: &str = ".github-sync.status";

#[derive(serde::Serialize, serde::Deserialize)]
struct StatusInfo {
    last_sync: SystemTime,
    pending_changes: Vec<String>,
}

pub fn handle() -> Result<()> {
    println!("\n{}", "GitHub Sync Status".bold());
    println!("{}", "=".repeat(50));

    // Check if service is running
    let pid_file = PathBuf::from(PID_FILE);
    if pid_file.exists() {
        let pid = fs::read_to_string(&pid_file)?.trim().to_string();
        logger::success(&format!("Service is running (PID: {})", pid));
    } else {
        logger::error("Service is not running");
    }

    // Load and show configuration
    match Config::load() {
        Ok(config) => {
            println!("\n{}", "Configuration:".bold());
            println!("Remote: {}", config.remote_url);
            println!("Branch: {}", config.branch);
            println!("Sync interval: {}s", config.sync_interval);
            
            println!("\n{}", "Watched Paths:".bold());
            if config.sync_paths.is_empty() {
                println!("- Current directory (.)");
            } else {
                for path in config.sync_paths {
                    println!("- {}", path.display());
                }
            }
        }
        Err(e) => {
            logger::warn(&format!("Could not load configuration: {}", e));
        }
    }

    // Show sync status if available
    let status_file = PathBuf::from(STATUS_FILE);
    if status_file.exists() {
        match fs::read_to_string(&status_file) {
            Ok(content) => {
                if let Ok(status: StatusInfo) = serde_json::from_str(&content) {
                    let last_sync: DateTime<Local> = status.last_sync.into();
                    
                    println!("\n{}", "Sync Status:".bold());
                    println!("Last sync: {}", last_sync.format("%Y-%m-%d %H:%M:%S"));
                    
                    if !status.pending_changes.is_empty() {
                        println!("\n{}", "Pending Changes:".bold());
                        for change in status.pending_changes {
                            println!("- {}", change);
                        }
                    }
                }
            }
            Err(_) => {
                logger::warn("Could not read sync status");
            }
        }
    }

    // Show Git status
    if pid_file.exists() {
        match get_git_status() {
            Ok((ahead, behind)) => {
                println!("\n{}", "Git Status:".bold());
                if ahead > 0 {
                    println!("Commits ahead of remote: {}", ahead);
                }
                if behind > 0 {
                    println!("Commits behind remote: {}", behind);
                }
                if ahead == 0 && behind == 0 {
                    logger::success("In sync with remote");
                }
            }
            Err(_) => {
                logger::warn("Could not get Git status");
            }
        }
    }

    println!(); // Add final newline
    Ok(())
}

fn get_git_status() -> Result<(usize, usize)> {
    use git2::{Repository, BranchType};
    
    let repo = Repository::open(".")?;
    let head = repo.head()?;
    let branch_name = head.shorthand().unwrap_or("HEAD");
    
    // Get local and remote commits
    let local = head.target().ok_or_else(|| {
        crate::error::JugaadError::GitError(git2::Error::from_str("Could not get local HEAD"))
    })?;
    
    let remote_branch = repo.find_branch(&format!("origin/{}", branch_name), BranchType::Remote)?;
    let remote = remote_branch.get().target().ok_or_else(|| {
        crate::error::JugaadError::GitError(git2::Error::from_str("Could not get remote HEAD"))
    })?;
    
    let (ahead, behind) = repo.graph_ahead_behind(local, remote)?;
    Ok((ahead, behind))
}
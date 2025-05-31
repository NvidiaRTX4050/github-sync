use std::io::{self, Write};
use colored::*;
use crate::config::Config;
use crate::error::Result;
use crate::logger;
use std::path::PathBuf;

pub fn handle(
    remote: Option<String>,
    branch: Option<String>,
    paths: Option<String>,
    interval: Option<u64>,
) -> Result<()> {
    // Load existing config or create new one
    let mut config = Config::load().unwrap_or(Config {
        remote_url: String::new(),
        branch: "main".to_string(),
        sync_paths: Vec::new(),
        sync_interval: 300, // 5 minutes default
    });

    // Update config with provided values
    if let Some(remote) = remote {
        if !remote.starts_with("git@") {
            logger::error("Remote URL must be an SSH URL (starting with git@)");
            return Ok(());
        }
        config.remote_url = remote;
    }

    if let Some(branch) = branch {
        config.branch = branch;
    }

    if let Some(paths) = paths {
        config.sync_paths = paths
            .split(',')
            .map(|p| PathBuf::from(p.trim()))
            .collect();
    }

    if let Some(interval) = interval {
        if interval < 5 {
            logger::error("Sync interval must be at least 5 seconds");
            return Ok(());
        }
        config.sync_interval = interval;
    }

    // Save the updated config
    config.save()?;
    logger::success("Configuration updated successfully");

    // Show current config
    println!("\nCurrent configuration:");
    println!("Remote: {}", config.remote_url);
    println!("Branch: {}", config.branch);
    println!("Sync interval: {}s", config.sync_interval);
    println!("\nWatched paths:");
    if config.sync_paths.is_empty() {
        println!("- Current directory (.)");
    } else {
        for path in config.sync_paths {
            println!("- {}", path.display());
        }
    }

    Ok(())
} 
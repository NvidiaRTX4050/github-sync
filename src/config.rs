use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::error::{Result, GitHubSyncError};

const CONFIG_FILE: &str = ".github-sync.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub remote_url: String,
    pub branch: String,
    pub sync_paths: Vec<PathBuf>,
    pub sync_interval: u64,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_file = PathBuf::from(CONFIG_FILE);
        if !config_file.exists() {
            return Err(GitHubSyncError::ConfigError("Configuration file not found. Run 'ghs config' first.".to_string()));
        }

        let content = fs::read_to_string(config_file)?;
        serde_json::from_str(&content)
            .map_err(|e| GitHubSyncError::ConfigError(format!("Invalid configuration: {}", e)))
    }

    pub fn save(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| GitHubSyncError::ConfigError(format!("Failed to serialize config: {}", e)))?;

        fs::write(CONFIG_FILE, content)
            .map_err(|e| GitHubSyncError::ConfigError(format!("Failed to write config: {}", e)))?;

        Ok(())
    }

    pub fn is_configured(&self) -> bool {
        !self.remote_url.is_empty() && !self.branch.is_empty()
    }
}
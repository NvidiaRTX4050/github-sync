use std::fs;
use std::path::PathBuf;
use octocrab::Octocrab;
use crate::error::{Result, GitHubSyncError};
use crate::logger;

const TOKEN_FILE: &str = ".github-sync-token";

pub struct GitHub {
    client: Octocrab,
}

impl GitHub {
    pub async fn new() -> Result<Self> {
        let token = Self::get_token()?;
        let client = Octocrab::builder()
            .personal_token(token)
            .build()
            .map_err(|e| GitHubSyncError::GitHubError(e.to_string()))?;

        Ok(Self { client })
    }

    pub async fn ensure_repository(&self, name: &str) -> Result<String> {
        // Extract username from token
        let user = self.client.current()
            .user()
            .await
            .map_err(|e| GitHubSyncError::GitHubError(e.to_string()))?;

        // Check if repo exists
        match self.client.repos(user.login.clone()).get(name).await {
            Ok(_) => {
                logger::info(&format!("Repository {}/{} already exists", user.login, name));
                Ok(format!("git@github.com:{}/{}.git", user.login, name))
            }
            Err(_) => {
                // Create new repository
                logger::info(&format!("Creating new repository: {}", name));
                let repo = self.client
                    .repos()
                    .create(name)
                    .private(true)
                    .send()
                    .await
                    .map_err(|e| GitHubSyncError::GitHubError(e.to_string()))?;

                Ok(format!("git@github.com:{}/{}.git", user.login, name))
            }
        }
    }

    fn get_token() -> Result<String> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| GitHubSyncError::ConfigError("Could not find config directory".to_string()))?;
        let token_path = config_dir.join("github-sync").join(TOKEN_FILE);

        if token_path.exists() {
            Ok(fs::read_to_string(&token_path)?.trim().to_string())
        } else {
            // Create config directory if it doesn't exist
            if let Some(parent) = token_path.parent() {
                fs::create_dir_all(parent)?;
            }

            logger::error("GitHub token not found. Please create a personal access token with 'repo' scope at:");
            logger::info("https://github.com/settings/tokens/new");
            logger::info("Then run: ghs auth <token>");
            
            Err(GitHubSyncError::ConfigError("GitHub token not found".to_string()))
        }
    }

    pub fn save_token(token: &str) -> Result<()> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| GitHubSyncError::ConfigError("Could not find config directory".to_string()))?;
        let token_dir = config_dir.join("github-sync");
        fs::create_dir_all(&token_dir)?;
        
        fs::write(token_dir.join(TOKEN_FILE), token)?;
        logger::success("GitHub token saved successfully");
        Ok(())
    }
} 
use thiserror::Error;
use std::io;

pub type Result<T> = std::result::Result<T, GitHubSyncError>;

#[derive(Error, Debug)]
pub enum GitHubSyncError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("Git error: {0}")]
    GitError(#[from] git2::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Watch error: {0}")]
    WatchError(String),

    #[error("Signal error: {0}")]
    SignalError(String),

    #[error("GitHub error: {0}")]
    GitHubError(String),

    #[error("Unknown error: {0}")]
    UnknownError(String),
}

#[cfg(unix)]
impl From<nix::Error> for GitHubSyncError {
    fn from(err: nix::Error) -> Self {
        GitHubSyncError::SignalError(err.to_string())
    }
}

#[cfg(windows)]
impl From<windows::core::Error> for GitHubSyncError {
    fn from(err: windows::core::Error) -> Self {
        GitHubSyncError::SignalError(err.to_string())
    }
} 
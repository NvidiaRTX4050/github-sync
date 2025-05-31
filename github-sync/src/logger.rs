// src/logger.rs

use colored::*;
use chrono::Local;

#[derive(Debug)]
pub enum Status {
    Success,  // For successful operations
    Error,    // For failures and errors
    Info,     // For general information
    Warn,     // For warnings
    Sync,     // For sync-specific operations
    Watch,    // For file watcher events
}

pub fn log(status: Status, message: &str) {
    let timestamp = Local::now().format("%H:%M:%S").to_string().dimmed();
    
    let status_str = match status {
        Status::Success => "✓".green().bold(),
        Status::Error => "✗".red().bold(),
        Status::Info => "ℹ".bright_blue().bold(),
        Status::Warn => "!".yellow().bold(),
        Status::Sync => "⟳".magenta().bold(),
        Status::Watch => "👁".bright_cyan().bold(),
    };

    println!("{} {} {}", timestamp, status_str, message);
}

// Convenience functions
pub fn success(msg: &str) { log(Status::Success, msg); }
pub fn error(msg: &str) { log(Status::Error, msg); }
pub fn info(msg: &str) { log(Status::Info, msg); }
pub fn warn(msg: &str) { log(Status::Warn, msg); }
pub fn sync(msg: &str) { log(Status::Sync, msg); }
pub fn watch(msg: &str) { log(Status::Watch, msg); }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_log_types() {
        println!("\nLogger Demo:");
        success("Repository synchronized successfully");
        error("Failed to connect to remote");
        info("Initializing sync service");
        warn("Large number of files changed");
        sync("Pulling changes from remote");
        watch("File modified: src/main.rs");
    }
}
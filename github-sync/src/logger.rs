// src/logger.rs

use colored::*;
/*pub enum Status {
    // General statuses
    OK,
    FAIL,
    WARN,
    INFO,
    DEBUG,

    // Auth module
    AuthSuccess,
    AuthFailed,
    UserCancelled,

    // Init module
    InitStarted,
    InitSuccess,
    InitAlreadyExists,
    InitFailed,

    // Watcher (file change detection)
    WatchStarted,
    FileAdded(String),
    FileRemoved(String),
    FileModified(String),
    FileRenamed { from: String, to: String },
    NoChanges,
    IdleTimeout,

    // Sync logic
    SyncStarted,
    SyncComplete,
    SyncConflict,
    SyncSkipped,
    SyncAborted,
    SyncError(String),

    // Git operations
    GitPullSuccess,
    GitPullConflict,
    GitCommitCreated,
    GitCommitSkipped,
    GitPushSuccess,
    GitPushFailed,
    GitBranchBackup(String), // For backups like safe-deletion-2025-05-24

    // Safety / validation
    BulkDeletionWarning(u32),  // % files deleted
    UnsafeOperationBlocked,
    SafeModeActivated,
    BackupBranchCreated(String),

    // Remote status
    RemoteConnected,
    RemoteDisconnected,
    RemoteError(String),

    // Internal / misc
    ConfigLoaded,
    ConfigMissing,
    ConfigError(String),
    TempFileError(String),
}
 */
pub enum Status {
    Ok,
    Fail,
    Info,
}

pub fn log(module: &str, status: Status, message: &str) {
    let status_str = match status {
        Status::Ok => "[ OK ]".green(),
        Status::Fail => "[FAIL]".red(),
        Status::Info => "[INFO]".cyan(),
    };

    let module_str = format!("[ {} ]", module).yellow();

    println!("{} {} {}", module_str, status_str, message);
}

// Convenience wrappers
pub fn log_ok(module: &str, message: &str) {
    log(module, Status::Ok, message);
}

pub fn log_fail(module: &str, message: &str) {
    log(module, Status::Fail, message);
}

pub fn log_info(module: &str, message: &str) {
    log(module, Status::Info, message);
}

fn main()
{
    log_ok("auth", "Token saved.");
    log_fail("watcher", "No changes detected.");
    log_info("sync", "Started syncing with remote.");

}
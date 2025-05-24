// src/commands/start.rs
use crate::logger::{log, Status};

pub fn handle() {
    log(Status::Info, "Starting file watcher...");
    if let Err(e) = watcher::start() {
        log(Status::Fail, &format!("Failed to start watcher: {e}"));
    } else {
        log(Status::OK, "Watcher started successfully.");
    }
}

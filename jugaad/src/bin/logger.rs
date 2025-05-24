// src/logger.rs

use colored::*;

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
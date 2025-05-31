use std::fs;
use std::path::PathBuf;
use crate::error::Result;
use crate::logger;

const PID_FILE: &str = ".github-sync.pid";

pub fn handle() -> Result<()> {
    let pid_file = PathBuf::from(PID_FILE);
    
    if !pid_file.exists() {
        logger::info("GitHub Sync is not running.");
        return Ok(());
    }

    // Read PID from file
    let pid_str = fs::read_to_string(&pid_file)?;
    let pid: i32 = pid_str.trim().parse().map_err(|_| {
        fs::remove_file(&pid_file)?;
        logger::error("Invalid PID file found, cleaned up.");
        return Ok(());
    })?;

    // Send SIGTERM to the process
    #[cfg(unix)]
    {
        use nix::sys::signal::{self, Signal};
        use nix::unistd::Pid;

        match signal::kill(Pid::from_raw(pid), Signal::SIGTERM) {
            Ok(_) => {
                logger::success("GitHub Sync stopped successfully.");
                fs::remove_file(&pid_file)?;
            }
            Err(e) => {
                logger::error(&format!("Failed to stop GitHub Sync: {}", e));
                // Clean up PID file if process doesn't exist
                if e == nix::Error::ESRCH {
                    fs::remove_file(&pid_file)?;
                }
            }
        }
    }

    #[cfg(windows)]
    {
        use windows::Win32::System::Threading::{OpenProcess, TerminateProcess, PROCESS_TERMINATE};
        use windows::Win32::Foundation::{HANDLE, CloseHandle};

        unsafe {
            let handle = OpenProcess(PROCESS_TERMINATE, false, pid as u32);
            if handle.is_ok() {
                if TerminateProcess(handle, 0).is_ok() {
                    logger::success("GitHub Sync stopped successfully.");
                } else {
                    logger::error("Failed to stop GitHub Sync.");
                }
                CloseHandle(handle);
            } else {
                logger::error("Failed to find GitHub Sync process.");
            }
        }
        fs::remove_file(&pid_file)?;
    }

    Ok(())
} 
use std::process::Command;
use crate::error::Result;
use crate::logger;
use dirs;

pub fn handle() -> Result<()> {
    // Get sync root
    let sync_root = dirs::home_dir()
        .expect("Could not find home directory")
        .join(".github-sync");

    // Get git log
    let output = Command::new("git")
        .args([
            "log",
            "--pretty=format:%C(yellow)%h %C(reset)%ad %C(green)%an%C(reset): %s",
            "--date=relative",
            "-n",
            "20"
        ])
        .current_dir(&sync_root)
        .output()?;

    if output.status.success() {
        println!("\n📋 Last 20 sync operations:\n");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    } else {
        logger::error("Failed to get sync history");
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
} 
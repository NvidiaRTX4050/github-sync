use std::{process::Command, thread, time::Duration};
use std::str;

/// Checks if the local repo is behind the remote.
fn is_remote_updated() -> bool {
    let fetch_output = Command::new("git")
        .arg("fetch")
        .output()
        .expect("Failed to fetch from remote");

    if !fetch_output.status.success() {
        eprintln!("git fetch failed: {}", String::from_utf8_lossy(&fetch_output.stderr));
        return false;
    }

    let rev_list_output = Command::new("git")
        .args(["rev-list", "HEAD..origin/main", "--count"])
        .output()
        .expect("Failed to check rev-list");

    if !rev_list_output.status.success() {
        eprintln!("git rev-list failed: {}", String::from_utf8_lossy(&rev_list_output.stderr));
        return false;
    }

    let count_str = String::from_utf8_lossy(&rev_list_output.stdout);
    let count = count_str.trim().parse::<u32>().unwrap_or(0);
    count > 0
}

pub fn start_remote_watcher() {
    thread::spawn(|| loop {
        if is_remote_updated() {
            println!("Remote has new commits. Pulling changes...");
            let pull_output = Command::new("git")
                .arg("pull")
                .output()
                .expect("Failed to pull changes");

            if pull_output.status.success() {
                println!("Changes synced successfully.");
            } else {
                eprintln!("git pull failed: {}", String::from_utf8_lossy(&pull_output.stderr));
            }
        }

        thread::sleep(Duration::from_secs(2));
    });
}

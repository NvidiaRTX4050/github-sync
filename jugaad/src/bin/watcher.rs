// src/bin/watch.rs
use notify::{Config, RecommendedWatcher, RecursiveMode, Result as NotifyResult, Watcher, Event, EventKind};
use std::{
    path::PathBuf,
    process::Command,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

fn main() -> NotifyResult<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: watch <folder_to_watch>");
        std::process::exit(1);
    }

    let path = PathBuf::from(&args[1]);
    if !path.exists() || !path.is_dir() {
        eprintln!("Error: The provided path does not exist or is not a directory.");
        std::process::exit(1);
    }

    println!("📂 Watching folder: {:?}", path);

    let last_event_time = Arc::new(Mutex::new(Instant::now()));
    let path_clone = path.clone();

    // Spawn a thread for syncing logic
    let last_event_time_clone = Arc::clone(&last_event_time);
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));

        let elapsed = {
            let time = last_event_time_clone.lock().unwrap();
            time.elapsed()
        };

        if elapsed >= Duration::from_secs(5) {
            println!("🟢 No activity for 5s. Syncing...");

            // Check if there are changes to commit
            let output = Command::new("git")
                .args(["status", "--porcelain"])
                .current_dir(&path_clone)
                .output()
                .expect("Failed to run git status");

            if !output.stdout.is_empty() {
                let now = chrono::Local::now();
                let message = format!("sync: Auto-update [{}]", now.format("%Y-%m-%d %H:%M:%S"));

                Command::new("git")
                    .args(["add", "."])
                    .current_dir(&path_clone)
                    .status()
                    .expect("Failed to git add");

                Command::new("git")
                    .args(["commit", "-m", &message])
                    .current_dir(&path_clone)
                    .status()
                    .expect("Failed to git commit");

                let push_status = Command::new("git")
                    .args(["push"])
                    .current_dir(&path_clone)
                    .status()
                    .expect("Failed to git push");

                if push_status.success() {
                    println!("✅ Changes pushed at {}", now.format("%Y-%m-%d %H:%M:%S"));
                } else {
                    println!("❌ git push failed. You may need to pull or resolve conflicts.");
                }
            } else {
                println!("🟢 No changes to sync.");
            }

            *last_event_time_clone.lock().unwrap() = Instant::now() + Duration::from_secs(9999); // prevent loop spam
        }
    });

    // Watcher for filesystem events
    let last_event_time_watcher = Arc::clone(&last_event_time);
    let mut watcher: RecommendedWatcher = Watcher::new(
        move |res: Result<Event, notify::Error>| match res {
            Ok(event) => {
                if matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_)) {
                    println!("🟡 File system activity detected.");
                    *last_event_time_watcher.lock().unwrap() = Instant::now();
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        },
        Config::default(),
    )?;

    watcher.watch(&path, RecursiveMode::Recursive)?;
    loop {
        thread::park();
    }
}
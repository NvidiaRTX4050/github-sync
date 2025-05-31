use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};
use std::time::{Duration, Instant, SystemTime};
use std::collections::HashSet;
use std::fs;
use notify::{Watcher as NotifyWatcher, RecursiveMode, Result as NotifyResult, Event, EventKind};
use crate::error::{Result, JugaadError};
use crate::git::GitSync;
use crate::logger;

const BATCH_WINDOW: Duration = Duration::from_secs(2);
const STATUS_FILE: &str = ".github-sync.status";

#[derive(serde::Serialize, serde::Deserialize)]
struct StatusInfo {
    last_sync: SystemTime,
    pending_changes: Vec<String>,
}

pub struct FileWatcher {
    watcher: notify::RecommendedWatcher,
    rx: Receiver<NotifyResult<Event>>,
    git: GitSync,
    watched_paths: HashSet<PathBuf>,
    last_sync: Instant,
}

impl FileWatcher {
    pub fn new(git: GitSync) -> Result<Self> {
        let (tx, rx) = channel();
        
        let watcher = notify::recommended_watcher(move |res| {
            tx.send(res).unwrap_or_else(|e| logger::error(&format!("Watch error: {}", e)));
        })?;

        Ok(Self {
            watcher,
            rx,
            git,
            watched_paths: HashSet::new(),
            last_sync: Instant::now(),
        })
    }

    pub fn watch<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref().to_path_buf();
        if self.watched_paths.insert(path.clone()) {
            logger::watch(&format!("Starting to watch: {}", path.display()));
            self.watcher.watch(&path, RecursiveMode::Recursive)?;
        }
        Ok(())
    }

    pub fn unwatch<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref().to_path_buf();
        if self.watched_paths.remove(&path) {
            logger::watch(&format!("Stopped watching: {}", path.display()));
            self.watcher.unwatch(&path)?;
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        logger::info("File watcher started");
        
        let mut pending_changes = HashSet::new();
        let mut last_event = Instant::now();

        // Initialize status file
        self.update_status(&Vec::new())?;

        loop {
            // Check for new events with a timeout
            match self.rx.recv_timeout(Duration::from_secs(1)) {
                Ok(Ok(event)) => {
                    // Skip git directory changes and status file changes
                    if is_git_path(&event.paths[0]) || is_status_file(&event.paths[0]) {
                        continue;
                    }

                    match event.kind {
                        EventKind::Create(_) => {
                            logger::watch(&format!("Created: {}", event.paths[0].display()));
                            pending_changes.insert(event.paths[0].clone());
                        },
                        EventKind::Modify(_) => {
                            logger::watch(&format!("Modified: {}", event.paths[0].display()));
                            pending_changes.insert(event.paths[0].clone());
                        },
                        EventKind::Remove(_) => {
                            logger::watch(&format!("Removed: {}", event.paths[0].display()));
                            pending_changes.insert(event.paths[0].clone());
                        },
                        _ => continue,
                    }
                    last_event = Instant::now();

                    // Update status file with pending changes
                    let changes: Vec<String> = pending_changes.iter()
                        .map(|p| p.to_string_lossy().into_owned())
                        .collect();
                    self.update_status(&changes)?;
                },
                Ok(Err(e)) => {
                    logger::error(&format!("Watch error: {}", e));
                },
                Err(_) => {
                    // Timeout - check if we should process pending changes
                    if !pending_changes.is_empty() && last_event.elapsed() >= BATCH_WINDOW {
                        self.process_changes(&pending_changes)?;
                        pending_changes.clear();
                        // Clear pending changes in status file
                        self.update_status(&Vec::new())?;
                    }
                }
            }
        }
    }

    fn process_changes(&mut self, changes: &HashSet<PathBuf>) -> Result<()> {
        // Don't sync too frequently
        if self.last_sync.elapsed() < Duration::from_secs(5) {
            logger::info("Skipping sync - too soon since last sync");
            return Ok(());
        }

        logger::sync(&format!("Processing {} changes...", changes.len()));
        
        // Log the changes
        for path in changes {
            logger::info(&format!("  {}", path.display()));
        }

        // Perform the sync
        self.git.sync()?;
        self.last_sync = Instant::now();

        Ok(())
    }

    fn update_status(&self, pending_changes: &Vec<String>) -> Result<()> {
        let status = StatusInfo {
            last_sync: SystemTime::now(),
            pending_changes: pending_changes.clone(),
        };

        let status_json = serde_json::to_string_pretty(&status)
            .map_err(|e| JugaadError::ConfigError(format!("Failed to serialize status: {}", e)))?;

        fs::write(STATUS_FILE, status_json)
            .map_err(|e| JugaadError::ConfigError(format!("Failed to write status file: {}", e)))?;

        Ok(())
    }
}

fn is_git_path(path: &Path) -> bool {
    path.components().any(|c| c.as_os_str() == ".git")
}

fn is_status_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|n| n == STATUS_FILE)
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;
    use std::thread;

    #[test]
    fn test_watcher_setup() {
        let temp = tempdir().unwrap();
        let git = GitSync::new(
            temp.path(),
            "git@github.com:test/repo.git",
            "main"
        ).unwrap();
        
        let mut watcher = FileWatcher::new(git).unwrap();
        watcher.watch(temp.path()).unwrap();
        
        // Create a file and verify it's detected
        let test_file = temp.path().join("test.txt");
        fs::write(&test_file, "test content").unwrap();
        
        // Give the watcher a moment to detect the change
        thread::sleep(Duration::from_millis(100));
        
        // Clean up
        watcher.unwatch(temp.path()).unwrap();
    }
} 
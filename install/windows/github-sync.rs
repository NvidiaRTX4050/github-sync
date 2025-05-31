use windows_service::{
    define_windows_service,
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType,
    },
    service_control_handler::{self, ServiceControlHandler},
    service_dispatcher,
};
use std::{ffi::OsString, time::Duration};
use std::sync::mpsc;

const SERVICE_NAME: &str = "GitHubSync";
const SERVICE_DISPLAY_NAME: &str = "GitHub Sync Service";
const SERVICE_DESCRIPTION: &str = "Two-way file synchronization using Git";

define_windows_service!(ffi_service_main, github_sync_main);

fn github_sync_main(arguments: Vec<OsString>) {
    if let Err(e) = run_service(arguments) {
        // Log error using Windows event logger
        windows_service::Result::Err(e).unwrap();
    }
}

pub fn run_service(_arguments: Vec<OsString>) -> windows_service::Result<()> {
    let (shutdown_tx, shutdown_rx) = mpsc::channel();

    let event_handler = move |control_event| -> ServiceControlHandler {
        match control_event {
            ServiceControl::Stop => {
                shutdown_tx.send(()).unwrap();
                ServiceControlHandler::NoError
            }
            ServiceControl::Interrogate => ServiceControlHandler::NoError,
            _ => ServiceControlHandler::NotImplemented,
        }
    };

    let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;

    status_handle.set_service_status(ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    // Start the GitHub Sync service
    let config = crate::config::Config::load().unwrap_or_default();
    let sync_root = dirs::home_dir()
        .expect("Could not find home directory")
        .join(".github-sync");

    let git = crate::git::GitSync::new(
        &sync_root,
        &config.remote_url,
        &config.branch
    ).expect("Failed to initialize Git sync");

    let mut watcher = crate::watcher::FileWatcher::new(git)
        .expect("Failed to initialize file watcher");

    // Watch the sync directory
    watcher.watch(&sync_root).expect("Failed to start watching directory");

    // Start remote watcher
    crate::remote_watcher::start_remote_watcher();

    // Wait for shutdown signal
    shutdown_rx.recv().unwrap();

    status_handle.set_service_status(ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    Ok(())
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    service_dispatcher::start(SERVICE_NAME, ffi_service_main)?;
    Ok(())
} 
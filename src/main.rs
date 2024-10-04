// Disable console popup on windows
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use std::sync::mpsc;
use std::thread;

use xdwlan_login::config::load_config;
use xdwlan_login::logger::setup_logger;
use xdwlan_login::tasks::{LoginTask, Task};

/// On Windows, the tray task and the login task run in parallel. The tray task is responsible for showing the tray icon and handling user interactions, while the login task is responsible for checking network connectivity and logging in.
#[cfg(target_os = "windows")]
fn run() -> anyhow::Result<()> {
    use xdwlan_login::tasks::TrayTask;

    log::info!("Start.");

    let (tx_login, rx_login) = mpsc::channel();
    let (tx_tray, rx_tray) = mpsc::channel();

    let config = load_config()?;
    let login_task = LoginTask::new(config.username, config.password);
    let login_task_handle = thread::spawn(move || login_task.run(tx_tray, rx_login));

    TrayTask::new().run(tx_login, rx_tray)?;
    login_task_handle.join().unwrap()?;

    Ok(())
}

/// On Linux, the program runs in CLI mode. The login task runs in the thread (if not oneshot) for checking network connectivity and logging in. The main thread is responsible for handling signals and quitting the login task.
#[cfg(target_os = "linux")]
fn run() -> anyhow::Result<()> {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    use xdwlan_login::tasks::AppEvent;

    // Parse command line arguments.
    let args = clap::Command::new("xdwlan-login")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Login to Xidian University's wireless network.")
        .arg(
            clap::Arg::new("oneshot")
                .short('o')
                .long("oneshot")
                .help("Run once and quit.")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    log::info!("Start.");

    let config = load_config()?;
    let login_task = LoginTask::new(config.username, config.password);

    match args.get_one::<bool>("oneshot") {
        Some(true) => {
            log::info!("Running in oneshot mode.");

            // Run `login` method directly in oneshot mode.
            if login_task.is_online() {
                log::info!("You are online.");
                return Ok(());
            }

            loop {
                login_task.login()?;

                // Wait a second for network to be ready.
                thread::sleep(Duration::from_secs(1));
                if login_task.is_online() {
                    log::info!("You are online.");
                    break;
                }

                // Hang up for 5 seconds for next login attempt to avoid being banned.
                thread::sleep(Duration::from_secs(5));
            }
        }
        _ => {
            log::info!("Running in daemon mode.");

            let (tx_login, rx_login) = mpsc::channel();
            let (tx_main, _rx_main) = mpsc::channel();

            // Run `run` method in daemon mode.
            let login_task_handle = thread::spawn(move || login_task.run(tx_main, rx_login));

            let term = Arc::new(AtomicBool::new(false));
            signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&term))?;
            signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term))?;

            while !term.load(Ordering::Relaxed) {
                log::debug!("Main task is running.");
                thread::sleep(Duration::from_secs(1));
            }
            tx_login.send(AppEvent::Quit)?;
            login_task_handle.join().unwrap()?;
        }
    };

    Ok(())
}

fn main() {
    setup_logger();

    if let Err(e) = run() {
        log::error!("{}", e);
    }
    log::info!("Quit.");
}

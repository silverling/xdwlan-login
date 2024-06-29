// Disable console popup on windows
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use std::sync::mpsc;
use std::thread;

use xdwlan_login::config::load_config;
use xdwlan_login::logger::setup_logger;
use xdwlan_login::tasks::{LoginTask, Task, TrayTask};

#[cfg(windows)]
fn run() -> anyhow::Result<()> {
    log::info!("Start.");

    let (tx_login, rx_login) = mpsc::channel();
    let (tx_tray, rx_tray) = mpsc::channel();

    let config = load_config()?;
    let login_task = LoginTask::new(config.username, config.password);
    let login_task_handle = thread::spawn(move || login_task.run(tx_tray, rx_login));

    TrayTask::new().run(tx_login, rx_tray)?;
    login_task_handle.join().unwrap()?;

    log::info!("Quit.");
    Ok(())
}

fn main() {
    setup_logger();
    if let Err(e) = run() {
        log::error!("{}", e);
    }
}

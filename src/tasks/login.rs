use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use super::{AppEvent, Task};

const NODE_ENV: &str = if cfg!(debug_assertions) {
    "development"
} else {
    "production"
};

pub struct LoginTask {
    username: String,
    password: String,
    domain: String,
}

impl LoginTask {
    pub fn new(username: String, password: String, domain: String) -> Self {
        LoginTask {
            username,
            password,
            domain,
        }
    }

    pub fn is_online(&self) -> bool {
        let client = reqwest::blocking::ClientBuilder::new()
            .no_proxy()
            .build()
            .unwrap(); // This method only panics if called from within an async runtime.
        if let Ok(resp) = client.get("http://wifi.vivo.com.cn/generate_204").send() {
            if resp.status().as_u16() == 204 {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    fn get_login_url(&self) -> anyhow::Result<String> {
        let client = reqwest::blocking::ClientBuilder::new().no_proxy().build()?;

        // When you were offline, you will be redirct to the login page.
        // Sometimes, the redirection will fail, so we try at most 5 times.
        for _ in 0..5 {
            let resp = client.get("http://www.baidu.com").send()?;
            let content = resp.text()?;
            if content.contains("w.xidian.edu.cn") {
                let re = regex::Regex::new(
                    r#"(?m)action="(?P<url>https://w\.xidian\.edu\.cn[a-zA-Z0-9./_]+)""#,
                )?;
                if let Some(cap) = re.captures(&content) {
                    return Ok(cap["url"].to_string());
                }
            }
        }

        Err(anyhow::anyhow!("Login url not found.").into())
    }

    /// Open a browser and login to the network.
    pub fn login(&self) -> anyhow::Result<()> {
        let url = self.get_login_url()?;
        log::info!("Got login url: {}", url);

        let program_folder = crate::utils::get_program_folder();

        #[cfg(target_os = "windows")]
        let login_exe = program_folder.join("xdwlan-login-worker.exe");

        #[cfg(target_os = "linux")]
        let login_exe = program_folder.join("xdwlan-login-worker");

        let output = std::process::Command::new(login_exe)
            .env("NODE_ENV", NODE_ENV)
            .env("XDWLAN_LOGIN_URL", &url)
            .env("XDWLAN_USERNAME", &self.username)
            .env("XDWLAN_PASSWORD", &self.password)
            .env("XDWLAN_DOMAIN", &self.domain)
            .output()?;

        if output.status.success() {
            return Ok(());
        }

        anyhow::bail!("Login process exited with code {}", output.status);
    }
}

impl Task for LoginTask {
    fn run(&self, _sender: Sender<AppEvent>, receiver: Receiver<AppEvent>) -> anyhow::Result<()> {
        log::debug!("Login task started.");
        log::debug!(
            "Use username: {} and password: {}",
            self.username,
            self.password
        );

        // Sleep seconds and wake up when receive a message.
        let should_quit = |seconds: u64| {
            if let Ok(AppEvent::Quit) = receiver.recv_timeout(Duration::from_secs(seconds)) {
                return true;
            } else {
                return false;
            }
        };

        let simulate = || {
            log::info!("You are offline now.");

            loop {
                if let Err(e) = self.login() {
                    log::error!("{}", e);
                }

                // Wait a second for network to be ready.
                if should_quit(1) {
                    return;
                }

                if self.is_online() {
                    log::info!("Login successfully.");
                    break;
                }

                // Hang up for 5 seconds for next login attempt to avoid being banned.
                if should_quit(5) {
                    return;
                }
            }
        };

        // Check the network status at first.
        if self.is_online() {
            log::info!("You are already online.");
        } else {
            simulate();
        }

        loop {
            if should_quit(60) {
                return Ok(());
            }

            if !self.is_online() {
                simulate();
            }
        }
    }
}

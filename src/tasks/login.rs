use std::time::Duration;

use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::sync::mpsc::{Receiver, Sender};

use super::{AppEvent, Task};

pub struct LoginTask {
    username: String,
    password: String,
}

impl LoginTask {
    pub fn new(username: String, password: String) -> Self {
        LoginTask { username, password }
    }

    fn is_online(&self) -> bool {
        if let Ok(resp) = reqwest::blocking::get("http://wifi.vivo.com.cn/generate_204") {
            if resp.status().as_u16() == 204 {
                log::debug!("You are online.");
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    // In debug mode, we disable headless mode to see what's happening.
    #[cfg(debug_assertions)]
    fn create_browser(&self) -> anyhow::Result<Browser> {
        let browser = Browser::new(LaunchOptionsBuilder::default().headless(false).build()?)?;

        Ok(browser)
    }

    #[cfg(not(debug_assertions))]
    fn create_browser(&self) -> anyhow::Result<Browser> {
        use std::env::temp_dir;

        let user_data_dir = temp_dir().join("xdwlan-login");

        if !user_data_dir.exists() {
            std::fs::create_dir(&user_data_dir)?;
            log::info!("User data dir: {}", user_data_dir.display());
        }

        let browser = Browser::new(
            LaunchOptionsBuilder::default()
                .user_data_dir(Some(user_data_dir))
                .build()?,
        )?;

        Ok(browser)
    }

    /// Open a browser and login to the network.
    pub fn login(&self) -> anyhow::Result<()> {
        let browser = self.create_browser()?;

        // Create a new tab.
        let tab = browser.new_tab()?;

        // When you were offline, you will be redirct to the login page.
        // Sometimes, the redirection will fail, so we try at most 5 times.
        for i in 0..6 {
            if let Err(e) = tab.navigate_to("http://www.msftconnecttest.com/redirect") {
                log::debug!("Navigate Error: {}", e);

                if i == 5 {
                    return Err(anyhow::anyhow!("Navigate failed for 5 times."));
                }
            }
        }
        tab.wait_until_navigated()?;

        // We check the title of the page to determine whether we are redirected to the login page.
        let url = tab.get_url();
        if url.contains("w.xidian.edu.cn") {
            log::debug!("You are redirected to the login page {}", url);

            // Sometimes, the page will show a dialog says "Net Error".
            // Actually I don't know why, just reload the page to avoid it.
            // Page has to be reload with cache, otherwise it will always complain "Net Error".
            tab.reload(false, None)?;
            tab.wait_until_navigated()?;

            // We try to login here.
            log::info!("Try to login...");
            let body = tab.wait_for_element("body")?;
            body.call_js_fn(
                r#"function login() {
                    if (document.querySelector('div.control > button.btn-confirm')) {
                        document.querySelector('div.control > button.btn-confirm').click();
                    }
                    document.querySelector('#username').value = 'username_placeholder';
                    document.querySelector('#password').value = 'password_placeholder';
                    document.querySelector('#login-account').click();
                }"#
                .replace("username_placeholder", &self.username)
                .replace("password_placeholder", &self.password)
                .as_str(),
                vec![],
                false,
            )?;
        } else {
            log::debug!("Unknown page url: {}", url);
        }

        Ok(())
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

        loop {
            if should_quit(60) {
                return Ok(());
            }

            if self.is_online() {
                continue;
            }

            log::info!("You are offline now.");
            loop {
                if let Err(e) = self.login() {
                    log::error!("{}", e);
                }

                // Wait a second for network to be ready.
                if should_quit(1) {
                    return Ok(());
                }

                if self.is_online() {
                    log::info!("Login successfully.");
                    break;
                }

                // Hang up for 5 seconds for next login attempt to avoid being banned.
                if should_quit(5) {
                    return Ok(());
                }
            }
        }
    }
}

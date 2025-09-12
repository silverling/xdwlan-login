use std::{path::PathBuf, process::Stdio};

use crate::utils::{get_available_port, get_cache_folder, get_program_folder};

pub enum IPCMessage {
    Quit,
}

pub struct LoginServer {
    url: String,
    handle: Option<std::thread::JoinHandle<()>>,
    request: reqwest::blocking::Client,
}

impl LoginServer {
    pub fn new() -> Self {
        // Start the login server in a background thread, listen on an available random port.
        let port = get_available_port().unwrap();
        let url = format!("http://localhost:{}", port);
        let handle = std::thread::spawn(move || {
            let mut command = std::process::Command::new(dump_server_exe().unwrap());
            command.current_dir(get_program_folder());
            command.env("XDWLAN_LOGIN_SERVER_PORT", port.to_string());
            command.env(
                "XDWLAN_LOGIN_CONFIG_PATH",
                get_program_folder().join("config.yaml"),
            );

            // Hide console window in release mode only
            #[cfg(not(debug_assertions))]
            {
                use std::os::windows::process::CommandExt;
                const CREATE_NO_WINDOW: u32 = 0x08000000;
                command.creation_flags(CREATE_NO_WINDOW);

                // In release mode, log to file instead of console
                command.env(
                    "XDWLAN_LOGIN_LOG_PATH",
                    get_program_folder().join("log.txt"),
                );
            }

            // Spawn the login server process
            let mut child = command
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("Failed to start login server.");

            // Pipe stdout to main thread
            if let Some(stdout) = child.stdout.take() {
                let stdout_handle = std::thread::spawn(move || {
                    use std::io::{BufRead, BufReader};
                    let reader = BufReader::new(stdout);
                    for line in reader.lines() {
                        if let Ok(line) = line {
                            println!("{}", line);
                        }
                    }
                });
                let _ = stdout_handle.join();
            }

            // Pipe stderr to main thread
            if let Some(stderr) = child.stderr.take() {
                let stderr_handle = std::thread::spawn(move || {
                    use std::io::{BufRead, BufReader};
                    let reader = BufReader::new(stderr);
                    for line in reader.lines() {
                        if let Ok(line) = line {
                            eprintln!("{}", line);
                        }
                    }
                });
                let _ = stderr_handle.join();
            }

            child.wait().expect("Failed to wait on login server.");
        });
        let request = reqwest::blocking::Client::new();

        LoginServer {
            url,
            handle: Some(handle),
            request,
        }
    }

    pub fn notify(&self, message: IPCMessage) -> anyhow::Result<String> {
        match message {
            IPCMessage::Quit => {
                let response = self.request.post(format!("{}/quit", self.url)).send()?;
                Ok(response.text()?)
            }
        }
    }

    pub fn exit(&mut self) -> anyhow::Result<()> {
        self.notify(IPCMessage::Quit)?;
        self.handle.take().unwrap().join().ok();
        Ok(())
    }
}

fn dump_server_exe() -> anyhow::Result<PathBuf> {
    const EXE: &[u8] =
        include_bytes!("../../../packages/xdwlan-login/build/xdwlan-login-windows-server.exe");
    let path = get_cache_folder().join("xdwlan-login-server.exe");
    println!("Dumping server exe to {:?}", path);
    std::fs::write(&path, EXE)?;
    Ok(path)
}

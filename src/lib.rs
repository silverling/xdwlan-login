pub mod config;
pub mod logger;
pub mod utils;

pub mod tasks {
    mod task;
    pub use task::{AppEvent, Task};

    mod login;
    pub use login::LoginTask;

    #[cfg(windows)]
    mod tray;
    #[cfg(windows)]
    pub use tray::TrayTask;
}

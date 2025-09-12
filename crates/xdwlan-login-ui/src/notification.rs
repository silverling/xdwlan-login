use crate::utils::get_cache_folder;
use std::path::PathBuf;
use tauri_winrt_notification::{Duration, IconCrop, Toast};

pub struct Notification {
    pub icon: PathBuf,
}

impl Notification {
    pub fn new() -> Self {
        Notification {
            icon: dump_icon().unwrap(),
        }
    }

    pub fn show(&self, text: &str) {
        Toast::new("Windows.SystemToast.HelloFace")
            .title("西电校园网登录助手")
            .text1(text)
            .duration(Duration::Short)
            .icon(&self.icon, IconCrop::Square, "")
            .sound(None)
            .show()
            .expect("Failed to show notification");
    }
}

fn dump_icon() -> anyhow::Result<PathBuf> {
    const ICON: &[u8] = include_bytes!("../../../resources/icons/avocado.ico");
    let path = get_cache_folder().join("app-icon.ico");
    std::fs::write(&path, ICON)?;
    Ok(path)
}

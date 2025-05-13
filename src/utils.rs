use std::{env, path::PathBuf};

pub fn get_program_path() -> PathBuf {
    let program_path = env::current_exe().expect("Failed to get current executable path");
    dunce::canonicalize(program_path).expect("Failed to canonicalize path")
}

pub fn get_program_folder() -> PathBuf {
    get_program_path()
        .parent()
        .expect("The program has no parent folder.")
        .to_path_buf()
}

#[cfg(target_os = "windows")]
const REG_KEY_NAME: &str = "Xidian WLAN Login";

#[cfg(target_os = "windows")]
pub fn is_autostart() -> bool {
    let hkcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let reg = hkcu.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run");
    if reg.is_err() {
        return false;
    }

    let path: Result<String, _> = reg.unwrap().get_value(REG_KEY_NAME);
    if path.is_err() {
        return false;
    }

    if path.unwrap() == get_program_path().to_str().unwrap() {
        return true;
    } else {
        return false;
    }
}

#[cfg(target_os = "windows")]
pub fn toggle_autostart() -> anyhow::Result<bool> {
    let hkcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let (reg, _) = hkcu.create_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run")?;
    let current_status = is_autostart();
    if current_status {
        reg.delete_value(REG_KEY_NAME)?;
        log::debug!("Disabled autostart.")
    } else {
        reg.set_value(REG_KEY_NAME, &get_program_path().to_str().unwrap())?;
        log::debug!("Enabled autostart.")
    }

    Ok(!current_status)
}

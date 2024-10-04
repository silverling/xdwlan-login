pub fn get_program_path() -> String {
    std::env::current_exe()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

pub fn get_program_folder() -> String {
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
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

    if path.unwrap() == get_program_path() {
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
        reg.set_value(REG_KEY_NAME, &get_program_path())?;
        log::debug!("Enabled autostart.")
    }

    Ok(!current_status)
}

use std::io::ErrorKind;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub username: String,
    pub password: String,
}

pub fn load_config() -> anyhow::Result<Config> {
    // On windows, the config file is expected to be in the same folder as the executable.
    #[cfg(target_os = "windows")]
    let config_file_path = format!("{}/config.yaml", crate::utils::get_program_folder());

    // On linux, the config file is expected to be in the $XDG_CONFIG_HOME/xdwlan-login folder.
    #[cfg(target_os = "linux")]
    let config_file_path = format!(
        "{}/xdwlan-login/config.yaml",
        dirs::config_dir().unwrap().to_str().unwrap()
    );

    let config = match std::fs::read_to_string(&config_file_path) {
        Ok(config) => config,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            return Err(anyhow::anyhow!(
                "Config file {} not found. Please create one.",
                config_file_path
            )
            .into());
        }
        Err(e) => {
            return Err(e.into());
        }
    };
    let config: Config = serde_yaml::from_str(&config)?;

    Ok(config)
}

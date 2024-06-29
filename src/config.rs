use serde::Deserialize;

use crate::utils::get_program_folder;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub username: String,
    pub password: String,
}

pub fn load_config() -> anyhow::Result<Config> {
    let program_folder = get_program_folder();
    let config_file_path = format!("{}/config.yaml", program_folder);
    let config = std::fs::read_to_string(config_file_path)?;
    let config: Config = serde_yaml::from_str(&config)?;

    Ok(config)
}

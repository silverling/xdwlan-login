use std::io::ErrorKind;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub username: String,
    pub password: String,
    #[serde(default)]
    pub domain: String,
}

pub fn load_config() -> anyhow::Result<Config> {
    // On windows and linux, the config file is expected to be in the same folder as the executable.
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    let config_file_path = crate::utils::get_program_folder().join("config.yaml");

    let config = match std::fs::read_to_string(&config_file_path) {
        Ok(config) => config,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            return Err(anyhow::anyhow!(
                "Config file {} not found. Please create one.",
                config_file_path.display()
            )
            .into());
        }
        Err(e) => {
            return Err(e.into());
        }
    };
    let config: Config = serde_yaml::from_str(&config)?;

    match config.domain.as_str() {
        "" | "@dx" | "@lt" | "@yd" => Ok(config),
        _ => Err(anyhow::anyhow!(
            "Invalid domain config: {}, available options are {} or leave it empty.",
            config.domain,
            vec!["@dx", "@lt", "@yd"].join(", ")
        )
        .into()),
    }
}

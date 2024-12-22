use crate::constants::CONFIG_LOCATION;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Config(IO({0:?}))")]
    Io(#[from] std::io::Error),

    #[error("Config(Json({0}))")]
    Json(#[from] serde_json::Error),
}
type Result<T> = std::result::Result<T, ConfigError>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub log: LogConfig,
    pub sql: SqlConfig,
    pub web: WebConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogConfig {
    pub level: String,
    pub dir: Option<String>,
    pub stdout: bool,
    pub fileskept: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SqlConfig {
    pub location: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebConfig {
    pub listen: String,
    pub https: Option<bool>,
    pub key_file: Option<String>,
    pub privkey_file: Option<String>,
}

pub fn read_config() -> Result<Config> {
    let path = get_config_location();

    let content = std::fs::read_to_string(path)?;

    let config: Config = serde_json::from_str(&content)?;

    Ok(config)
}

pub fn get_config_location() -> PathBuf {
    Path::new(crate::constants::CONFIG_LOCATION).to_path_buf()
}

// Checks if the config exists, and returns false if it didn't
pub fn check_config_exists() -> Result<bool> {
    if !std::fs::exists(CONFIG_LOCATION)? {
        write_default_config(CONFIG_LOCATION)?;
        Ok(false)
    } else {
        Ok(true)
    }
}

pub fn write_default_config(path: &str) -> Result<()> {
    let config = Config {
        log: LogConfig {
            level: "info".to_string(),
            dir: None,
            stdout: true,
            fileskept: None,
        },
        sql: SqlConfig {
            location: "spord-tracker.db".to_string(),
        },
        web: WebConfig {
            listen: "127.0.0.1:8080".to_string(),
            https: Some(false),
            key_file: Some("key.crt".to_string()),
            privkey_file: Some("privkey.key".to_string()),
        },
    };

    let config_content = serde_json::to_string_pretty(&config)?;
    let mut file = File::create(path)?;
    file.write_all(config_content.as_bytes())?;

    Ok(())
}

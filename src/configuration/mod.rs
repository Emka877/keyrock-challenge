use std::error::Error;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use ron::de::from_reader;
use serde::Deserialize;

use crate::exceptions::ConfigurationReadError;

/// App configuration data
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    /// The default currencies pair to fetch from the exchanges
    pub currency_pair: String,
}

pub fn read_configuration_file() -> Result<AppConfig, ConfigurationReadError> {
    let path: PathBuf = PathBuf::from("config/config.ron");

    if !path.exists() {
        return Err(ConfigurationReadError::new("The path to the configuration file doesn't exist."));
    }

    let file_open_result = File::open(path);

    if file_open_result.is_err() {
        return Err(ConfigurationReadError::new("Make sure the configuration file exists in the config folder."));
    }

    let file_handle = file_open_result.unwrap();

    let app_config: AppConfig = match from_reader(file_handle) {
        Ok(config) => config,
        Err(error) => {
            return Err(
                ConfigurationReadError::from(error.to_string())
            );
        }
    };

    Ok(app_config)
}

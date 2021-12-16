use lazy_static::lazy_static;
use crate::exceptions::ConfigurationReadError;
use super::datastructs::AppConfig;
use super::functions::read_configuration_file;

/// Initializes the app runtime configuration
///
/// Returns an AppConfig instance. If the configuration wasn't found, bails.
fn initialize_config_store() -> AppConfig {
    let app_config: Result<AppConfig, ConfigurationReadError> = read_configuration_file();

    // TODO: handle this more gracefully, so the app doesn't just bail.
    if app_config.is_err() {
        std::process::exit(1);
    }

    app_config.unwrap()
}

// Store containing the live app configuration
lazy_static!(
    pub static ref APP_CONFIG: AppConfig = initialize_config_store();
);

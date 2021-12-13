use crate::configuration::{AppConfig, read_configuration_file};
use crate::exceptions::ConfigurationReadError;

#[test]
pub fn test_read_configuration() {
    let app_config: Result<AppConfig, ConfigurationReadError> = read_configuration_file();

    assert_ne!(app_config.is_err(), true);
    assert_ne!(app_config.unwrap().currency_pair, "".to_owned());
}
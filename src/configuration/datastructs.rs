use serde::Deserialize;

/// App configuration data
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    /// The default currencies pair to fetch from the exchanges
    pub currency_pair: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            currency_pair: "ethbtc".to_owned(),
        }
    }
}

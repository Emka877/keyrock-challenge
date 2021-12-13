use serde::Deserialize;

/// App configuration data
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    /// The default currencies pair to fetch from the exchanges
    pub currency_pair: String,
}

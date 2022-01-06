use serde::Deserialize;

/// App configuration data
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    /// The default currencies pair to fetch from the exchanges
    pub currency_pair: String,
    // Limits the amount of entries of the final merged OrderBook (x per asks and bids)
    pub orderbook_entries_limit: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            currency_pair: "ethbtc".to_owned(),
            orderbook_entries_limit: 10,
        }
    }
}

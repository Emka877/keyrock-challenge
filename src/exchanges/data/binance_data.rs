use serde::Deserialize;

/// Represents data coming from the Binance Exchange
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct BinanceData {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: serde_json::Number,
    pub bids: Vec<Vec<String>>,
    pub asks: Vec<Vec<String>>,
}

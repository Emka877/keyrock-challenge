use serde::Deserialize;
use tungstenite::Message;
use crate::ExchangeWsTcpStream;
use async_trait::async_trait;
use crate::exceptions::StreamSubscriptionError;

/// Represents data coming from the Binance Exchange
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct BinanceData {
    #[serde(skip)]
    exchange: String,
    #[serde(rename = "lastUpdateId")]
    last_update_id: serde_json::Number,
    bids: Vec<Vec<String>>,
    asks: Vec<Vec<String>>,
}

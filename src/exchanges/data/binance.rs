use serde::Deserialize;
use tungstenite::Message;
use crate::exchanges::data::trait_exchange::Exchange;
use crate::ExchangeWsTcpStream;
use async_trait::async_trait;
use crate::exceptions::StreamSubscriptionError;

/// Represents the Binance exchange
pub struct Binance;

#[async_trait]
impl Exchange for Binance {
    async fn subscribe_to_orderbook_stream(active_stream: &mut ExchangeWsTcpStream) -> Result<Option<Message>, StreamSubscriptionError> {
        // Binance's URL already connects the stream to the right subscription
        Ok(None)
    }
}

/// Represents data coming from the Binance Exchange
#[derive(Debug, Clone, Deserialize)]
pub struct BinanceData {
    #[serde(skip)]
    exchange: String,
    #[serde(rename = "lastUpdateId")]
    last_update_id: serde_json::Number,
    bids: Vec<Vec<String>>,
    asks: Vec<Vec<String>>,
}

// impl Default for BinanceData {
//     fn default() -> Self {
//         Self {
//             exchange: "binance".to_owned(),
//             last_update_id: serde_json::Number::from(0),
//             bids: vec!(),
//             asks: vec!(),
//         }
//     }
// }

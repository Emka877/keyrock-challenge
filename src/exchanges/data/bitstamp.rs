use std::fmt::Formatter;
use serde::Deserialize;
use tungstenite::Message;
use crate::exceptions::StreamSubscriptionError;
use crate::exchanges::data::trait_exchange::Exchange;
use crate::ExchangeWsTcpStream;
use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use crate::configuration::APP_CONFIG;

/// Represents the Bitstamp exchange
pub struct Bitstamp;

#[async_trait]
impl Exchange for Bitstamp {
    async fn subscribe_to_orderbook_stream(active_stream: &mut ExchangeWsTcpStream) -> Result<Option<Message>, StreamSubscriptionError> {
        let currency_pair = format!("order_book_{}", APP_CONFIG.currency_pair.clone());
        let subscription_data: String = format!(r#"{{
            "event": "bts:subscribe",
            "data": {{"channel": "{}"}}
        }}"#, currency_pair);
        let bitstamp_subscribe_message = active_stream.send(Message::Text(subscription_data)).await;

        if bitstamp_subscribe_message.is_err() {
            return Err(StreamSubscriptionError::from("Could not subscribe to the Bitstamp stream"));
        }
        Ok(Some(active_stream.next().await.unwrap().unwrap()))
    }
}

/// Represents data coming from the Bitstamp Exchange
#[derive(Debug, Clone, Deserialize)]
pub struct BitstampData {
    data: BitstampExtraData,
    channel: String,
    event: String,
}

/*** Bitstamp data composition members are below ***/

#[derive(Debug, Clone, Deserialize)]
pub struct BitstampExtraData {
    timestamp: String,
    microtimestamp: String,
    bids: Vec<Vec<String>>,
    asks: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BitstampSubscription {
    event: String,
    channel: String,
    #[serde(skip_deserializing)]
    data: (),
}

use serde::Deserialize;
use tungstenite::Message;
use crate::exceptions::StreamSubscriptionError;
use crate::ExchangeWsTcpStream;
use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use crate::configuration::APP_CONFIG;

/// Represents data coming from the Bitstamp Exchange
#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct BitstampData {
    data: BitstampExtraData,
    channel: String,
    event: String,
}

/*** Bitstamp data composition members are below ***/

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct BitstampExtraData {
    timestamp: String,
    microtimestamp: String,
    bids: Vec<Vec<String>>,
    asks: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct BitstampSubscription {
    event: String,
    channel: String,
    #[serde(skip_deserializing)]
    data: (),
}

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
    pub data: BitstampExtraData,
    pub channel: String,
    pub event: String,
}

/*** Bitstamp data composition members are below ***/

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct BitstampExtraData {
    pub timestamp: String,
    pub microtimestamp: String,
    pub bids: Vec<Vec<String>>,
    pub asks: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct BitstampSubscription {
    pub event: String,
    pub channel: String,
    #[serde(skip_deserializing)]
    pub data: (),
}

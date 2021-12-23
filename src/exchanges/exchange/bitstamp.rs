use async_trait::async_trait;
use futures::{SinkExt, StreamExt};
use tungstenite::Message;
use crate::{Exchange, ExchangeWsTcpStream};
use crate::configuration::APP_CONFIG;
use crate::exceptions::StreamSubscriptionError;

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
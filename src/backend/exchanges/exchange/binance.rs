use tungstenite::Message;
use crate::exceptions::StreamSubscriptionError;
use crate::ExchangeWsTcpStream;
use super::Exchange;
use async_trait::async_trait;

/// Represents the Binance exchange
pub struct Binance;

#[async_trait]
impl Exchange for Binance {
    async fn subscribe_to_orderbook_stream(_: &mut ExchangeWsTcpStream) -> Result<Option<Message>, StreamSubscriptionError> {
        // Binance's URL already connects the stream to the right subscription
        Ok(None)
    }
}

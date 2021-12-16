use tungstenite::Message;
use crate::ExchangeWsTcpStream;
use async_trait::async_trait;
use crate::exceptions::StreamSubscriptionError;

#[async_trait]
pub trait Exchange {
    async fn subscribe_to_orderbook_stream(active_stream: &mut ExchangeWsTcpStream) -> Result<Option<Message>, StreamSubscriptionError>;
}
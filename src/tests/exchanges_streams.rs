use futures::SinkExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream};
use tungstenite::Message;

use crate::{exchanges::{ExchangeEndpoint, open_stream_to_exchange}, exceptions::OpenStreamError};


/// Only tests opening a Stream to an Exchange, then closes it immediately
#[tokio::test]
pub async fn test_open_stream_to_all_exchanges() {
    // Setup the streams
    let open_bitstamp_stream_result: Result<WebSocketStream<MaybeTlsStream<TcpStream>>, OpenStreamError> = open_stream_to_exchange(ExchangeEndpoint::Bitstamp).await;
    let open_binance_stream_result: Result<WebSocketStream<MaybeTlsStream<TcpStream>>, OpenStreamError> = open_stream_to_exchange(ExchangeEndpoint::Binance).await;

    // Check they're good to go
    assert_ne!(open_bitstamp_stream_result.is_err(), true);
    assert_ne!(open_binance_stream_result.is_err(), true);

    // Close the streams
    let _ = open_bitstamp_stream_result.unwrap().send(Message::Close(None)).await;
    let _ = open_binance_stream_result.unwrap().send(Message::Close(None)).await;
}

use crate::{exchanges::{
    ExchangeEndpoint,
    open_stream_to_exchange,
}, exceptions::OpenStreamError, ExchangeWsTcpStream, Bitstamp, Binance};
use crate::exchanges::Exchange;
use futures::{SinkExt, StreamExt};
use tungstenite::Message;
use crate::exceptions::StreamSubscriptionError;


/// Only tests opening a Stream to an Exchange, then closes it immediately
#[tokio::test]
pub async fn test_open_stream_to_all_exchanges() {
    // Setup the streams
    let open_bitstamp_stream_result: Result<ExchangeWsTcpStream, OpenStreamError> = open_stream_to_exchange(ExchangeEndpoint::Bitstamp).await;
    let open_binance_stream_result: Result<ExchangeWsTcpStream, OpenStreamError> = open_stream_to_exchange(ExchangeEndpoint::Binance).await;

    // Check they're good to go
    assert!(!open_bitstamp_stream_result.is_err());
    assert!(!open_binance_stream_result.is_err());
}

#[tokio::test]
pub async fn test_subscribing_to_exchange_orderbook() {
    // Setup the streams
    let open_bitstamp_stream_result: Result<ExchangeWsTcpStream, OpenStreamError> = open_stream_to_exchange(ExchangeEndpoint::Bitstamp).await;
    let open_binance_stream_result: Result<ExchangeWsTcpStream, OpenStreamError> = open_stream_to_exchange(ExchangeEndpoint::Binance).await;

    // Check they're good to go
    assert!(!open_bitstamp_stream_result.is_err());
    assert!(!open_binance_stream_result.is_err());

    // Unwrap the actual streams
    let mut bitstamp_stream: ExchangeWsTcpStream = open_bitstamp_stream_result.unwrap();
    let mut binance_stream: ExchangeWsTcpStream = open_binance_stream_result.unwrap();

    // Try subscribing to both exchanges' orderbooks
    let bitstamp_response: Result<Option<Message>, StreamSubscriptionError> = Bitstamp::subscribe_to_orderbook_stream(&mut bitstamp_stream).await;
    let binance_response: Result<Option<Message>, StreamSubscriptionError> = Binance::subscribe_to_orderbook_stream(&mut binance_stream).await;

    // Check for subscription request error
    assert!(!bitstamp_response.is_err());
    assert!(!binance_response.is_err());

    let bitstamp_message = bitstamp_response.unwrap();
    if bitstamp_message.is_some() {
        println!("{:#?}", bitstamp_message.as_ref().unwrap());
    }

    let binance_message = binance_response.unwrap();
    if binance_message.is_some() {
        println!("{:#?}", binance_message.as_ref().unwrap());
    }
}
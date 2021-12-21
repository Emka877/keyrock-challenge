use crate::{exchanges::{
    ExchangeEndpoint,
    open_stream_to_exchange,
}, exceptions::OpenStreamError, ExchangeWsTcpStream, Bitstamp, Binance, NormalizedExchangeData, Summary};
use crate::exchanges::Exchange;
use futures::{SinkExt, StreamExt};
use tungstenite::Message;
use crate::exceptions::StreamSubscriptionError;
use super::common::*;


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

/// Tests the correctness of the sorting method (bids high to low, asks low to high), from mock data.
#[test]
pub fn test_data_sorting_correctness() {
    // Read the mock data (actually coming from both Bitstamp and Binance)
    let mut data: Summary = read_all_mock_data();
    // Check if we got enough data (100 from Bitstamp, 20 from Binance)
    assert_eq!(data.get_asks().len(), 120);
    assert_eq!(data.get_bids().len(), 120);

    // Sort
    // Simulate data going to the gRPC server
    data.prepare();

    // Test if sorting is ok (Bids from highest to lowest, asks from lowest to highest)
    let mut prev_price: Option<f32> = None;
    for ask in data.get_asks().iter() {
        if prev_price.is_none() {
            prev_price = Some(ask.price);
            continue;
        }
        assert!(prev_price.is_some());
        assert!(prev_price.unwrap() <= ask.price);
        prev_price = Some(ask.price);
    }

    prev_price = None;
    for bid in data.get_bids().iter() {
        if prev_price.is_none() {
            prev_price = Some(bid.price);
            continue;
        }
        assert!(prev_price.is_some());
        assert!(prev_price.unwrap() >= bid.price);
        prev_price = Some(bid.price);
    }

    // Check if we output enough data
    assert_eq!(data.get_asks().len(), 10);
    assert_eq!(data.get_bids().len(), 10);
}

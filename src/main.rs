#[cfg(test)]
mod tests;

mod exceptions;
mod configuration;
/// This module contains everything related with streaming the final orderbook to the world
mod grpc_server;
/// Contains everything related to fetching the data from the remote exchanges
mod exchanges;
/// This module contains the methods used to merge the final orderbook
mod orderbook;

use exchanges::{open_stream_to_exchange, ExchangeEndpoint};
use futures::StreamExt;

use crate::exchanges::{Binance, BinanceData, Bitstamp, BitstampData, Exchange, ExchangeWsTcpStream};

#[tokio::main]
async fn main() {
    let mut bitstamp = open_stream_to_exchange(ExchangeEndpoint::Bitstamp).await.expect("Cannot open stream to Bitstamp.");
    let _ = Bitstamp::subscribe_to_orderbook_stream(&mut bitstamp).await;
    let mut binance = open_stream_to_exchange(ExchangeEndpoint::Binance).await.expect("Cannot open stream to Binance.");
    let _ = Binance::subscribe_to_orderbook_stream(&mut binance).await;

    // TODO: !!Error handling in stream reading.
    loop {
        let bitstamp_message = bitstamp.next().await;
        if bitstamp_message.is_some() && bitstamp_message.as_ref().unwrap().is_ok() {
            // Shadowing previous var
            let bitstamp_message = bitstamp_message.unwrap().unwrap();
            let bitstamp_data: BitstampData = serde_json::from_str(bitstamp_message.to_string().as_str()).unwrap();
            println!("{:?}", bitstamp_data);
        }

        let binance_message = binance.next().await;
        if binance_message.is_some() && binance_message.as_ref().unwrap().is_ok() {
            let binance_message = binance_message.unwrap().unwrap();
            let binance_data: BinanceData = serde_json::from_str(binance_message.to_text().unwrap()).unwrap();
            println!("{:?}", binance_data);
        }
    }
}

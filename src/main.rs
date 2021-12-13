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

#[tokio::main]
async fn main() {
    let mut bitstamp_stream = open_stream_to_exchange(ExchangeEndpoint::Bitstamp).await.expect("Cannot open stream to Bitstamp");
    let mut binance_stream = open_stream_to_exchange(ExchangeEndpoint::Binance).await.expect("Could open stream to Binance");
    
    let _ = binance_stream.close(None).await;
    let _ = bitstamp_stream.close(None).await;
}

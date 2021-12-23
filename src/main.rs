#[cfg(test)]
mod tests;

mod exceptions;
mod configuration;
/// This module contains everything related with streaming the final orderbook to the world
mod grpc;
/// Contains everything related to fetching the data from the remote exchanges
mod exchanges;
/// Persists the merged orderbook in RAM, in order to serve it via the gRPC server
mod persistence;

use std::net::SocketAddr;
use exchanges::{open_stream_to_exchange, ExchangeEndpoint};
use futures::StreamExt;
use tonic::transport::{Server, ServerTlsConfig};
use crate::configuration::APP_CONFIG;

use crate::exchanges::{Binance, BinanceData, Bitstamp, BitstampData, Exchange, ExchangeWsTcpStream};
use crate::grpc::orderbook::{NormalizedExchangeData, LocalSummary};
use crate::grpc::server::orderbook::orderbook_aggregator_server::OrderbookAggregatorServer;
use crate::grpc::server::server::OrderbookServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bitstamp = open_stream_to_exchange(ExchangeEndpoint::Bitstamp).await.expect("Cannot open stream to Bitstamp.");
    let _ = Bitstamp::subscribe_to_orderbook_stream(&mut bitstamp).await;
    let mut binance = open_stream_to_exchange(ExchangeEndpoint::Binance).await.expect("Cannot open stream to Binance.");
    let _ = Binance::subscribe_to_orderbook_stream(&mut binance).await;

    // Start the data gathering thread
    tokio::spawn(async move {
        dbg!("Start gathering data from exchanges.");
        loop {
            let mut final_orderbook: LocalSummary = LocalSummary::new(APP_CONFIG.currency_pair.clone().as_str());

            let binance_message = binance.next().await;
            if binance_message.is_some() && binance_message.as_ref().unwrap().is_ok() {
                let binance_message = binance_message.unwrap().unwrap();
                if binance_message.is_empty() {
                    continue;
                }
                let binance_parse_result: Result<BinanceData, serde_json::error::Error> = serde_json::from_str(binance_message.to_text().unwrap());
                if binance_parse_result.is_err() {
                    continue;
                }
                let binance_data: BinanceData = binance_parse_result.unwrap();
                let normalized: NormalizedExchangeData = binance_data.into();
                // println!("{:?}", normalized);

                final_orderbook.push(normalized);
            }

            let bitstamp_message = bitstamp.next().await;
            if bitstamp_message.is_some() && bitstamp_message.as_ref().unwrap().is_ok() {
                // Shadowing previous var
                let bitstamp_message = bitstamp_message.unwrap().unwrap();
                if bitstamp_message.is_empty() {
                    continue;
                }
                let bitstamp_parse_result: Result<BitstampData, serde_json::error::Error> = serde_json::from_str(bitstamp_message.to_string().as_str());
                if bitstamp_parse_result.is_err() {
                    continue;
                }
                let bitstamp_data: BitstampData = bitstamp_parse_result.unwrap();
                let normalized: NormalizedExchangeData = bitstamp_data.into();
                // println!("{:?}", normalized);

                final_orderbook.push(normalized);
            }

            final_orderbook.prepare();
            // println!("{:?}", final_orderbook);
            persistence::store::update_merged_orderbook(final_orderbook);
        }
    });

    // Create and run the gRPC server
    dbg!("Running the gRPC server.");
    let grpc_endpoint: SocketAddr = "0.0.0.0:50005".parse()?;
    let grpc: OrderbookServer = OrderbookServer::new();
    let service = OrderbookAggregatorServer::new(grpc);
    Server::builder()
        .add_service(service)
        .serve(grpc_endpoint)
        .await?;

    Ok(())
}

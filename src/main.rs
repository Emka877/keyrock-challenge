#[cfg(test)]
mod tests;

mod exceptions;
mod configuration;
/// This module contains everything related with streaming the final orderbook to the world
// mod grpc_server;
/// Contains everything related to fetching the data from the remote exchanges
mod exchanges;
/// This module contains the methods used to merge the final orderbook
// mod orderbook;

use exchanges::{open_stream_to_exchange, ExchangeEndpoint};
use tungstenite::Message;
use futures::{StreamExt, SinkExt};

use crate::exchanges::Bitstamp;

#[tokio::main]
async fn main() {
    // TODO: Move all this code as an actual test
    let mut bitstamp = open_stream_to_exchange(ExchangeEndpoint::Bitstamp).await.expect("Error opening stream to Bitstamp.");
    // let mut binance = open_stream_to_exchange(ExchangeEndpoint::Binance).await.expect("Cannot open stream to Binance.");
    
    let bitstamp_subscribe_message = bitstamp.send(Message::Text(r#"{
        "event": "bts:subscribe",
        "data": {
            "channel": "order_book_ethbtc"
        }
    }"#.to_owned())).await;
    
    if bitstamp_subscribe_message.is_err() {
        eprintln!("Could not subscribe to ethbtc.");
    } else {
        let _ = bitstamp.next().await;
    }

    loop {
        tokio::select! {
            bitstamp_msg = bitstamp.next() => {
                match bitstamp_msg {
                    Some(msg) => match msg {
                        Ok(msg) => match msg {
                            Message::Binary(x) => println!("binary {:?}", x),
                            Message::Text(data) => {
                                let incoming: Bitstamp = serde_json::from_str(&data).unwrap();
                                println!("{:?}", incoming);
                            },
                            Message::Ping(x) => {},
                            Message::Pong(x) => {},
                            Message::Close(reason) => { println!("Connection closed: {:?}", reason); },
                        },
                        Err(_) => {println!("server went away"); break;}
                        },
                    None => {println!("no message"); break;},
                }
            }
        };
    }

    let _ = bitstamp.send(Message::Close(None)).await;
}

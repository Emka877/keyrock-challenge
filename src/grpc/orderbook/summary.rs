use std::fmt::Formatter;
use crate::grpc::orderbook::{Level, NormalizedExchangeData};

/// Represents the merged orderbook that's going to be streamed over the gRPC server.
#[derive(Debug, Clone)]
pub struct Summary {
    currency_pair: String,
    asks: Vec<Level>,
    bids: Vec<Level>,
}

impl Summary {
    pub fn new(currency_pair: &str) -> Self {
        Self {
            currency_pair: currency_pair.to_owned(),
            asks: vec!(),
            bids: vec!(),
        }
    }

    /// Adds a new order level for the currency pair.
    pub fn push(&mut self, data: NormalizedExchangeData) {
        let mut new_bids: Vec<Level> = data.bids.into_iter().map(|bid| {
            Level::new(&data.exchange, bid.price, bid.amount)
        }).collect();
        self.bids.append(&mut new_bids);

        let mut new_asks: Vec<Level> = data.asks.into_iter().map(|ask| {
            Level::new(&data.exchange, ask.price, ask.amount)
        }).collect();
        self.asks.append(&mut new_asks);
    }

    pub fn merge(&mut self) {
        self.sort();
        self.trim();
    }

    /// Sorts both asks and bids by price then by amount traded
    fn sort(&mut self) {
        self.bids.sort();
        self.asks.sort();
        let reverse_asks: Vec<Level> = self.asks.clone().into_iter().rev().collect();
        self.asks = reverse_asks;
    }

    /// Trims the amount of asks and bids to 10 each
    fn trim(&mut self) {
        self.asks.truncate(10);
        self.bids.truncate(10);
    }

    /// Calculates the spread between top ask and top bid
    pub fn spread(&self) -> f32 {
        if self.asks.is_empty() || self.bids.is_empty() {
            return 0.0;
        }
        let top_ask: &Level = self.asks.get(0).unwrap();
        let top_bid: &Level = self.bids.get(0).unwrap();
        top_ask.price - top_bid.price
    }
}

// impl std::fmt::Display for Summary {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let mut message: String = format!("Currency pair: {}", self.currency_pair);
//
//         // asks
//         let mut asks: String = "".to_owned();
//         for ask in self.asks.iter() {
//             asks.push_str(ask.to_string().as_str());
//             asks.push('\n');
//         }
//
//         // bids
//         let mut bids: String = "".to_owned();
//         for bid in self.bids.iter() {
//             bids.push_str(bid.to_string().as_str());
//             bids.push('\n');
//         }
//
//         write!(f, "{}\n{}\n----- END ASKS-----\n{}\n----- END BIDS -----\n", message, asks, bids)
//     }
// }
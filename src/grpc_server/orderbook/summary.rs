use crate::grpc_server::orderbook::{Level, NormalizedExchangeData};

/// Represents the merged orderbook that's going to be streamed over the gRPC server.
#[derive(Debug)]
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
    pub fn push(&mut self, mut data: NormalizedExchangeData) {
        self.bids = data.bids.into_iter().map(|bid| {
            Level {
                exchange: data.exchange.clone(),
                price: bid.price,
                amount: bid.amount,
            }
        }).collect();

        self.asks = data.asks.into_iter().map(|ask| {
            Level {
                exchange: data.exchange.clone(),
                price: ask.price,
                amount: ask.amount,
            }
        }).collect();

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
    pub fn get_spread(&self) -> f32 {
        if self.asks.is_empty() || self.bids.is_empty() {
            return 0.0;
        }
        let top_ask: &Level = self.asks.get(0).unwrap();
        let top_bid: &Level = self.bids.get(0).unwrap();
        top_ask.price - top_bid.price
    }
}

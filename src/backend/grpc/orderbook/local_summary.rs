use crate::grpc::orderbook::{LocalLevel, NormalizedExchangeData};

/// Represents the merged orderbook that's going to be streamed over the gRPC server.
#[derive(Debug, Clone)]
pub struct LocalSummary {
    pub currency_pair: String,
    pub asks: Vec<LocalLevel>,
    pub bids: Vec<LocalLevel>,
}

impl LocalSummary {
    pub fn new(currency_pair: &str) -> Self {
        Self {
            currency_pair: currency_pair.to_owned(),
            asks: vec!(),
            bids: vec!(),
        }
    }

    /// Adds a new order level for the currency pair.
    pub fn push(&mut self, data: NormalizedExchangeData) {
        let mut new_bids: Vec<LocalLevel> = data.bids.into_iter().map(|bid| {
            LocalLevel::new(&data.exchange, bid.price, bid.amount)
        }).collect();
        self.bids.append(&mut new_bids);

        let mut new_asks: Vec<LocalLevel> = data.asks.into_iter().map(|ask| {
            LocalLevel::new(&data.exchange, ask.price, ask.amount)
        }).collect();
        self.asks.append(&mut new_asks);
    }

    pub fn prepare(&mut self) {
        self.sort();
        self.trim();
    }

    /// Sorts both asks and bids by price then by amount traded
    // Note: In Rust, Vecs are sorted from smallest to largest
    pub fn sort(&mut self) {
        self.bids.sort();
        // Reverse because we want the highest bids first
        self.bids.reverse();
        self.asks.sort();
    }

    /// Trims the amount of asks and bids to 10 each
    fn trim(&mut self) {
        self.asks.truncate(10);
        self.bids.truncate(10);
    }

    /// Calculates the spread between top ask and top bid
    pub fn spread(&self) -> f64 {
        if self.asks.is_empty() || self.bids.is_empty() {
            return 0.0;
        }
        let top_ask: f64 = self.asks.get(0).unwrap().price;
        let top_bid: f64 = self.bids.get(0).unwrap().price;
        top_ask - top_bid
    }

    pub fn get_asks(&self) -> Vec<LocalLevel> {
        self.asks.clone()
    }

    pub fn get_bids(&self) -> Vec<LocalLevel> {
        self.bids.clone()
    }
}

use crate::{BinanceData, BitstampData};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct NormalizedExchangeData {
    pub exchange: String,
    pub asks: Vec<ExchangePriceAmountPair>,
    pub bids: Vec<ExchangePriceAmountPair>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ExchangePriceAmountPair {
    pub price: f32,
    pub amount: f32,
}

impl From<&Vec<String>> for ExchangePriceAmountPair {
    fn from(source: &Vec<String>) -> Self {
        // TODO: Handle the eventual errors
        let price = fast_float::parse::<f32, _>(source.get(0).as_ref().unwrap()).unwrap();
        let amount = fast_float::parse::<f32, _>(source.get(1).as_ref().unwrap()).unwrap();
        Self {
            price,
            amount,
        }
    }
}

impl From<BinanceData> for NormalizedExchangeData {
    fn from(source: BinanceData) -> Self {
        let exchange: &'static str = "binance";
        let asks: Vec<ExchangePriceAmountPair> = source.asks.iter().map(|ask| {
            ask.into()
        }).collect();
        let bids: Vec<ExchangePriceAmountPair> = source.bids.iter().map(|bid| {
            bid.into()
        }).collect();

        Self {
            exchange: exchange.to_owned(),
            asks,
            bids,
        }
    }
}

impl From<BitstampData> for NormalizedExchangeData {
    fn from(source: BitstampData) -> Self {
        let exchange: &'static str = "bitstamp";
        let asks: Vec<ExchangePriceAmountPair> = source.data.asks.iter().map(|ask| {
            ask.into()
        }).collect();
        let bids: Vec<ExchangePriceAmountPair> = source.data.bids.iter().map(|bid| {
            bid.into()
        }).collect();

        Self {
            exchange: exchange.to_owned(),
            asks,
            bids,
        }
    }
}

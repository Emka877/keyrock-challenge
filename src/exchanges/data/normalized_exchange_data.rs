use crate::{BinanceData, BitstampData};

pub struct NormalizedExchangeData {
    exchange: String,
    asks: Vec<ExchangePriceAmountPair>,
    bids: Vec<ExchangePriceAmountPair>,
}

pub struct ExchangePriceAmountPair {
    price: f32,
    amount: f32,
}

impl From<BinanceData> for NormalizedExchangeData {
    fn from(source: BinanceData) -> Self {
        todo!()
    }
}

impl From<BitstampData> for NormalizedExchangeData {
    fn from(source: BitstampData) -> Self {
        todo!()
    }
}

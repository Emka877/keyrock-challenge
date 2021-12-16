mod binance_data;
mod bitstamp_data;
mod normalized_exchange_data;

pub use binance_data::*;
pub use bitstamp_data::*;
pub use crate::exchanges::exchange::trait_exchange::*;
pub use normalized_exchange_data::*;
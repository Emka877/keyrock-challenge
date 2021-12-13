use serde::Deserialize;

/// Represents data coming from the Bitstamp Exchange
#[derive(Debug, Clone, Deserialize)]
pub struct Bitstamp {
    data: BitstampData,
    channel: String,
    event: String,
}

impl Bitstamp {
    pub fn take_top_ten(self) -> Self {
        let mut result = self.clone();
        todo!()
    }
}

/*** Bitstamp data composition members are below ***/

#[derive(Debug, Clone, Deserialize)]
pub struct BitstampData {
    timestamp: String,
    microtimestamp: String,
    bids: Vec<Vec<String>>,
    asks: Vec<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BitstampSubscription {
    event: String,
    channel: String,
    #[serde(skip_deserializing)]
    data: (),
}
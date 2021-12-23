// This module does not include tests, but tools that are common to many tests.

use std::fs::File;
use std::path::PathBuf;
use crate::{BinanceData, BitstampData, NormalizedExchangeData, LocalSummary};

/// Reads the mock data for both exchanges (see mock files in the "mocks" folder)
pub fn read_all_mock_data() -> LocalSummary {
    let mut summary: LocalSummary = LocalSummary::new("ethbtc");

    // TODO: DRY

    // Read from binance data
    let path: &'static str = "mocks/binance_mock.json";
    let pathbuf: PathBuf = PathBuf::from(path);
    if !pathbuf.exists() || pathbuf.is_dir() {
        panic!("Mock data for binance cannot be found at path: {}", path);
    }
    let file_handle: File = File::open(pathbuf).unwrap();
    let binance_parse: BinanceData = match serde_json::from_reader(file_handle) {
        Ok(x) => x,
        Err(error) => {
            panic!("Binance mock data could not be parsed: {}", error);
        }
    };
    let normalized: NormalizedExchangeData = NormalizedExchangeData::from(binance_parse);
    summary.push(normalized);


    // Read from bitstamp data
    let path: &'static str = "mocks/bitstamp_mock.json";
    let pathbuf: PathBuf = PathBuf::from(path);
    if !pathbuf.exists() || pathbuf.is_dir() {
        panic!("Mock data for bitstamp cannot be found at path: {}", path);
    }
    let file_handle: File = File::open(pathbuf).unwrap();
    let bitstsamp_parse: BitstampData = match serde_json::from_reader(file_handle) {
        Ok(x) => x,
        Err(error) => {
            panic!("Bitstamp mock data could not be parsed: {}", error);
        }
    };
    let normalized: NormalizedExchangeData = NormalizedExchangeData::from(bitstsamp_parse);
    summary.push(normalized);

    summary
}

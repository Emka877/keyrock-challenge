use lazy_static::lazy_static;
use crate::{APP_CONFIG, LocalSummary};
use std::sync::{Mutex, RwLock};

lazy_static!(
    static ref LATEST_ORDERBOOK: RwLock<LocalSummary> = initialize_store();
);

fn initialize_store() -> RwLock<LocalSummary> {
    // dbg!("The orderbook is being initialized.");
    let initial_orderbook: LocalSummary = LocalSummary::new(&APP_CONFIG.currency_pair.clone());
    RwLock::new(initial_orderbook)
}

pub fn update_merged_orderbook(new_orderbook: LocalSummary) {
    *LATEST_ORDERBOOK.write().unwrap() = new_orderbook;
    // dbg!("The orderbook was updated!");
}

pub fn get_merged_orderbook() -> LocalSummary {
    // dbg!("The orderbook is being accessed.");
    let original = LATEST_ORDERBOOK.read().unwrap();
    original.clone()
}
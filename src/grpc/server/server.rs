use std::pin::Pin;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tonic::{Request, Response, Status};
use crate::grpc::server::orderbook::{Empty, Summary};
use crate::grpc::server::orderbook::orderbook_aggregator_server::OrderbookAggregator;
use tokio_stream::Stream;
use crate::LocalSummary;

pub struct OrderbookServer {}

impl OrderbookServer {
    pub fn new() -> Self {
        Self {}
    }
}

#[tonic::async_trait]
impl OrderbookAggregator for OrderbookServer {
    type BookSummaryStream =
        Pin<Box<dyn Stream<Item = Result<Summary, Status>> + Send + Sync + 'static>>;

    async fn book_summary(&self, request: Request<Empty>) -> Result<Response<Self::BookSummaryStream>, Status> {
        // TODO: comment
        dbg!("Incoming request to get the merged orderbook");

        let (tx, rx): (Sender<Summary>, Receiver<Summary>) = mpsc::channel(4);
        // Ready to send out
        tokio::spawn(async move {
            loop {
                let mut orderbook: Summary = crate::persistence::store::get_merged_orderbook().into();
                tx.send(orderbook);
            }
        });

        Ok(Response::new(Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx))))
    }
}
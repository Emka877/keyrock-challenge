use std::pin::Pin;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::mpsc::error::SendError;
use tonic::{Request, Response, Status};
use crate::grpc::server::orderbook::{Empty, Summary};
use crate::grpc::server::orderbook::orderbook_aggregator_server::OrderbookAggregator;
use tokio_stream::Stream;

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
        dbg!("Incoming request to get the merged orderbook");

        let (tx, rx): (Sender<Result<Summary, Status>>, Receiver<Result<Summary, Status>>) = mpsc::channel(4);

        // Ready to send out
        let sleep: tokio::time::Duration = tokio::time::Duration::from_millis(100);
        tokio::spawn(async move {
            loop {
                let orderbook: Summary = crate::persistence::store::get_merged_orderbook().into();
                let res: Result<(), SendError<Result<Summary, Status>>> = tx.send(Ok(orderbook)).await;

                if res.is_err() {
                    break;
                }

                tokio::time::sleep(sleep).await;
            }
        });

        Ok(
            Response::new(
                Box::pin(
                    tokio_stream::wrappers::ReceiverStream::new(rx),
                )
            )
        )
    }
}
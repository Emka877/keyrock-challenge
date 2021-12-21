use tonic::{Request, Response, Status};
use crate::grpc::server::orderbook::Empty;
use crate::grpc::server::orderbook::orderbook_aggregator_server::OrderbookAggregator;
use tokio_stream::{Stream, StreamExt};

pub struct OrderbookServer {

}

#[tonic::async_trait]
impl OrderbookAggregator for OrderbookServer {
    type BookSummaryStream = ();

    async fn book_summary(&self, request: Request<Empty>) -> Result<Response<Self::BookSummaryStream>, Status> {
        todo!()
    }
}
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::handshake::client::Response;
use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;
use tokio_tungstenite::{connect_async_tls_with_config, MaybeTlsStream, WebSocketStream};
use crate::exceptions::OpenStreamError;

use super::endpoints::ExchangeEndpoint;

pub type ExchangeWsTcpStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Opens a new stream to the selected exchange endpoint.
pub async fn open_stream_to_exchange(exchange: ExchangeEndpoint) -> Result<ExchangeWsTcpStream, OpenStreamError> {
    // Setup the stream connection options
    let connect_endpoint: String = exchange.to_string();
    let url = url::Url::parse(&connect_endpoint).unwrap();
    let ws_config: WebSocketConfig = WebSocketConfig::default();

    // Open the actual stream
    let conn: (ExchangeWsTcpStream, Response) = connect_async_tls_with_config(url, Some(ws_config), None)
        .await
        .expect(format!("Could not connect to exchange {}", connect_endpoint).as_str());

    // Handle response
    let response_status: u16 = conn.1.status().as_u16();

    if response_status >= 400 {
        return Err(conn.1.into());
    }
    
    Ok(conn.0)
}

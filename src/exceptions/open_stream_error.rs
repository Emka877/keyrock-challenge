/// Exception when a Websocket stream to an exchange cannot be opened.
#[derive(Debug, Clone)]
pub struct OpenStreamError {
    cause: String,
    status: Option<u16>,
}

impl From<tokio_tungstenite::tungstenite::handshake::client::Response> for OpenStreamError {
    fn from(root: tokio_tungstenite::tungstenite::handshake::client::Response) -> Self {
        Self {
            cause: "Handshake error".to_owned(),
            status: Some(root.status().as_u16()),
        }
    }
}

impl std::fmt::Display for OpenStreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot open stream to exchange: (Http code {}) {}", self.status.unwrap_or(0), self.cause)
    }
}

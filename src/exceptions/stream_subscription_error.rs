use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub struct StreamSubscriptionError {
    cause: String,
}

impl std::fmt::Display for StreamSubscriptionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<&str> for StreamSubscriptionError {
    fn from(cause: &str) -> Self {
        Self {
            cause: cause.to_owned(),
        }
    }
}

impl From<String> for StreamSubscriptionError {
    fn from(cause: String) -> Self {
        Self {
            cause: cause.clone(),
        }
    }
}

impl std::error::Error for StreamSubscriptionError {}

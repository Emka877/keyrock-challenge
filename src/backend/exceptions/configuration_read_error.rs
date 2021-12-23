use std::fmt::Formatter;

/// Error while reading the app configuration file
#[derive(Debug)]
pub struct ConfigurationReadError {
    cause: String,
}

impl From<&str> for ConfigurationReadError {
    fn from(cause: &str) -> Self {
        Self {
            cause: cause.to_owned(),
        }
    }
}

impl From<String> for ConfigurationReadError {
    fn from(cause: String) -> Self {
        Self {
            cause: cause.clone()
        }
    }
}

impl ConfigurationReadError {
    pub fn new(cause: &str) -> Self {
        Self {
            cause: cause.to_owned(),
        }
    }
}

impl std::fmt::Display for ConfigurationReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable to load configuration: {}", self.cause)
    }
}

impl std::error::Error for ConfigurationReadError {

}
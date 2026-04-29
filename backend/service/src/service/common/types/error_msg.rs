//! Generic Error Message API type

use poem_openapi::{NewType, types::Example};

/// Error Message
#[derive(NewType)]
#[oai(example)]
pub struct ErrorMessage(String);

impl Example for ErrorMessage {
    /// An example of error message.
    fn example() -> Self {
        Self("An error has occurred, the details of the error are ...".to_owned())
    }
}

impl From<anyhow::Error> for ErrorMessage {
    fn from(value: anyhow::Error) -> Self {
        Self(value.to_string())
    }
}

impl From<poem::Error> for ErrorMessage {
    fn from(value: poem::Error) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for ErrorMessage {
    fn from(val: String) -> Self {
        Self(val)
    }
}

impl From<&str> for ErrorMessage {
    fn from(val: &str) -> Self {
        Self(val.to_owned())
    }
}

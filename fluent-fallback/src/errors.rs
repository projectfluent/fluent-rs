use fluent_bundle::FluentError;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum LocalizationError {
    Bundle {
        id: Option<String>,
        error: FluentError,
    },
    MissingMessage {
        id: String,
    },
    MissingValue {
        id: String,
    },
    SyncRequestInAsyncMode,
}

impl<I: ToString> From<(I, FluentError)> for LocalizationError {
    fn from(pieces: (I, FluentError)) -> Self {
        Self::Bundle {
            id: Some(pieces.0.to_string()),
            error: pieces.1,
        }
    }
}

impl From<FluentError> for LocalizationError {
    fn from(error: FluentError) -> Self {
        Self::Bundle { id: None, error }
    }
}

impl std::fmt::Display for LocalizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bundle {
                id: Some(id),
                error,
            } => write!(f, "Bundle {} error: {}", id, error),
            Self::Bundle { id: None, error } => write!(f, "Bundle error: {}", error),
            Self::MissingMessage { id } => write!(f, "Missing message: {}", id),
            Self::MissingValue { id } => write!(f, "Missing value in message: {}", id),
            Self::SyncRequestInAsyncMode => {
                write!(f, "Triggered synchronous format while in async mode")
            }
        }
    }
}

impl Error for LocalizationError {}

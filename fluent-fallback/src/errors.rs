use fluent_bundle::FluentError;
use std::error::Error;
use unic_langid::LanguageIdentifier;

#[derive(Debug, PartialEq)]
pub enum LocalizationError {
    Bundle {
        error: FluentError,
    },
    Resolver {
        id: String,
        locale: LanguageIdentifier,
        errors: Vec<FluentError>,
    },
    MissingMessage {
        id: String,
    },
    MissingValue {
        id: String,
    },
    SyncRequestInAsyncMode,
}

impl From<FluentError> for LocalizationError {
    fn from(error: FluentError) -> Self {
        Self::Bundle { error }
    }
}

impl std::fmt::Display for LocalizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bundle { error } => write!(f, "[fluent][bundle] error: {}", error),
            Self::Resolver { id, locale, errors } => {
                let errors: Vec<String> = errors.iter().map(|err| err.to_string()).collect();
                write!(
                    f,
                    "[fluent][resolver] errors in {}/{}: {}",
                    locale.to_string(),
                    id,
                    errors.join(", ")
                )
            }
            Self::MissingMessage { id } => write!(f, "[fluent] Missing message: {}", id),
            Self::MissingValue { id } => write!(f, "[fluent] Missing value in message: {}", id),
            Self::SyncRequestInAsyncMode => {
                write!(f, "Triggered synchronous format while in async mode")
            }
        }
    }
}

impl Error for LocalizationError {}

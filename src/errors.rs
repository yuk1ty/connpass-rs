//! Provides error types using in this crate.

use thiserror::Error;

/// General errors for this crate.
#[derive(Debug, Error)]
#[error(transparent)]
pub enum ConnpassCliError {
    /// Errors around validation.
    Validation(ValidationError),
    /// Errors around HTTP connection including JSON decoding, status code, etc.
    HttpResponse(HttpResponseError),
}

/// Represents errors around validation.
#[derive(Debug, Error)]
pub enum ValidationError {
    /// Uses when a value is out of the specific range.
    #[error("{msg}")]
    OutOfRange { msg: String },
    /// Uses when unexpected token is passed to a value.
    #[error("{msg}")]
    InvalidToken { msg: String },
}

/// Represents errors around HTTP connection.
#[derive(Debug, Error)]
pub enum HttpResponseError {
    /// Uses when an error cannot be categorised any more.
    #[error("{0}")]
    Various(String),
    /// Uses when decoding JSON failed.
    #[error("{0}")]
    JsonDecode(String),
    /// For representing HTTP status code 403.
    #[error("Forbidden")]
    Forbidden,
    /// For representing HTTP status code 500.
    #[error("Internal Server Error")]
    InternalServerError,
    /// For representing HTTP status code 503.
    #[error("Service Unavailable")]
    ServiceUnavailable,
    /// Convert from the errors from reqwest crate to the domain specific error type.
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

pub type ConnpassResult<T> = core::result::Result<T, ConnpassCliError>;

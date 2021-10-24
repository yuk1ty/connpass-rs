use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum ConnpassCliError {
    Validation(ValidationError),
    HttpResponse(HttpResponseError),
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("{msg}")]
    OutOfRange { msg: String },
    #[error("{msg}")]
    InvalidToken { msg: String },
}

#[derive(Debug, Error)]
pub enum HttpResponseError {
    #[error("{0}")]
    Various(String),
    #[error("{0}")]
    JsonDecode(String),
    #[error("Forbidden")]
    Forbidden,
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Service Unavailable")]
    ServiceUnavailable,
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

pub type ConnpassResult<T> = core::result::Result<T, ConnpassCliError>;

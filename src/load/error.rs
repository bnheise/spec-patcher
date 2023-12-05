use thiserror::Error;

use crate::error::Reporter;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Http request failed")]
    Send(#[from] reqwest::Error),
    #[error("Failed to deserialize response: {1}")]
    Deserialize(#[source] reqwest::Error, &'static str),
    #[error("Failed to format request url")]
    UrlFormat(#[from] url::ParseError),
    #[error("Missing required field from API response: {0}")]
    MissingField(&'static str),
}

impl Reporter for Error {}

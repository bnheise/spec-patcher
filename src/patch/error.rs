use thiserror::Error;

use crate::error::Reporter;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Required field missing: {0}")]
    MissingField(&'static str),
    #[error("Failed to serialize value: {0}")]
    Serialize(#[from] serde_json::Error),
}

impl Reporter for Error {}

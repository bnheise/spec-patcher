use thiserror::Error;

use crate::error::Reporter;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Required field missing: {0}")]
    MissingField(&'static str),
    #[error("Failed to serialize value: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("Missing open api schema for: {0}")]
    MissingSchema(String),
    #[error("Provided schema is invalid: {0}")]
    InvalidSchema(&'static str),
    #[error("Provided object definition is invalid: {0}")]
    InvalidObjectDef(&'static str),
}

impl Reporter for Error {}

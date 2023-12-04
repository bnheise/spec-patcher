use thiserror::Error;

#[derive(Debug, Error)]
pub enum PatchError {
    #[error("Required field missing: {0}")]
    MissingField(&'static str),
    #[error("Failed to serialize value: {0}")]
    Serialize(#[from] serde_json::Error),
}

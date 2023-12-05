use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to serialize or deserialize spec")]
    Serialize(#[from] serde_json::Error),
    #[error("Failed to write open-api spec to file")]
    Write(#[from] std::io::Error),
}

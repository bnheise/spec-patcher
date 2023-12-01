use thiserror::Error;

use crate::{config::ConfigError, load::client::ClientError};

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Failed to initialize: {0}")]
    Initialize(#[from] ConfigError),
    #[error("Failed to load spec: {0}")]
    Load(#[from] ClientError),
}

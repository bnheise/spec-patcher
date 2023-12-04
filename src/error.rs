use thiserror::Error;

use crate::{config::ConfigError, load::LoadError, output::OutputError};

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Failed to initialize: {0}")]
    Initialize(#[from] ConfigError),
    #[error("Failed to load spec: {0}")]
    Load(#[from] LoadError),
    #[error("Failed to output open api spec")]
    Output(#[from] OutputError),
}

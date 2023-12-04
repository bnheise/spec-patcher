use config::ConfigError;
use serde::Deserialize;
use thiserror::Error;

use super::sub::{connection::ConnectionOpt, output::Output, source::SourceOpt};

#[derive(Debug, Clone, Deserialize)]
pub struct FileEnvConfig(InnerConfigOpt);

#[derive(Debug, Deserialize, Clone)]
pub struct InnerConfigOpt {
    pub connection: Option<ConnectionOpt>,
    pub source: Option<SourceOpt>,
    pub output: Option<Output>,
}

impl FileEnvConfig {
    const FILE_NAME: &'static str = "spec-patcher.yaml";
    const ENV_PREFIX: &'static str = "SPECPATCH";

    pub fn init() -> Result<InnerConfigOpt, FileEnvConfigError> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name(Self::FILE_NAME).required(false))
            .add_source(config::Environment::with_prefix(Self::ENV_PREFIX).ignore_empty(true))
            .build()?;

        Ok(settings.try_deserialize()?)
    }
}

#[derive(Debug, Error)]
pub enum FileEnvConfigError {
    #[error("Error initializing file config:\n\t{0}")]
    ConfigError(#[from] ConfigError),
}

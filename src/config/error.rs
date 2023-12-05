use thiserror::Error;

use super::file::FileEnvConfigError;

/// Possible errors that may arise when loading a configuration.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to initialize settings from config file")]
    FileConfig(#[from] FileEnvConfigError),
    #[error("Missing required argument: {0}")]
    MissingArg(&'static str),
    #[error("Please provide authentication by either providing an oauth secret or a username and password")]
    MissingAuth,
    #[error("You have provided both an oauth secret and basic auth credentials. Please chooseo only one")]
    AuthConflict,
}

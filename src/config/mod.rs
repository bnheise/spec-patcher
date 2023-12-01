use clap::Parser;
use serde::Deserialize;
use thiserror::Error;

use crate::config::file::FileEnvConfig;

use self::{
    file::{FileEnvConfigError, InnerConfigOpt},
    sub::{
        connection::{Auth, Connection, ConnectionOpt},
        source::{Source, SourceOpt},
    },
};

mod file;
pub mod sub;

#[derive(Debug, Parser, Deserialize, Clone)]
#[command(author, version, about, long_about = None)]
struct InnerConfig {
    #[command(flatten)]
    pub connection: ConnectionOpt,
    #[command(flatten)]
    pub source: SourceOpt,
}

impl InnerConfig {
    pub fn init() -> Result<Config, ConfigError> {
        let file_args = FileEnvConfig::init()?;
        let cli_args = Self::parse();

        Config::try_merge(cli_args, file_args)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub connection: Connection,
    pub source: Source,
}

impl Config {
    fn try_merge(cli: InnerConfig, file: InnerConfigOpt) -> Result<Self, ConfigError> {
        let base_url = match &cli.connection.base_url {
            Some(url) => url,
            None => file
                .connection
                .as_ref()
                .ok_or(ConfigError::MissingArg("url"))?
                .base_url
                .as_ref()
                .ok_or(ConfigError::MissingArg("url"))?,
        }
        .to_owned();

        let auth = match (cli.connection.secret, cli.connection.basic_auth) {
            (None, Some(basic)) => Auth::Basic(basic),
            (Some(secret), None) => Auth::OAuth(secret),
            (Some(_), Some(_)) => Err(ConfigError::AuthConflict)?,
            (None, None) => file
                .connection
                .ok_or(ConfigError::MissingAuth)?
                .secret
                .map(Auth::OAuth) // TODO: add basic auth from file
                .ok_or(ConfigError::MissingArg("auth"))?,
        };

        let source_type = match &cli.source.source_type {
            Some(source_type) => source_type,
            None => file
                .source
                .as_ref()
                .ok_or(ConfigError::MissingArg("source"))?
                .source_type
                .as_ref()
                .ok_or(ConfigError::MissingArg("source"))?,
        }
        .to_owned();

        let erc = match &cli.source.erc {
            Some(source_type) => source_type,
            None => file
                .source
                .as_ref()
                .ok_or(ConfigError::MissingArg("erc"))?
                .erc
                .as_ref()
                .ok_or(ConfigError::MissingArg("erc"))?,
        }
        .to_owned();

        Ok(Config {
            connection: Connection { base_url, auth },
            source: Source { source_type, erc },
        })
    }

    pub fn init() -> Result<Self, ConfigError> {
        InnerConfig::init()
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to initialize settings from config file:\n\t{0}")]
    FileConfig(#[from] FileEnvConfigError),
    #[error("Missing required argument: {0}")]
    MissingArg(&'static str),
    #[error("Please provide authentication by either providing an oauth secret or a username and password")]
    MissingAuth,
    #[error("You have provided both an oauth secret and basic auth credentials. Please chooseo only one")]
    AuthConflict,
}

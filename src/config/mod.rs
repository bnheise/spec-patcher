use clap::Parser;
use serde::Deserialize;
use url::Url;

use crate::config::file::FileEnvConfig;

use self::{
    file::InnerConfigOpt,
    sub::{
        connection::{Auth, Connection, ConnectionOpt, TokenAuth},
        output::Output,
        patch::{InnerCase, Patch, PatchOpt},
        source::{Source, SourceOpt},
    },
};

mod error;
mod file;
pub mod sub;
pub use error::Error;

/// Private struct use for initializing parameters entered from the command
/// line. These results will be merged with parameters derived from files
/// or environment variables into the final [Config].
#[derive(Debug, Parser, Deserialize, Clone)]
#[command(author, version, about, long_about = None)]
struct InnerConfig {
    #[command(flatten)]
    pub connection: ConnectionOpt,
    #[command(flatten)]
    pub source: SourceOpt,
    #[command(flatten)]
    pub output: Output,
    #[command(flatten)]
    pub patch: PatchOpt,
}

impl InnerConfig {
    /// Initialize the command line config and the [FileEnvConfig], then merge
    /// them together to create the final [Config].
    pub fn init() -> Result<Config, Error> {
        dotenv::dotenv().expect("Dotenv failed");
        let file_args = FileEnvConfig::init()?;
        let cli_args = Self::parse();

        Config::try_merge(cli_args, file_args)
    }
}

/// Contains the configuration settings for loading, patching, and outputting the
/// open api specification.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Config {
    pub connection: Connection,
    pub source: Source,
    pub output: Output,
    pub patch: Patch,
}

impl Config {
    /// Attempts to merge the settings taken from the command line, a config file, and
    /// environment variables. The precedence goes in the following order, where lower
    /// numbers overwrite larger numbers:
    ///
    /// 1. command line
    /// 2. config file
    /// 3. environment variable
    ///
    /// The purpose of this ordering is that we want the most explicitly set parameters
    /// to take precedence. Therefore, if a setting was configured as an environment
    /// variable, but a config file exists that also provides the parameter, then the config
    /// file takes precedence because a config file is more explicit than an environment
    /// variable. By the same token, if the user provided an input via a command line
    /// argument, this is even more explicit than a config file and so overwrites the
    /// same setting even if it was already defined there.
    fn try_merge(cli: InnerConfig, file: InnerConfigOpt) -> Result<Self, Error> {
        let base_url = Self::merge_base_url(&cli, &file)?;

        let auth = match (cli.connection.secret, cli.connection.basic_auth) {
            (None, Some(basic)) => Some(Auth::Basic(basic)),
            (Some(secret), None) => Some(Auth::OAuth(secret)),
            (Some(_), Some(_)) => Err(Error::AuthConflict)?,
            (None, None) => file
                .connection
                .as_ref()
                .ok_or(Error::MissingAuth)?
                .secret
                .as_ref()
                .map(TokenAuth::to_owned)
                .map(Auth::OAuth),
        };

        let auth = match auth {
            Some(auth) => auth,
            None => file
                .connection
                .ok_or(Error::MissingAuth)?
                .basic_auth
                .map(Auth::Basic)
                .ok_or(Error::MissingAuth)?,
        };

        let source_type = match &cli.source.source_type {
            Some(source_type) => source_type,
            None => file
                .source
                .as_ref()
                .ok_or(Error::MissingArg("source"))?
                .source_type
                .as_ref()
                .ok_or(Error::MissingArg("source"))?,
        }
        .to_owned();

        let erc = match &cli.source.erc {
            Some(source_type) => source_type,
            None => file
                .source
                .as_ref()
                .ok_or(Error::MissingArg("erc"))?
                .erc
                .as_ref()
                .ok_or(Error::MissingArg("erc"))?,
        }
        .to_owned();

        let output = match &cli.output.output {
            Some(output) => Some(output.to_owned()),
            None => {
                if let Some(output) = &file.output {
                    output.output.as_ref().map(|path| path.to_owned())
                } else {
                    None
                }
            }
        }
        .to_owned();

        let enum_case = match &cli.patch.enum_case {
            Some(case) => *case,
            None => file
                .patch
                .as_ref()
                .and_then(|patch| patch.enum_case.as_ref().map(InnerCase::to_owned))
                .unwrap_or_default(),
        }
        .to_owned();

        Ok(Config {
            connection: Connection { base_url, auth },
            source: Source { source_type, erc },
            output: Output { output },
            patch: Patch { enum_case },
        })
    }

    /// Extract the base_url parameters from the config sources and returns
    /// the one with the highest precedence
    fn merge_base_url(cli: &InnerConfig, file: &InnerConfigOpt) -> Result<Url, Error> {
        let res = match &cli.connection.base_url {
            Some(url) => url,
            None => file
                .connection
                .as_ref()
                .ok_or(Error::MissingArg("url"))?
                .base_url
                .as_ref()
                .ok_or(Error::MissingArg("url"))?,
        }
        .to_owned();

        Ok(res)
    }

    /// Initialize the configuration via a combination of command line arguments,
    /// environment variables, and parameters from a config file
    pub fn init() -> Result<Self, Error> {
        InnerConfig::init()
    }
}

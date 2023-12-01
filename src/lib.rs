use error::CliError;

use crate::config::Config;

mod config;
mod error;
mod load;
mod patch;

pub fn run() -> Result<(), CliError> {
    let config = Config::init()?;

    let spec = load::load(&config)?;

    Ok(())
}

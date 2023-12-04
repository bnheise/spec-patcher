use error::CliError;
use output::output;

use crate::config::Config;

mod config;
mod error;
mod load;
mod output;
mod patch;

pub fn run() -> Result<(), CliError> {
    let config = Config::init()?;

    let metadata = load::load(&config)?;

    output(&config.output, &metadata.spec)?;

    Ok(())
}

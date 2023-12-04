use crate::config::Config;
use error::CliError;
use patch::patch;

mod config;
mod error;
mod load;
mod output;
mod patch;

pub fn run() -> Result<(), CliError> {
    let config = Config::init()?;

    let metadata = load::load(&config)?;

    let spec = patch(&config, metadata);

    output::output(&config.output, &spec)?;

    Ok(())
}

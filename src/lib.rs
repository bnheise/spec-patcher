use crate::config::Config;
use error::CliError;
use patch::patch;

mod config;
pub mod error;
mod load;
pub mod object;
mod output;
mod patch;
mod schema;

pub fn run() -> Result<(), CliError> {
    let config = Config::init()?;

    let metadata = load::load(&config)?;

    let spec = patch(&config, metadata)?;

    output::output(&config.output, &spec)?;

    Ok(())
}

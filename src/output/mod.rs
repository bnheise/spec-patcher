use openapi::v3_0::Spec;
use std::{fs, io::Write};
use thiserror::Error;

use crate::config::sub::output::Output;

pub fn output(config: &Output, spec: &Spec) -> Result<(), OutputError> {
    let output = serde_json::to_string_pretty(&spec)?;
    if let Some(path) = &config.output {
        fs::write(path, output)?;
        Ok(())
    } else {
        std::io::stdout().write_all(output.as_bytes())?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum OutputError {
    #[error("Failed to serialize open-api spec: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("Failed to write open-api spec to file: {0}")]
    Write(#[from] std::io::Error),
}

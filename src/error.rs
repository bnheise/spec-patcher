use crate::{config, patch};
use crate::{load, output};
use std::error::Error;
use thiserror::Error;

/// Represents all errors that may arise when running application
#[derive(Debug, Error)]
pub enum CliError {
    #[error("Failed to initialize")]
    Initialize(#[from] config::Error),
    #[error("Failed to load spec")]
    Load(#[from] load::Error),
    #[error("Failed to output open api spec")]
    Output(#[from] output::Error),
    #[error("Failed to patch openapi spec")]
    Patch(#[from] patch::Error),
}

pub trait Reporter: Error {
    fn report(&self, header: &'static str) {
        println!("\n{header}");
        println!("{self}");
        let mut source = self.source();
        while let Some(s) = source {
            println!("\t{s}");
            source = s.source();
        }
    }
}

impl Reporter for CliError {}

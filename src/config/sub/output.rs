use clap::Args;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Args, Deserialize)]
pub struct Output {
    #[arg(short = 'o', long = "out")]
    pub output: Option<PathBuf>,
}

use clap::Args;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Args, Deserialize, Default)]
pub struct Output {
    #[arg(short = 'o', long = "out")]
    pub output: Option<PathBuf>,
}

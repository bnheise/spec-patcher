use clap::{Args, ValueEnum};
use serde::Deserialize;

#[derive(Debug, Clone, Args, Deserialize)]
pub struct SourceOpt {
    #[arg(value_enum, short = 's', long = "source")]
    pub source_type: Option<SourceType>,
    #[arg(short = 'e', long = "erc")]
    pub erc: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Source {
    pub source_type: SourceType,
    pub erc: String,
}

#[derive(Debug, Clone, ValueEnum, Deserialize)]
pub enum SourceType {
    SystemObject,
    CustomObject,
    HeadlessApi,
}

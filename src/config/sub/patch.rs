use clap::{Args, ValueEnum};
use convert_case::Case;
use serde::Deserialize;

#[derive(Debug, Clone, Args, Deserialize)]
pub struct PatchOpt {
    #[arg(value_enum, long = "enum_case")]
    pub enum_case: Option<InnerCase>,
}

#[derive(Debug, Deserialize, Clone, Copy, Default, ValueEnum, PartialEq)]
pub enum InnerCase {
    #[default]
    Camel,
    Pascal,
    ScreamingSnake,
    UpperCamel,
    Snake,
}

impl From<InnerCase> for Case {
    fn from(value: InnerCase) -> Self {
        match value {
            InnerCase::Camel => Self::Camel,
            InnerCase::Pascal => Self::Pascal,
            InnerCase::ScreamingSnake => Self::ScreamingSnake,
            InnerCase::UpperCamel => Self::UpperCamel,
            InnerCase::Snake => Self::Snake,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Copy)]
pub struct Patch {
    pub enum_case: InnerCase,
}

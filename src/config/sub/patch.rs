use clap::{Args, ValueEnum};
use convert_case::Case;
use serde::Deserialize;

#[derive(Debug, Clone, Args, Deserialize)]
pub struct PatchOpt {
    #[arg(value_enum, long = "enum_case")]
    pub enum_case: Option<InnerCase>, // TODO: allow configuration to let users not generate picklist enums
}

#[derive(Debug, Deserialize, Clone, Copy, Default, ValueEnum, PartialEq)]
pub enum InnerCase {
    Upper,
    Lower,
    Title,
    Toggle,
    #[default]
    Camel,
    Pascal,
    UpperCamel,
    Snake,
    UpperSnake,
    ScreamingSnake,
    Kebab,
    Cobol,
    UpperKebab,
    Train,
    Flat,
    UpperFlat,
    Alternating,
}

impl From<InnerCase> for Case {
    fn from(value: InnerCase) -> Self {
        match value {
            InnerCase::Camel => Self::Camel,
            InnerCase::Pascal => Self::Pascal,
            InnerCase::ScreamingSnake => Self::ScreamingSnake,
            InnerCase::UpperCamel => Self::UpperCamel,
            InnerCase::Snake => Self::Snake,
            InnerCase::Upper => Self::Upper,
            InnerCase::Lower => Self::Lower,
            InnerCase::Title => Self::Title,
            InnerCase::Toggle => Self::Toggle,
            InnerCase::UpperSnake => Self::UpperSnake,
            InnerCase::Kebab => Self::Kebab,
            InnerCase::Cobol => Self::Cobol,
            InnerCase::UpperKebab => Self::UpperKebab,
            InnerCase::Train => Self::Train,
            InnerCase::Flat => Self::Flat,
            InnerCase::UpperFlat => Self::UpperFlat,
            InnerCase::Alternating => Self::Alternating,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Copy, Default)]
pub struct Patch {
    pub enum_case: InnerCase,
}

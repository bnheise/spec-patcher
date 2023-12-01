use serde::Deserialize;

use crate::config::{sub::source::SourceType, Config};

use self::client::{Client, ClientError};

pub mod client;

pub fn load(config: &Config) -> Result<MetaData, ClientError> {
    let client = Client::new();
    let object_def = match config.source.source_type {
        SourceType::SystemObject | SourceType::CustomObject => Some(client.get_def(config)?),
        SourceType::HeadlessApi => None,
    };

    dbg!(object_def);
    todo!()
}

#[derive(Debug)]
pub struct MetaData {
    pub spec: openapi::Spec,
    pub object_def: Option<ObjectDef>,
}

#[derive(Debug, Deserialize)]
pub struct ObjectDef;

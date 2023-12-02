use self::client::{Client, ClientError};
use crate::config::{sub::source::SourceType, Config};
use liferay_object::models::ObjectDefinition;

pub mod client;

pub fn load(config: &Config) -> Result<MetaData, ClientError> {
    let mut client = Client::new();
    let object_def = match config.source.source_type {
        SourceType::SystemObject | SourceType::CustomObject => Some(client.get_def(config)?),
        SourceType::HeadlessApi => None,
    };

    let endpoint = format_endpoint(&config.source.source_type, object_def.as_ref())?;

    let spec = client.get_spec(config, endpoint)?;

    dbg!(spec);
    todo!()
}

fn format_endpoint(
    source_type: &SourceType,
    object_def: Option<&ObjectDefinition>,
) -> Result<String, ClientError> {
    let endpoint = if let Some(object_def) = &object_def {
        let path = object_def
            .rest_context_path
            .as_ref()
            .ok_or(ClientError::MissingField("rest_context_path"))?;
        match source_type {
            SourceType::SystemObject => format!("{path}/openapi.json"),
            SourceType::CustomObject => {
                let mut parts = path.split('/').collect::<Vec<&str>>();
                parts.pop();
                format!("{}/openapi.json", parts.join("/"))
            }
            SourceType::HeadlessApi => unreachable!(),
        }
    } else {
        todo!("get path from user input")
        // pattern : /o/{api-name}/{api-version}/openapi.json
    };

    Ok(endpoint)
}

#[derive(Debug)]
pub struct MetaData {
    pub spec: openapi::Spec,
    pub object_def: Option<ObjectDefinition>,
}

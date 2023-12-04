use self::client::Client;
use crate::config::{sub::source::SourceType, Config};
use liferay_object::models::ObjectDefinition;
use list_type::models::ListTypeDefinition;
use thiserror::Error;

pub mod client;

pub fn load(config: &Config) -> Result<MetaData, LoadError> {
    let mut client = Client::new();
    let object_def = match config.source.source_type {
        SourceType::SystemObject | SourceType::CustomObject => Some(client.get_def(config)?),
        SourceType::HeadlessApi => None,
    };

    let picklists = if let Some(object_def) = object_def.as_ref() {
        let picklists = &object_def.object_fields.as_ref().map(|fields| {
            let ercs = fields
                .iter()
                .filter_map(|field| {
                    field
                        .list_type_definition_external_reference_code
                        .to_owned()
                })
                .collect::<Vec<String>>();
            client.get_picklists(config, ercs)
        });
        picklists.to_owned()
    } else {
        None
    };

    let endpoint = format_endpoint(&config.source.source_type, object_def.as_ref())?;
    let spec = client.get_spec(config, endpoint)?;

    Ok(MetaData {
        spec,
        object_def,
        picklists,
    })
}

fn format_endpoint(
    source_type: &SourceType,
    object_def: Option<&ObjectDefinition>,
) -> Result<String, LoadError> {
    let endpoint = if let Some(object_def) = &object_def {
        let path = object_def
            .rest_context_path
            .as_ref()
            .ok_or(LoadError::MissingField("rest_context_path"))?;
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
    pub spec: openapi::v3_0::Spec,
    pub object_def: Option<ObjectDefinition>,
    pub picklists: Option<Vec<ListTypeDefinition>>,
}

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("Http request failed: {0}")]
    Send(#[from] reqwest::Error),
    #[error("Failed to deserialize response: {0}")]
    Deserialize(reqwest::Error, &'static str),
    #[error("Failed to format request url")]
    UrlFormat(#[from] url::ParseError),
    #[error("Missing required field from API response: {0}")]
    MissingField(&'static str),
}

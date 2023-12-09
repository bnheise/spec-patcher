use self::client::Client;
use crate::{
    config::{sub::source::SourceType, Config},
    object,
};
use liferay_object::models::ObjectDefinition;
use list_type::models::ListTypeDefinition;

pub mod client;
mod error;
pub use error::Error;
use oas::OpenAPIV3;
mod extract;
mod format;

pub fn load(config: &Config) -> Result<MetaData, Error> {
    let mut client = Client::new();
    let object_def = match config.source.source_type {
        SourceType::SystemObject | SourceType::CustomObject => Some(client.get_def(config)?),
        SourceType::HeadlessApi => None,
    };

    let picklists = if let Some(object_def) = object_def.as_ref() {
        let ercs = object::extract::picklist_ercs(object_def);
        let picklists = client.get_picklists(config, ercs);
        Some(picklists)
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
) -> Result<String, Error> {
    let endpoint = if let Some(object_def) = &object_def {
        let path = object_def
            .rest_context_path
            .as_ref()
            .ok_or(Error::MissingField("rest_context_path"))?;
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

/// The collection of data extracted from Liferay that is needed to
/// perform patching of the open api spec
#[derive(Debug, Clone)]
pub struct MetaData {
    pub spec: OpenAPIV3,
    pub object_def: Option<ObjectDefinition>,
    pub picklists: Option<Vec<ListTypeDefinition>>,
}

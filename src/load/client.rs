use crate::config::Config;
use thiserror::Error;

use super::ObjectDef;

#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn get_spec(&self, config: &Config) -> Result<openapi::Spec, ClientError> {
        self.client
            .get(config.connection.base_url.as_str())
            .send()
            .map_err(ClientError::Send)?
            .json()
            .map_err(|e| ClientError::Deserialize(e, "openapi spec"))
    }

    // fn format_url(base_url: &Url, source: &Source) -> Url {}

    pub fn get_def(&self, config: &Config) -> Result<ObjectDef, ClientError> {
        let erc = &config.source.erc;
        let url = Self::format_object_def_url(config.connection.base_url.as_str(), erc);
        self.client
            .get(url)
            .send()
            .map_err(ClientError::Send)?
            .json()
            .map_err(|e| ClientError::Deserialize(e, "object definition"))
    }

    fn format_object_def_url(base_url: &str, erc: &str) -> String {
        format!(
            "{base_url}/o/object-admin/v1.0/object-definitions/by-external-reference-code/${erc}"
        )
    }
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Http request failed: {0}")]
    Send(reqwest::Error),
    #[error("Failed to deserialize response: {0}")]
    Deserialize(reqwest::Error, &'static str),
}

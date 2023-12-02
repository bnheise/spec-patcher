use crate::config::Config;
use thiserror::Error;
use url::Url;

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

    pub fn get_def(&self, config: &Config) -> Result<ObjectDef, ClientError> {
        let url = Self::format_object_def_url(&config.connection.base_url, &config.source.erc)?;
        let mut req = self.client.get(url);

        req = match &config.connection.auth {
            crate::config::sub::connection::Auth::Basic(basic) => {
                req.basic_auth(basic.username.to_owned(), Some(basic.password.to_owned()))
            }
            crate::config::sub::connection::Auth::OAuth(_) => todo!(),
        };

        let res = req.send().map_err(ClientError::Send)?;

        res.error_for_status_ref()?;

        res.json()
            .map_err(|e| ClientError::Deserialize(e, "object definition"))
    }

    fn format_object_def_url(base_url: &Url, erc: &str) -> Result<Url, ClientError> {
        let endpoint =
            format!("/o/object-admin/v1.0/object-definitions/by-external-reference-code/{erc}");

        Ok(base_url.join(&endpoint)?)
    }
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Http request failed: {0}")]
    Send(#[from] reqwest::Error),
    #[error("Failed to deserialize response: {0}")]
    Deserialize(reqwest::Error, &'static str),
    #[error("Failed to format request url")]
    UrlFormat(#[from] url::ParseError),
}

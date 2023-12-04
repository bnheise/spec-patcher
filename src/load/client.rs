use crate::config::{sub::connection::TokenAuth, Config};
use liferay_object::models::ObjectDefinition;
use reqwest::{
    blocking::RequestBuilder,
    header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE},
};
use serde::{Deserialize, Serialize};
use url::Url;

use super::LoadError;

#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::blocking::Client,
    token: Option<TokenResponse>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            token: None,
        }
    }

    pub fn get_spec(
        &mut self,
        config: &Config,
        endpoint: String,
    ) -> Result<openapi::v3_0::Spec, LoadError> {
        let mut req = self.client.get(config.connection.base_url.join(&endpoint)?);
        req = self.set_auth(req, config)?;
        req.send()
            .map_err(LoadError::Send)?
            .json()
            .map_err(|e| LoadError::Deserialize(e, "openapi spec"))
    }

    fn set_auth(
        &mut self,
        req: RequestBuilder,
        config: &Config,
    ) -> Result<RequestBuilder, LoadError> {
        let req = match &config.connection.auth {
            crate::config::sub::connection::Auth::Basic(basic) => {
                req.basic_auth(basic.username.to_owned(), Some(basic.password.to_owned()))
            }
            crate::config::sub::connection::Auth::OAuth(bearer) => {
                let token = self.get_token(config, bearer)?;
                req.bearer_auth(token)
            }
        };

        Ok(req)
    }

    pub fn get_def(&mut self, config: &Config) -> Result<ObjectDefinition, LoadError> {
        let url = Self::format_object_def_url(&config.connection.base_url, &config.source.erc)?;
        let mut req = self.client.get(url);

        req = self.set_auth(req, config)?;

        let res = req.send().map_err(LoadError::Send)?;

        res.error_for_status_ref()?;

        res.json()
            .map_err(|e| LoadError::Deserialize(e, "object definition"))
    }

    fn format_object_def_url(base_url: &Url, erc: &str) -> Result<Url, LoadError> {
        let endpoint =
            format!("/o/object-admin/v1.0/object-definitions/by-external-reference-code/{erc}");

        Ok(base_url.join(&endpoint)?)
    }

    fn get_token(&mut self, config: &Config, bearer: &TokenAuth) -> Result<&str, LoadError> {
        if self.token.is_none() {
            self.token = Some(self.fetch_token(config, bearer)?);
        }
        Ok(&self.token.as_ref().expect("should never fail").access_token)
    }

    fn fetch_token(&self, config: &Config, bearer: &TokenAuth) -> Result<TokenResponse, LoadError> {
        let token_params = TokenParams {
            grant_type: GrantType::ClientCredentials,
            client_id: bearer.client_id.to_owned(),
            client_secret: bearer.client_secret.to_owned(),
        };

        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded; charset=utf-8"),
        );

        let url = Self::format_oauth_token_url(&config.connection.base_url)?;
        let req = self.client.post(url).headers(headers).query(&token_params);

        let res = req.send()?;
        res.error_for_status_ref()?;

        Ok(res.json()?)
    }

    fn format_oauth_token_url(base_url: &Url) -> Result<Url, LoadError> {
        let endpoint = "/o/oauth2/token";
        Ok(base_url.join(endpoint)?)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]

struct TokenParams {
    grant_type: GrantType,
    client_id: String,
    client_secret: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: TokenType,
    expires_in: usize,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
enum TokenType {
    Bearer,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum GrantType {
    ClientCredentials,
}

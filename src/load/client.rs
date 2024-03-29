use crate::{
    config::{sub::connection::TokenAuth, Config},
    error::Reporter,
};
use liferay_object::models::ObjectDefinition;
use list_type::models::ListTypeDefinition;
use oas::OpenAPIV3;
use reqwest::{
    blocking::RequestBuilder,
    header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE},
};
use serde::{Deserialize, Serialize};

use super::{error::Error, format};

/// A Liferay api client
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

    /// Load an open api specification from a remote Liferay instance
    pub fn get_spec(&mut self, config: &Config, endpoint: String) -> Result<OpenAPIV3, Error> {
        let mut req = self.client.get(config.connection.base_url.join(&endpoint)?);
        req = self.set_auth(req, config)?;
        req.send()
            .map_err(Error::Send)?
            .json()
            .map_err(|e| Error::Deserialize(e, "openapi spec"))
    }

    /// Set the internal auth configuration for the client
    fn set_auth(&mut self, req: RequestBuilder, config: &Config) -> Result<RequestBuilder, Error> {
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

    /// Load an object definition from a remote Liferay instance
    pub fn get_def(&mut self, config: &Config) -> Result<ObjectDefinition, Error> {
        let url = format::object_def_url(&config.connection.base_url, &config.source.erc)?;
        let mut req = self.client.get(url);

        req = self.set_auth(req, config)?;

        let res = req.send().map_err(Error::Send)?;

        res.error_for_status_ref()?;

        res.json()
            .map_err(|e| Error::Deserialize(e, "object definition"))
    }

    /// Load a picklist definition from a remote Liferay instance
    pub fn get_picklist(
        &mut self,
        config: &Config,
        erc: &str,
    ) -> Result<ListTypeDefinition, Error> {
        let url = format::picklist_url(&config.connection.base_url, erc)?;
        let mut req = self.client.get(url);

        req = self.set_auth(req, config)?;

        let res = req.send().map_err(Error::Send)?;

        res.error_for_status_ref()?;

        res.json()
            .map_err(|e| Error::Deserialize(e, "picklist definition"))
    }

    /// Load a set of picklists from a remote Liferay instance
    pub fn get_picklists(&mut self, config: &Config, ercs: Vec<String>) -> Vec<ListTypeDefinition> {
        let (picklists, errors): (Vec<_>, Vec<_>) = ercs
            .iter()
            .map(|erc| self.get_picklist(config, erc))
            .partition(Result::is_ok);

        let picklists: Vec<_> = picklists.into_iter().map(Result::unwrap).collect();
        let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();

        for error in errors {
            error.report("WARNING: failed to load picklist")
        }

        picklists
    }

    /// Get an oauth token from a remote Liferay instance if it hasn't been fetched yet.
    /// Otherwise, fetch it.
    fn get_token(&mut self, config: &Config, bearer: &TokenAuth) -> Result<&str, Error> {
        if self.token.is_none() {
            self.token = Some(self.fetch_token(config, bearer)?);
        }
        Ok(&self.token.as_ref().expect("should never fail").access_token)
    }

    /// Fetch an oauth token from a remote Liferay instance
    fn fetch_token(&self, config: &Config, bearer: &TokenAuth) -> Result<TokenResponse, Error> {
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

        let url = format::oauth_token_url(&config.connection.base_url)?;
        let req = self.client.post(url).headers(headers).query(&token_params);

        let res = req.send()?;
        res.error_for_status_ref()?;

        Ok(res.json()?)
    }
}

/// Params for retrieving an oauth token from a remote Liferay instance
#[derive(Debug, Clone, PartialEq, Serialize)]

struct TokenParams {
    grant_type: GrantType,
    client_id: String,
    client_secret: String,
}

/// The response data from an oauth token response
#[derive(Debug, Clone, PartialEq, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: TokenType,
    expires_in: usize,
}

/// Enum representing different types of oauth tokens
#[derive(Debug, Clone, PartialEq, Deserialize)]
enum TokenType {
    Bearer,
}

/// Enum representing different types of oauth grant types
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum GrantType {
    ClientCredentials,
}

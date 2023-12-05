use url::Url;

use super::Error;

pub fn picklist_url(base_url: &Url, erc: &str) -> Result<Url, Error> {
    let endpoint = format!(
        "/o/headless-admin-list-type/v1.0/list-type-definitions/by-external-reference-code/{erc}"
    );

    Ok(base_url.join(&endpoint)?)
}

pub fn object_def_url(base_url: &Url, erc: &str) -> Result<Url, Error> {
    let endpoint =
        format!("/o/object-admin/v1.0/object-definitions/by-external-reference-code/{erc}");

    Ok(base_url.join(&endpoint)?)
}

pub fn oauth_token_url(base_url: &Url) -> Result<Url, Error> {
    let endpoint = "/o/oauth2/token";
    Ok(base_url.join(endpoint)?)
}

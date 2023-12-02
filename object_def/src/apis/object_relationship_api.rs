use serde::{Deserialize, Serialize};
/*
 * Object
 *
 * A Java client JAR is available for use with the group ID 'com.liferay', artifact ID 'com.liferay.object.admin.rest.client', and version '1.0.61'.
 *
 * The version of the OpenAPI document: v1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for typed errors of method [`delete_object_relationship`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteObjectRelationshipError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_object_relationship_batch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteObjectRelationshipBatchError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_object_definition_by_external_reference_code_object_relationships_page`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetObjectDefinitionByExternalReferenceCodeObjectRelationshipsPageError {
    DefaultResponse(crate::models::PageObjectRelationship),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_object_definition_object_relationships_page`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetObjectDefinitionObjectRelationshipsPageError {
    DefaultResponse(crate::models::PageObjectRelationship),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_object_relationship`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetObjectRelationshipError {
    DefaultResponse(crate::models::ObjectRelationship),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_object_definition_by_external_reference_code_object_relationship`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostObjectDefinitionByExternalReferenceCodeObjectRelationshipError {
    DefaultResponse(crate::models::ObjectRelationship),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_object_definition_object_relationship`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostObjectDefinitionObjectRelationshipError {
    DefaultResponse(crate::models::ObjectRelationship),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_object_definition_object_relationship_batch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostObjectDefinitionObjectRelationshipBatchError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`post_object_definition_object_relationships_page_export_batch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PostObjectDefinitionObjectRelationshipsPageExportBatchError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_object_relationship`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutObjectRelationshipError {
    DefaultResponse(crate::models::ObjectRelationship),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_object_relationship_batch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutObjectRelationshipBatchError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

pub async fn delete_object_relationship(
    configuration: &configuration::Configuration,
    object_relationship_id: &str,
) -> Result<(), Error<DeleteObjectRelationshipError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1.0/object-relationships/{objectRelationshipId}",
        local_var_configuration.base_path,
        objectRelationshipId = crate::apis::urlencode(object_relationship_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteObjectRelationshipError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_object_relationship_batch(
    configuration: &configuration::Configuration,
    callback_url: Option<&str>,
    body: Option<serde_json::Value>,
) -> Result<(), Error<DeleteObjectRelationshipBatchError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1.0/object-relationships/batch",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = callback_url {
        local_var_req_builder =
            local_var_req_builder.query(&[("callbackURL", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteObjectRelationshipBatchError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_object_definition_by_external_reference_code_object_relationships_page(
    configuration: &configuration::Configuration,
    external_reference_code: &str,
    filter: Option<&str>,
    page: Option<&str>,
    page_size: Option<&str>,
    search: Option<&str>,
    sort: Option<&str>,
) -> Result<
    crate::models::PageObjectRelationship,
    Error<GetObjectDefinitionByExternalReferenceCodeObjectRelationshipsPageError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1.0/object-definitions/by-external-reference-code/{externalReferenceCode}/object-relationships", local_var_configuration.base_path, externalReferenceCode=crate::apis::urlencode(external_reference_code));
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder =
            local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("pageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = search {
        local_var_req_builder =
            local_var_req_builder.query(&[("search", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder =
            local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<
            GetObjectDefinitionByExternalReferenceCodeObjectRelationshipsPageError,
        > = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_object_definition_object_relationships_page(
    configuration: &configuration::Configuration,
    object_definition_id: &str,
    filter: Option<&str>,
    page: Option<&str>,
    page_size: Option<&str>,
    search: Option<&str>,
    sort: Option<&str>,
) -> Result<
    crate::models::PageObjectRelationship,
    Error<GetObjectDefinitionObjectRelationshipsPageError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1.0/object-definitions/{objectDefinitionId}/object-relationships",
        local_var_configuration.base_path,
        objectDefinitionId = crate::apis::urlencode(object_definition_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder =
            local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("pageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = search {
        local_var_req_builder =
            local_var_req_builder.query(&[("search", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder =
            local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetObjectDefinitionObjectRelationshipsPageError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_object_relationship(
    configuration: &configuration::Configuration,
    object_relationship_id: &str,
) -> Result<crate::models::ObjectRelationship, Error<GetObjectRelationshipError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1.0/object-relationships/{objectRelationshipId}",
        local_var_configuration.base_path,
        objectRelationshipId = crate::apis::urlencode(object_relationship_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetObjectRelationshipError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn post_object_definition_by_external_reference_code_object_relationship(
    configuration: &configuration::Configuration,
    external_reference_code: &str,
    object_relationship: Option<crate::models::ObjectRelationship>,
) -> Result<
    crate::models::ObjectRelationship,
    Error<PostObjectDefinitionByExternalReferenceCodeObjectRelationshipError>,
> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1.0/object-definitions/by-external-reference-code/{externalReferenceCode}/object-relationships", local_var_configuration.base_path, externalReferenceCode=crate::apis::urlencode(external_reference_code));
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&object_relationship);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<
            PostObjectDefinitionByExternalReferenceCodeObjectRelationshipError,
        > = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn post_object_definition_object_relationship(
    configuration: &configuration::Configuration,
    object_definition_id: &str,
    object_relationship: Option<crate::models::ObjectRelationship>,
) -> Result<crate::models::ObjectRelationship, Error<PostObjectDefinitionObjectRelationshipError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1.0/object-definitions/{objectDefinitionId}/object-relationships",
        local_var_configuration.base_path,
        objectDefinitionId = crate::apis::urlencode(object_definition_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&object_relationship);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PostObjectDefinitionObjectRelationshipError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn post_object_definition_object_relationship_batch(
    configuration: &configuration::Configuration,
    object_definition_id: &str,
    callback_url: Option<&str>,
    body: Option<serde_json::Value>,
) -> Result<(), Error<PostObjectDefinitionObjectRelationshipBatchError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1.0/object-definitions/{objectDefinitionId}/object-relationships/batch",
        local_var_configuration.base_path,
        objectDefinitionId = crate::apis::urlencode(object_definition_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = callback_url {
        local_var_req_builder =
            local_var_req_builder.query(&[("callbackURL", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<PostObjectDefinitionObjectRelationshipBatchError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn post_object_definition_object_relationships_page_export_batch(
    configuration: &configuration::Configuration,
    object_definition_id: &str,
    filter: Option<&str>,
    search: Option<&str>,
    sort: Option<&str>,
    callback_url: Option<&str>,
    content_type: Option<&str>,
    field_names: Option<&str>,
) -> Result<(), Error<PostObjectDefinitionObjectRelationshipsPageExportBatchError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1.0/object-definitions/{objectDefinitionId}/object-relationships/export-batch",
        local_var_configuration.base_path,
        objectDefinitionId = crate::apis::urlencode(object_definition_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = filter {
        local_var_req_builder =
            local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = search {
        local_var_req_builder =
            local_var_req_builder.query(&[("search", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder =
            local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = callback_url {
        local_var_req_builder =
            local_var_req_builder.query(&[("callbackURL", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = content_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("contentType", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = field_names {
        local_var_req_builder =
            local_var_req_builder.query(&[("fieldNames", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<PostObjectDefinitionObjectRelationshipsPageExportBatchError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn put_object_relationship(
    configuration: &configuration::Configuration,
    object_relationship_id: &str,
    object_relationship: Option<crate::models::ObjectRelationship>,
) -> Result<crate::models::ObjectRelationship, Error<PutObjectRelationshipError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1.0/object-relationships/{objectRelationshipId}",
        local_var_configuration.base_path,
        objectRelationshipId = crate::apis::urlencode(object_relationship_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&object_relationship);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PutObjectRelationshipError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn put_object_relationship_batch(
    configuration: &configuration::Configuration,
    callback_url: Option<&str>,
    body: Option<serde_json::Value>,
) -> Result<(), Error<PutObjectRelationshipBatchError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1.0/object-relationships/batch",
        local_var_configuration.base_path
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = callback_url {
        local_var_req_builder =
            local_var_req_builder.query(&[("callbackURL", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<PutObjectRelationshipBatchError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

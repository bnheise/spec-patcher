/*
 * List Type
 *
 * A Java client JAR is available for use with the group ID 'com.liferay', artifact ID 'om.liferay.headless.admin.list.type.client', and version '1.0.0'.. A Java client JAR is available for use with the group ID 'com.liferay', artifact ID 'com.liferay.headless.admin.list.type.client', and version '1.0.23'.
 *
 * The version of the OpenAPI document: v1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListTypeEntry {
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions:
        Option<::std::collections::HashMap<String, ::std::collections::HashMap<String, String>>>,
    #[serde(rename = "dateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "dateModified", skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
    #[serde(
        rename = "externalReferenceCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub external_reference_code: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "name_i18n", skip_serializing_if = "Option::is_none")]
    pub name_i18n: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "x-class-name", skip_serializing_if = "Option::is_none")]
    pub x_class_name: Option<String>,
}

impl ListTypeEntry {
    pub fn new() -> ListTypeEntry {
        ListTypeEntry {
            actions: None,
            date_created: None,
            date_modified: None,
            external_reference_code: None,
            id: None,
            key: None,
            name: None,
            name_i18n: None,
            r#type: None,
            x_class_name: None,
        }
    }
}

impl Default for ListTypeEntry {
    fn default() -> Self {
        Self::new()
    }
}

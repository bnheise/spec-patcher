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
pub struct PageListTypeEntry {
    #[serde(rename = "lastPage", skip_serializing_if = "Option::is_none")]
    pub last_page: Option<i64>,
    #[serde(rename = "totalCount", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::ListTypeEntry>>,
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "facets", skip_serializing_if = "Option::is_none")]
    pub facets: Option<Vec<crate::models::Facet>>,
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions:
        Option<::std::collections::HashMap<String, ::std::collections::HashMap<String, String>>>,
}

impl PageListTypeEntry {
    pub fn new() -> PageListTypeEntry {
        PageListTypeEntry {
            last_page: None,
            total_count: None,
            items: None,
            page_size: None,
            page: None,
            facets: None,
            actions: None,
        }
    }
}

impl Default for PageListTypeEntry {
    fn default() -> Self {
        Self::new()
    }
}
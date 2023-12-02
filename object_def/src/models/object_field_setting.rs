/*
 * Object
 *
 * A Java client JAR is available for use with the group ID 'com.liferay', artifact ID 'com.liferay.object.admin.rest.client', and version '1.0.61'.
 *
 * The version of the OpenAPI document: v1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObjectFieldSetting {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "objectFieldId", skip_serializing_if = "Option::is_none")]
    pub object_field_id: Option<i64>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[serde(rename = "x-class-name", skip_serializing_if = "Option::is_none")]
    pub x_class_name: Option<String>,
}

impl ObjectFieldSetting {
    pub fn new() -> ObjectFieldSetting {
        ObjectFieldSetting {
            id: None,
            name: None,
            object_field_id: None,
            value: None,
            x_class_name: None,
        }
    }
}



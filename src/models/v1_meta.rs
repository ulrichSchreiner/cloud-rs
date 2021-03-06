/*
 * cloud-api
 *
 * Resource for managing cloud entities
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: devops@f-i-ts.de
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct V1Meta {
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "apiversion", skip_serializing_if = "Option::is_none")]
    pub apiversion: Option<String>,
    #[serde(rename = "created_time", skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(rename = "updated_time", skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

impl V1Meta {
    pub fn new() -> V1Meta {
        V1Meta {
            annotations: None,
            apiversion: None,
            created_time: None,
            id: None,
            kind: None,
            labels: None,
            updated_time: None,
            version: None,
        }
    }
}



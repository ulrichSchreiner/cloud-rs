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
pub struct V1ProjectCreateRequest {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::V1Meta>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "quotas", skip_serializing_if = "Option::is_none")]
    pub quotas: Option<Box<crate::models::V1QuotaSet>>,
    #[serde(rename = "tenant_id", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}

impl V1ProjectCreateRequest {
    pub fn new() -> V1ProjectCreateRequest {
        V1ProjectCreateRequest {
            description: None,
            meta: None,
            name: None,
            quotas: None,
            tenant_id: None,
        }
    }
}


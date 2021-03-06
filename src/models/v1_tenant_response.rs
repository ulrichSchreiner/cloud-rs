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
pub struct V1TenantResponse {
    #[serde(rename = "tenant", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Box<crate::models::V1Tenant>>,
}

impl V1TenantResponse {
    pub fn new() -> V1TenantResponse {
        V1TenantResponse {
            tenant: None,
        }
    }
}



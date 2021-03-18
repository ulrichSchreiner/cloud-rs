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
pub struct ModelsV1SizeResponse {
    #[serde(rename = "changed", skip_serializing_if = "Option::is_none")]
    pub changed: Option<String>,
    #[serde(rename = "constraints")]
    pub constraints: Vec<crate::models::ModelsV1SizeConstraint>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ModelsV1SizeResponse {
    pub fn new(constraints: Vec<crate::models::ModelsV1SizeConstraint>, id: String) -> ModelsV1SizeResponse {
        ModelsV1SizeResponse {
            changed: None,
            constraints,
            created: None,
            description: None,
            id,
            name: None,
        }
    }
}


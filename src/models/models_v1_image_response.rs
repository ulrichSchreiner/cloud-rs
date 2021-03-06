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
pub struct ModelsV1ImageResponse {
    #[serde(rename = "changed", skip_serializing_if = "Option::is_none")]
    pub changed: Option<String>,
    #[serde(rename = "classification", skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "expirationDate")]
    pub expiration_date: String,
    #[serde(rename = "features")]
    pub features: Vec<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "usedby")]
    pub usedby: Vec<String>,
}

impl ModelsV1ImageResponse {
    pub fn new(expiration_date: String, features: Vec<String>, id: String, usedby: Vec<String>) -> ModelsV1ImageResponse {
        ModelsV1ImageResponse {
            changed: None,
            classification: None,
            created: None,
            description: None,
            expiration_date,
            features,
            id,
            name: None,
            url: None,
            usedby,
        }
    }
}



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
pub struct ModelsV1PartitionResponse {
    #[serde(rename = "bootconfig")]
    pub bootconfig: Box<crate::models::ModelsV1PartitionBootConfiguration>,
    #[serde(rename = "changed", skip_serializing_if = "Option::is_none")]
    pub changed: Option<String>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "mgmtserviceaddress", skip_serializing_if = "Option::is_none")]
    pub mgmtserviceaddress: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "privatenetworkprefixlength", skip_serializing_if = "Option::is_none")]
    pub privatenetworkprefixlength: Option<i64>,
}

impl ModelsV1PartitionResponse {
    pub fn new(bootconfig: crate::models::ModelsV1PartitionBootConfiguration, id: String) -> ModelsV1PartitionResponse {
        ModelsV1PartitionResponse {
            bootconfig: Box::new(bootconfig),
            changed: None,
            created: None,
            description: None,
            id,
            mgmtserviceaddress: None,
            name: None,
            privatenetworkprefixlength: None,
        }
    }
}



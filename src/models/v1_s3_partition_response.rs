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
pub struct V1S3PartitionResponse {
    #[serde(rename = "endpoint")]
    pub endpoint: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "ready")]
    pub ready: bool,
}

impl V1S3PartitionResponse {
    pub fn new(endpoint: String, id: String, ready: bool) -> V1S3PartitionResponse {
        V1S3PartitionResponse {
            endpoint,
            id,
            ready,
        }
    }
}



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
pub struct V1S3GetRequest {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "partition")]
    pub partition: String,
    #[serde(rename = "project")]
    pub project: String,
    #[serde(rename = "tenant")]
    pub tenant: String,
}

impl V1S3GetRequest {
    pub fn new(id: String, partition: String, project: String, tenant: String) -> V1S3GetRequest {
        V1S3GetRequest {
            id,
            partition,
            project,
            tenant,
        }
    }
}



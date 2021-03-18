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
pub struct V1S3DeleteRequest {
    #[serde(rename = "force")]
    pub force: bool,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "partition")]
    pub partition: String,
    #[serde(rename = "project")]
    pub project: String,
    #[serde(rename = "tenant")]
    pub tenant: String,
}

impl V1S3DeleteRequest {
    pub fn new(force: bool, id: String, partition: String, project: String, tenant: String) -> V1S3DeleteRequest {
        V1S3DeleteRequest {
            force,
            id,
            partition,
            project,
            tenant,
        }
    }
}


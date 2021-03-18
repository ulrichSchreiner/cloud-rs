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
pub struct V1S3CreateRequest {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "key")]
    pub key: Box<crate::models::V1S3Key>,
    #[serde(rename = "max_buckets")]
    pub max_buckets: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "partition")]
    pub partition: String,
    #[serde(rename = "project")]
    pub project: String,
    #[serde(rename = "tenant")]
    pub tenant: String,
}

impl V1S3CreateRequest {
    pub fn new(id: String, key: crate::models::V1S3Key, max_buckets: i64, name: String, partition: String, project: String, tenant: String) -> V1S3CreateRequest {
        V1S3CreateRequest {
            id,
            key: Box::new(key),
            max_buckets,
            name,
            partition,
            project,
            tenant,
        }
    }
}



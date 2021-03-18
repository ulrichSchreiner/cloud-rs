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
pub struct V1S3ListRequest {
    #[serde(rename = "partition")]
    pub partition: String,
}

impl V1S3ListRequest {
    pub fn new(partition: String) -> V1S3ListRequest {
        V1S3ListRequest {
            partition,
        }
    }
}


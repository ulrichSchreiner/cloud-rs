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
pub struct V1S3Key {
    #[serde(rename = "access_key")]
    pub access_key: String,
    #[serde(rename = "secret_key")]
    pub secret_key: String,
}

impl V1S3Key {
    pub fn new(access_key: String, secret_key: String) -> V1S3Key {
        V1S3Key {
            access_key,
            secret_key,
        }
    }
}



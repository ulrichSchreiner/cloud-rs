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
pub struct RestStatus {
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "status")]
    pub status: String,
}

impl RestStatus {
    pub fn new(message: String, status: String) -> RestStatus {
        RestStatus {
            message,
            status,
        }
    }
}



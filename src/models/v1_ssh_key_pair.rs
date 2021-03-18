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
pub struct V1SshKeyPair {
    #[serde(rename = "PrivateKey")]
    pub private_key: String,
    #[serde(rename = "PublicKey")]
    pub public_key: String,
}

impl V1SshKeyPair {
    pub fn new(private_key: String, public_key: String) -> V1SshKeyPair {
        V1SshKeyPair {
            private_key,
            public_key,
        }
    }
}


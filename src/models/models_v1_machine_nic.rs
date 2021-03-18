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
pub struct ModelsV1MachineNic {
    #[serde(rename = "mac")]
    pub mac: String,
    #[serde(rename = "name")]
    pub name: String,
}

impl ModelsV1MachineNic {
    pub fn new(mac: String, name: String) -> ModelsV1MachineNic {
        ModelsV1MachineNic {
            mac,
            name,
        }
    }
}


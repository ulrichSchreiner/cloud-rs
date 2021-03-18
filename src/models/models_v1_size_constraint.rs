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
pub struct ModelsV1SizeConstraint {
    #[serde(rename = "max")]
    pub max: i64,
    #[serde(rename = "min")]
    pub min: i64,
    #[serde(rename = "type")]
    pub _type: String,
}

impl ModelsV1SizeConstraint {
    pub fn new(max: i64, min: i64, _type: String) -> ModelsV1SizeConstraint {
        ModelsV1SizeConstraint {
            max,
            min,
            _type,
        }
    }
}


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
pub struct ModelsV1MachineHardware {
    #[serde(rename = "cpu_cores")]
    pub cpu_cores: i64,
    #[serde(rename = "disks")]
    pub disks: Vec<crate::models::ModelsV1MachineBlockDevice>,
    #[serde(rename = "memory")]
    pub memory: i64,
    #[serde(rename = "nics")]
    pub nics: Vec<crate::models::ModelsV1MachineNic>,
}

impl ModelsV1MachineHardware {
    pub fn new(cpu_cores: i64, disks: Vec<crate::models::ModelsV1MachineBlockDevice>, memory: i64, nics: Vec<crate::models::ModelsV1MachineNic>) -> ModelsV1MachineHardware {
        ModelsV1MachineHardware {
            cpu_cores,
            disks,
            memory,
            nics,
        }
    }
}



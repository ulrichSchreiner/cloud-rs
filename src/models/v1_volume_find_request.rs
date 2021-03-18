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
pub struct V1VolumeFindRequest {
    #[serde(rename = "PartitionID")]
    pub partition_id: String,
    #[serde(rename = "ProjectID")]
    pub project_id: String,
    #[serde(rename = "VolumeID")]
    pub volume_id: String,
}

impl V1VolumeFindRequest {
    pub fn new(partition_id: String, project_id: String, volume_id: String) -> V1VolumeFindRequest {
        V1VolumeFindRequest {
            partition_id,
            project_id,
            volume_id,
        }
    }
}


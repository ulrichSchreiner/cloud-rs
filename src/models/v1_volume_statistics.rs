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
pub struct V1VolumeStatistics {
    #[serde(rename = "CompressionRatio")]
    pub compression_ratio: f64,
    #[serde(rename = "LogicalUsedStorage")]
    pub logical_used_storage: i64,
    #[serde(rename = "PhysicalUsedStorage")]
    pub physical_used_storage: i64,
}

impl V1VolumeStatistics {
    pub fn new(compression_ratio: f64, logical_used_storage: i64, physical_used_storage: i64) -> V1VolumeStatistics {
        V1VolumeStatistics {
            compression_ratio,
            logical_used_storage,
            physical_used_storage,
        }
    }
}


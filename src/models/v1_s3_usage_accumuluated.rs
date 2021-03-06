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
pub struct V1S3UsageAccumuluated {
    /// the accumulated reported number of objects of the s3 buckets in this response
    #[serde(rename = "currentnumberofobjects")]
    pub currentnumberofobjects: String,
    /// the accumulated reported size of the s3 buckets in this response
    #[serde(rename = "currentsize")]
    pub currentsize: String,
    /// the duration that this s3 bucket is running
    #[serde(rename = "lifetime")]
    pub lifetime: i64,
    /// the accumulated storage seconds of the s3 buckets in this response (byte*s)
    #[serde(rename = "storageseconds")]
    pub storageseconds: String,
}

impl V1S3UsageAccumuluated {
    pub fn new(currentnumberofobjects: String, currentsize: String, lifetime: i64, storageseconds: String) -> V1S3UsageAccumuluated {
        V1S3UsageAccumuluated {
            currentnumberofobjects,
            currentsize,
            lifetime,
            storageseconds,
        }
    }
}



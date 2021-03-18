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
pub struct V1S3UsageResponse {
    #[serde(rename = "accumulatedusage")]
    pub accumulatedusage: Box<crate::models::V1S3UsageAccumuluated>,
    /// the start time in the accounting window to look at
    #[serde(rename = "from")]
    pub from: String,
    /// the end time in the accounting window to look at (defaults to current system time)
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// the usage data of the individual s3 buckets
    #[serde(rename = "usage")]
    pub usage: Vec<crate::models::V1S3Usage>,
}

impl V1S3UsageResponse {
    pub fn new(accumulatedusage: crate::models::V1S3UsageAccumuluated, from: String, usage: Vec<crate::models::V1S3Usage>) -> V1S3UsageResponse {
        V1S3UsageResponse {
            accumulatedusage: Box::new(accumulatedusage),
            from,
            to: None,
            usage,
        }
    }
}


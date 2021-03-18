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
pub struct V1IpUsageRequest {
    /// the start time in the accounting window to look at
    #[serde(rename = "from")]
    pub from: String,
    /// the project id to account for
    #[serde(rename = "projectid", skip_serializing_if = "Option::is_none")]
    pub projectid: Option<String>,
    /// the tenant to get the container usage for (defaults to all tenants)
    #[serde(rename = "tenant", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
    /// the end time in the accounting window to look at (defaults to current system time)
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

impl V1IpUsageRequest {
    pub fn new(from: String) -> V1IpUsageRequest {
        V1IpUsageRequest {
            from,
            projectid: None,
            tenant: None,
            to: None,
        }
    }
}


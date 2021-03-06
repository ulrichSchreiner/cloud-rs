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
pub struct V1NetworkUsage {
    /// the cluster id of this network device
    #[serde(rename = "clusterid")]
    pub clusterid: String,
    /// the cluster name of this network device
    #[serde(rename = "clustername")]
    pub clustername: String,
    /// the device name of this network device
    #[serde(rename = "device")]
    pub device: String,
    /// the ingoing traffic of this network device (byte)
    #[serde(rename = "in")]
    pub _in: String,
    /// the duration that this network device is running
    #[serde(rename = "lifetime")]
    pub lifetime: i64,
    /// the outgoing traffic of this network device (byte)
    #[serde(rename = "out")]
    pub out: String,
    /// the partition of this network device
    #[serde(rename = "partition")]
    pub partition: String,
    /// the project id of this network device
    #[serde(rename = "projectid")]
    pub projectid: String,
    /// the project name of this network device
    #[serde(rename = "projectname")]
    pub projectname: String,
    /// the tenant of this network device
    #[serde(rename = "tenant")]
    pub tenant: String,
    /// the total traffic of this network device (byte)
    #[serde(rename = "total")]
    pub total: String,
    /// warnings that occurred when calculating the usage of this device's network traffic
    #[serde(rename = "warnings")]
    pub warnings: Vec<String>,
}

impl V1NetworkUsage {
    pub fn new(clusterid: String, clustername: String, device: String, _in: String, lifetime: i64, out: String, partition: String, projectid: String, projectname: String, tenant: String, total: String, warnings: Vec<String>) -> V1NetworkUsage {
        V1NetworkUsage {
            clusterid,
            clustername,
            device,
            _in,
            lifetime,
            out,
            partition,
            projectid,
            projectname,
            tenant,
            total,
            warnings,
        }
    }
}



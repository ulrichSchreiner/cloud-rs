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
pub struct V1VolumeUsage {
    /// accounting annotations present on the last accounting report of this volume
    #[serde(rename = "annotations")]
    pub annotations: Vec<String>,
    /// the capacity seconds of this volume (byte*s)
    #[serde(rename = "capacityseconds")]
    pub capacityseconds: String,
    /// the class of this volume
    #[serde(rename = "class")]
    pub class: String,
    /// the cluster id of this volume
    #[serde(rename = "clusterid")]
    pub clusterid: String,
    /// the cluster name of this volume
    #[serde(rename = "clustername")]
    pub clustername: String,
    /// the end time of this volume
    #[serde(rename = "end")]
    pub end: String,
    /// the duration that this volume is running
    #[serde(rename = "lifetime")]
    pub lifetime: i64,
    /// the name of this volume
    #[serde(rename = "name")]
    pub name: String,
    /// the partition of this volume
    #[serde(rename = "partition")]
    pub partition: String,
    /// the project id of this volume
    #[serde(rename = "projectid")]
    pub projectid: String,
    /// the project name of this volume
    #[serde(rename = "projectname")]
    pub projectname: String,
    /// the start time of this volume
    #[serde(rename = "start")]
    pub start: String,
    /// the tenant of this volume
    #[serde(rename = "tenant")]
    pub tenant: String,
    /// the type of this volume
    #[serde(rename = "type")]
    pub _type: String,
    /// the uuid of this volume
    #[serde(rename = "uuid")]
    pub uuid: String,
    /// warnings that occurred when calculating the usage of this volume
    #[serde(rename = "warnings")]
    pub warnings: Vec<String>,
}

impl V1VolumeUsage {
    pub fn new(annotations: Vec<String>, capacityseconds: String, class: String, clusterid: String, clustername: String, end: String, lifetime: i64, name: String, partition: String, projectid: String, projectname: String, start: String, tenant: String, _type: String, uuid: String, warnings: Vec<String>) -> V1VolumeUsage {
        V1VolumeUsage {
            annotations,
            capacityseconds,
            class,
            clusterid,
            clustername,
            end,
            lifetime,
            name,
            partition,
            projectid,
            projectname,
            start,
            tenant,
            _type,
            uuid,
            warnings,
        }
    }
}



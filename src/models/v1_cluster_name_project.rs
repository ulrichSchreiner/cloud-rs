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
pub struct V1ClusterNameProject {
    /// cluster name
    #[serde(rename = "cluster_name", skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    /// generated middle-part of gardener shoot namespace, e.g. 'ps5d42'
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl V1ClusterNameProject {
    pub fn new() -> V1ClusterNameProject {
        V1ClusterNameProject {
            cluster_name: None,
            project: None,
        }
    }
}



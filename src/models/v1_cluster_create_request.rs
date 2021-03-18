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
pub struct V1ClusterCreateRequest {
    #[serde(rename = "AdditionalNetworks")]
    pub additional_networks: Vec<String>,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "EgressRules")]
    pub egress_rules: Vec<crate::models::V1EgressRule>,
    #[serde(rename = "FirewallControllerVersion")]
    pub firewall_controller_version: String,
    #[serde(rename = "FirewallImage")]
    pub firewall_image: String,
    #[serde(rename = "FirewallSize")]
    pub firewall_size: String,
    #[serde(rename = "Kubernetes")]
    pub kubernetes: Box<crate::models::V1Kubernetes>,
    #[serde(rename = "Labels")]
    pub labels: ::std::collections::HashMap<String, String>,
    #[serde(rename = "Maintenance")]
    pub maintenance: Box<crate::models::V1Maintenance>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "PartitionID")]
    pub partition_id: String,
    #[serde(rename = "ProjectID")]
    pub project_id: String,
    #[serde(rename = "Purpose")]
    pub purpose: String,
    #[serde(rename = "Tenant")]
    pub tenant: String,
    #[serde(rename = "Workers")]
    pub workers: Vec<crate::models::V1Worker>,
}

impl V1ClusterCreateRequest {
    pub fn new(additional_networks: Vec<String>, description: String, egress_rules: Vec<crate::models::V1EgressRule>, firewall_controller_version: String, firewall_image: String, firewall_size: String, kubernetes: crate::models::V1Kubernetes, labels: ::std::collections::HashMap<String, String>, maintenance: crate::models::V1Maintenance, name: String, partition_id: String, project_id: String, purpose: String, tenant: String, workers: Vec<crate::models::V1Worker>) -> V1ClusterCreateRequest {
        V1ClusterCreateRequest {
            additional_networks,
            description,
            egress_rules,
            firewall_controller_version,
            firewall_image,
            firewall_size,
            kubernetes: Box::new(kubernetes),
            labels,
            maintenance: Box::new(maintenance),
            name,
            partition_id,
            project_id,
            purpose,
            tenant,
            workers,
        }
    }
}



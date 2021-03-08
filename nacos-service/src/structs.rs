use serde::{Deserialize, Serialize};

type Weight = f32;

#[derive(Builder, Debug, Deserialize, Serialize)]
#[builder(setter(into))]
pub struct RegisterInstance {
    ip: String,
    port: u16,
    #[serde(rename = "namepsaceId")]
    namespace_id: Option<String>,
    #[serde(rename = "serviceName")]
    service_name: String,
    weight: Weight,
    enabled: bool,
    healthy: bool,
    metadata: Option<String>,
    #[serde(rename = "clusterName")]
    cluster_name: Option<String>,
    #[serde(rename = "groupName")]
    group_name: Option<String>,
    ephemeral: bool,
}

#[derive(Builder, Debug, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct DeregistryInstance {
    #[serde(rename = "serviceName")]
    service_name: String,
    #[serde(rename = "groupName")]
    group_name: Option<String>,
    ephemeral: bool,
    ip: String,
    port: u16,
    #[serde(rename = "clusterName")]
    cluster_name: String,
    #[serde(rename = "namespaceId")]
    namespace_id: String,
}

#[derive(Builder, Debug, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct ModifyInstance {
    #[serde(rename = "serviceName")]
    service_name: String,
    #[serde(rename = "groupName")]
    group_name: String,
    ephemeral: bool,
    ip: String,
    port: u16,
    #[serde(rename = "clusterName")]
    cluster_name: String,
    #[serde(rename = "namespaceId")]
    namespace_id: String,
    weight: Weight,
    enabled: bool,
    metadata: String,
}

#[derive(Builder, Debug, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct QueryInstance {
    #[serde(rename = "serviceName")]
    service_name: String,
    #[serde(rename = "groupName")]
    group_name: String,
    #[serde(rename = "namespaceId")]
    namespace_id: String,
    clusters: String,
    #[serde(rename = "healthyOnly")]
    healthy_only: bool,
}

#[derive(Builder, Debug, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct QueryInstanceDetail {
    #[serde(rename = "namespaceId")]
    namespace_id: String,
    #[serde(rename = "serviceName")]
    service_name: String,
    #[serde(rename = "groupName")]
    group_name: String,
    ephemeral: bool,
    ip: String,
    port: String,
    cluster: String,
}

#[derive(Builder, Debug, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct SendInstanceBeat {
    #[serde(rename = "namespaceId")]
    namespace_id: String,
    #[serde(rename = "serviceName")]
    service_name: String,
    #[serde(rename = "groupName")]
    group_name: String,
    beat: String,
}

#[derive(Builder, Debug, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct CreateService {
    #[serde(rename = "serviceName")]
    service_name: String,
    #[serde(rename = "groupName")]
    group_name: String,
    #[serde(rename = "namespaceId")]
    namespace_id: String,
    #[serde(rename = "protectThreshold")]
    protect_threshold: f32,
    #[serde(rename = "metadata")]
    metadata: String,
    selector: String,
}

#[derive(Builder, Debug, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct DeleteService {
    #[serde(rename = "serviceName")]
    service_name: String,
    #[serde(rename = "groupName")]
    group_name: String,
    #[serde(rename = "namespaceId")]
    namespace_id: String,
}

#[derive(Builder, Debug, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct UpdateService {
    #[serde(rename = "serviceName")]
    service_name: String,
    #[serde(rename = "groupName")]
    group_name: String,
    #[serde(rename = "namespaceId")]
    namespace_id: String,
    #[serde(rename = "protectThreshold")]
    protect_threshold: f32,
    metadata: String,
    selector: String,
}

#[derive(Builder, Debug, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct QueryService {
    #[serde(rename = "serviceName")]
    service_name: String,
    #[serde(rename = "groupName")]
    group_name: String,
    #[serde(rename = "namespaceId")]
    namespace_id: String,
}

#[derive(Builder, Debug, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct QueryServiceList {
    #[serde(rename = "pageNo")]
    page_no: u32,
    #[serde(rename = "pageSize")]
    page_size: u32,
    #[serde(rename = "groupName")]
    group_name: String,
    #[serde(rename = "namespaceId")]
    namespace_id: String,
}

#[derive(Builder, Debug, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct UpdateSystemSwitch {
    entry: String,
    value: String,
    debug: bool,
}

#[derive(Builder, Debug, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct UpdateInstanceHealthStatus {
    #[serde(rename = "serviceName")]
    service_name: String,
    #[serde(rename = "groupName")]
    group_name: String,
    #[serde(rename = "namespaceId")]
    namespace_id: String,
    #[serde(rename = "clusterName")]
    cluster_name: String,
    ip: String,
    port: u16,
    healthy: bool,
}

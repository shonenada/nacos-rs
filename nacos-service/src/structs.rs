use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterInstance {
    ip: String,
    port: u16,
    namespace_id: Option<String>,
    weight: f32,
    enabled: bool,
    healthy: bool,
    metadata: Option<String>,
    cluster_name: Option<String>,
    service_name: String,
    group_name: Option<String>,
    ephemeral: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnregistryInstance {
    service_name: String,
    group_name: Option<String>,
    ephemeral: bool,
    ip: String,
    port: u16,
    cluster_name: String,
    namespace_id: String,
}

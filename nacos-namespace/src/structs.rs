use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Namespace {
    pub namespace: String,
    #[serde(rename = "namespaceShowName")]
    pub namespace_show_name: String,
    pub quota: u32,
    #[serde(rename = "configCount")]
    pub config_count: u32,
    #[serde(rename = "type")]
    pub namespace_type: u32,
}

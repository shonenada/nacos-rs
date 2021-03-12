use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    tenant: String,
    #[serde(rename = "dataId")]
    data_id: String,
    group: String,
}

#[derive(Builder, Debug, Deserialize, Serialize)]
#[builder(setter(into))]
pub struct ListenConfig {
    #[serde(rename = "Listening-Configs")]
    listening_configs: String,
    tenant: String,
    #[serde(rename = "dataId")]
    data_id: String,
    group: String,
    #[serde(rename = "contentMD5")]
    content_md5: String,
}

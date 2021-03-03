use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    tenant: String,
    #[serde(rename = "dataId")]
    data_id: String,
    group: String,
}

use anyhow::Result;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct CommonResponse<T> {
    code: u16,
    message: Option<String>,
    data: T,
}

#[derive(Clone, Debug)]
pub struct NacosConfig {
    scheme: String,
    host: String,
    port: u16,
    context_path: String,
}

impl NacosConfig {
    pub fn new(scheme: &str, host: &str, port: u16, context_path: &str) -> Self {
        Self{
            scheme: scheme.to_string(),
            host: host.to_string(),
            port,
            context_path: context_path.to_string(),
        }
    }

    fn make_url(&self, sp: &str) -> String {
        let subpath;
        if !sp.starts_with("/") {
            subpath = format!("/{}", sp);
        } else {
            subpath = sp.to_string();
        }

        format!(
            "{}://{}:{}{}{}",
            self.scheme, self.host, self.port, self.context_path, subpath
        )
    }

    pub async fn get_config(&self, data_id: &str, group: &str, tenant: &str) -> Result<String> {
        let url = self.make_url("/v1/cs/configs");
        let client = reqwest::Client::new();
        let query = [
            ("dataId", data_id),
            ("group", group),
            ("tenant", tenant),
        ];
        let resp = client.get(&url).query(&query).send().await?;
        let status = resp.status();
        match status {
            StatusCode::OK => {
                let result: String  = resp.text().await?;
                Ok(result)
            },
            _ => {
                let body = resp.text().await?;
                Err(anyhow!(
                    "Failed to request {}, status code is {}, body: {}",
                    url,
                    status,
                    body
                ))
            }
        }
    }
}
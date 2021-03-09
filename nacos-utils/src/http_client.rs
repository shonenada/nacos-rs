use anyhow::Result;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

use crate::CommonResponse;

#[derive(Clone, Debug)]
pub struct NacosConfig {
    scheme: String,
    host: String,
    port: u16,
    context_path: String,
}

#[derive(Clone, Debug)]
pub struct NacosClient {
    config: NacosConfig,
}

impl NacosConfig {
    pub fn new(scheme: &str, host: &str, port: u16, context_path: &str) -> Self {
        Self {
            scheme: scheme.to_string(),
            host: host.to_string(),
            port,
            context_path: context_path.to_string(),
        }
    }

    pub fn make_url(&self, sp: &str) -> String {
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

    pub fn into_client(&self) -> NacosClient {
        NacosClient::new(self.clone())
    }
}

impl NacosClient {
    pub fn new(config: NacosConfig) -> Self {
        Self { config }
    }

    pub fn make_url(&self, sp: &str) -> String {
        self.config.make_url(sp)
    }

    pub async fn get_json<T>(&self, sp: &str) -> Result<CommonResponse<T>>
    where
        T: DeserializeOwned,
    {
        let url = self.make_url(sp);
        let resp = reqwest::get(&url).await?;
        let status = resp.status();
        match status {
            StatusCode::OK => {
                let body: CommonResponse<T> = resp.json().await?;
                Ok(body)
            }
            _ => {
                let body = resp.text().await?;
                Err(anyhow!(
                    "Failed to request {}, status code is {}, body: {}",
                    self.config.context_path,
                    status,
                    body
                ))
            }
        }
    }
}

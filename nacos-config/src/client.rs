use anyhow::Result;
use reqwest::StatusCode;

use nacos_utils::NacosHTTPClient;

use crate::structs::ListenConfig;

#[derive(Clone, Debug)]
pub struct NacosConfigClient {
    client: NacosHTTPClient,
}

impl NacosConfigClient {
    pub fn new(client: &NacosHTTPClient) -> Self {
        Self {
            client: client.clone(),
        }
    }

    fn make_url(&self, sp: &str) -> String {
        self.client.make_url(sp)
    }

    pub async fn get_config(&self, data_id: &str, group: &str, tenant: &str) -> Result<String> {
        let query = [("dataId", data_id), ("group", group), ("tenant", tenant)];
        self.client.get_string("/v1/cs/configs", &query).await
    }

    pub async fn publish_config(
        &self,
        data_id: &str,
        group: &str,
        content: &str,
        tenant: &str,
        config_type: &str,
    ) -> Result<()> {
        let params = [
            ("dataId", data_id),
            ("group", group),
            ("tenant", tenant),
            ("content", content),
            ("type", config_type),
        ];
        self.client.simple_post("/v1/cs/configs", &params).await
    }

    pub async fn delete_config(&self, data_id: &str, group: &str, tenant: &str) -> Result<()> {
        let params = [("dataId", data_id), ("group", group), ("tenant", tenant)];
        self.client.simple_delete("/v1/cs/configs", &params).await
    }

    pub async fn listen_config(&self, args: &ListenConfig) -> Result<String> {
        let url = self.make_url("/v1/cs/configs/listener");
        let client = reqwest::Client::new();
        let resp = client.post(&url).form(args).send().await?;
        let status = resp.status();
        match status {
            StatusCode::OK => {
                let result: String = resp.text().await?;
                Ok(result)
            }
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

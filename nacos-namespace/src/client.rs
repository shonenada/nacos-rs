use anyhow::Result;
use reqwest::StatusCode;

use nacos_utils::{NacosClient, NacosResponse};

use crate::structs::Namespace;

#[derive(Clone, Debug)]
pub struct NacosNamespace {
    client: NacosClient,
}

impl NacosNamespace {
    pub fn new(client: &NacosClient) -> Self {
        Self {
            client: client.clone(),
        }
    }

    fn make_url(&self, sp: &str) -> String {
        self.client.make_url(sp)
    }

    pub async fn list_namespaces(&self) -> Result<Vec<Namespace>> {
        let resp: NacosResponse<Vec<Namespace>> =
            self.client.get_json("/v1/console/namespaces").await?;
        Ok(resp.get_data())
    }

    pub async fn create_namespace(&self, ns_id: &str, name: &str, description: &str) -> Result<()> {
        let params = [
            ("customNamespaceId", ns_id),
            ("namespaceName", name),
            ("namespaceDesc", description),
        ];
        self.client
            .simple_post("/v1/console/namespaces", &params)
            .await
    }

    pub async fn update_namespace(
        &self,
        namespace_id: &str,
        name: &str,
        description: &str,
    ) -> Result<()> {
        let url = self.make_url("/v1/console/namespaces");
        let client = reqwest::Client::new();
        let params = [
            ("namespace", namespace_id),
            ("namespaceShowName", name),
            ("namespaceDesc", description),
        ];
        let resp = client.put(&url).form(&params).send().await?;
        let status = resp.status();
        match status {
            StatusCode::OK => {
                let body: String = resp.text().await?;
                if body == "true" {
                    Ok(())
                } else {
                    Err(anyhow!(
                        "Failed to request {}, status code is {}, body: {}",
                        url,
                        status,
                        body
                    ))
                }
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

    pub async fn delete_namespace(&self, namespace_id: &str) -> Result<()> {
        let url = self.make_url("/v1/console/namespaces");
        let client = reqwest::Client::new();
        let params = [("namespaceId", namespace_id)];
        let resp = client.delete(&url).form(&params).send().await?;
        let status = resp.status();
        match status {
            StatusCode::OK => {
                let body: String = resp.text().await?;
                if body == "true" {
                    Ok(())
                } else {
                    Err(anyhow!(
                        "Failed to request {}, status code is {}, body: {}",
                        url,
                        status,
                        body
                    ))
                }
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

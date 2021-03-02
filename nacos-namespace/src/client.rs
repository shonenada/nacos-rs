use anyhow::Result;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::structs::Namespace;

#[derive(Debug, Serialize, Deserialize)]
struct CommonResponse<T> {
    code: u16,
    message: Option<String>,
    data: T,
}

#[derive(Clone, Debug)]
pub struct NacosNamespace {
    scheme: String,
    host: String,
    port: u16,
    context_path: String,
}

impl NacosNamespace {
    pub fn new(scheme: &str, host: &str, port: u16, context_path: &str) -> Self {
        Self {
            scheme: scheme.to_string(),
            host: host.to_string(),
            port,
            context_path: context_path.to_string(),
        }
    }

    fn make_uri(&self) -> String {
        format!(
            "{}://{}:{}{}",
            self.scheme, self.host, self.port, self.context_path
        )
    }

    pub async fn list_namespaces(&self) -> Result<Vec<Namespace>> {
        let url = format!("{}/v1/console/namespaces", self.make_uri());
        let resp = reqwest::get(&url).await?;
        let status = resp.status();
        match status {
            StatusCode::OK => {
                let body: CommonResponse<Vec<Namespace>> = resp.json().await?;
                Ok(body.data)
            }
            _ => {
                let body = resp.text().await?;
                Err(anyhow!(
                    "Failed to request /{}/v1/console/namespaces, status code is {}, body: {}",
                    self.context_path,
                    status,
                    body
                ))
            }
        }
    }

    pub async fn create_namespace(
        &self,
        ns_id: String,
        name: String,
        description: String,
    ) -> Result<()> {
        let url = format!("{}/v1/console/namespaces", self.make_uri());
        let client = reqwest::Client::new();
        let params = [
            ("customNamespaceId", ns_id),
            ("namespaceName", name),
            ("namespaceDesc", description),
        ];
        let resp = client.post(&url).form(&params).send().await?;
        let status = resp.status();
        match status {
            StatusCode::OK => {
                let body: String = resp.text().await?;
                if body == "true" {
                    Ok(())
                } else {
                    Err(anyhow!(
                        "Failed to request {}/v1/console/namespaces, status code is {}, body: {}",
                        self.context_path,
                        status,
                        body
                    ))
                }
            }
            _ => {
                let body = resp.text().await?;
                Err(anyhow!(
                    "Failed to request /{}/v1/console/namespaces, status code is {}, body: {}",
                    self.context_path,
                    status,
                    body
                ))
            }
        }
    }
}

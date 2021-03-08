use anyhow::Result;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::structs::*;

#[derive(Debug, Serialize, Deserialize)]
struct CommonResponse<T> {
    code: u16,
    message: Option<String>,
    data: T,
}

#[derive(Clone, Debug)]
pub struct NacosService {
    scheme: String,
    host: String,
    port: u16,
    context_path: String,
}

impl NacosService {
    pub fn new(scheme: &str, host: &str, port: u16, context_path: &str) -> Self {
        Self {
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

    pub async fn register_instance(&self, args: &RegisterInstance) -> Result<()> {
        let url = self.make_url("/v1/ns/instance");
        let client = reqwest::Client::new();
        let resp = client.post(&url).form(args).send().await?;
        let status = resp.status();
        match status {
            StatusCode::OK => {
                let body: String = resp.text().await?;
                if body == "ok" {
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
                    "Failed to request {}, status coode is {}, body: {}",
                    url,
                    status,
                    body
                ))
            }
        }
    }

    pub async fn deregister_instance(&self, args: &DeregistryInstance) -> Result<()> {
        let url = self.make_url("/v1/ns/instance");
        let client = reqwest::Client::new();
        let resp = client.delete(&url).form(args).send().await?;
        let status = resp.status();
        match status {
            StatusCode::OK => {
                let body: String = resp.text().await?;
                if body == "ok" {
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

    pub async fn modify_instance(&self, args: &ModifyInstance) -> Result<()> {
        let url = self.make_url("/v1/ns/instance");
        let client = reqwest::Client::new();
        let resp = client.put(&url).form(args).send().await?;
        let status = resp.status();
        match status {
            StatusCode::OK => {
                let body: String = resp.text().await?;
                if body == "ok" {
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

    // TODO: FIXME String
    pub async fn query_instances(&self, args: &QueryInstance) -> Result<String> {
        let url = self.make_url("/v1/ns/instance/list");
        let client = reqwest::Client::new();
        let resp = client.get(&url).query(args).send().await?;
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

    // TODO: FIXME string
    pub async fn query_instance_detail(&self, args: &QueryInstanceDetail) -> Result<String> {
        let url = self.make_url("/v1/ns/instance");
        let client = reqwest::Client::new();
        let resp = client.get(&url).query(args).send().await?;
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

    pub async fn send_instance_beat(&self, args: &SendInstanceBeat) -> Result<()> {
        let url = self.make_url("/v1/ns/instance/beat");
        let client = reqwest::Client::new();
        let resp = client.put(&url).query(args).send().await?;
        let status = resp.status();
        match status {
            StatusCode::OK => {
                let body: String = resp.text().await?;
                if body == "ok" {
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

    pub async fn create_service(&self, args: &CreateService) -> Result<()> {
        let url = self.make_url("/v1/ns/service");
        let client = reqwest::Client::new();
        let resp = client.post(&url).form(args).send().await?;
        let status = resp.status();
        match status {
            StatusCode::OK => {
                let body: String = resp.text().await?;
                if body == "ok" {
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

    pub async fn update_service(&self, args: &UpdateService) -> Result<()> {
        let url = self.make_url("/v1/ns/instance");
        let client = reqwest::Client::new();
        let resp = client.put(&url).form(args).send().await?;
        let status = resp.status();
        match status {
            StatusCode::OK => {
                let body: String = resp.text().await?;
                if body == "ok" {
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

    pub async fn delete_service(&self, args: &DeleteService) -> Result<()> {
        let url = self.make_url("/v1/ns/instance");
        let client = reqwest::Client::new();
        let resp = client.delete(&url).form(args).send().await?;
        let status = resp.status();
        match status {
            StatusCode::OK => {
                let body: String = resp.text().await?;
                if body == "ok" {
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

    pub async fn query_service(&self, args: &QueryService) -> Result<String> {
        let url = self.make_url("/v1/ns/service");
        let client = reqwest::Client::new();
        let resp = client.get(&url).form(args).send().await?;
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

    pub async fn query_service_list(&self, args: &QueryServiceList) -> Result<String> {
        let url = self.make_url("/v1/ns/service/list");
        let client = reqwest::Client::new();
        let resp = client.get(&url).query(args).send().await?;
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

    pub async fn query_system_switch(&self) -> Result<String> {
        let url = self.make_url("/v1/ns/operator/switches");
        let client = reqwest::Client::new();
        let resp = client.get(&url).send().await?;
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

    pub async fn update_system_switch(&self, args: &UpdateSystemSwitch) -> Result<()> {
        let url = self.make_url("/v1/ns/operator/switches");
        let client = reqwest::Client::new();
        let resp = client.put(&url).form(args).send().await?;
        let status = resp.status();
        match status {
            StatusCode::OK => {
                let body: String = resp.text().await?;
                if body == "ok" {
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

    pub async fn query_system_metrics(&self) -> Result<String> {
        let url = self.make_url("/v1/ns/operator/metrics");
        let client = reqwest::Client::new();
        let resp = client.get(&url).send().await?;
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

    pub async fn query_service_list(&self, healthy: bool) -> Result<String> {
        let url = self.make_url("/v1/ns/operator/services");
        let client = reqwest::Client::new();
        let resp = client
            .get(&url)
            .query(&[("healthy", healthy)])
            .send()
            .await?;
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

    pub async fn get_leader(&self) -> Result<String> {
        let url = self.make_url("/v1/ns/raft/leader");
        let client = reqwest::Client::new();
        let resp = client.get(&url).send().await?;
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

    pub async fn update_instance_health_status(
        &self,
        args: &UpdateInstanceHealthStatus,
    ) -> Result<()> {
        let url = self.make_url("/v1/ns/health/instance");
        let client = reqwest::Client::new();
        let resp = client.put(&url).form(args).send().await?;
        let status = resp.status();
        match status {
            StatusCode::OK => {
                let body: String = resp.text().await?;
                if body == "ok" {
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

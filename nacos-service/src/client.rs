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

    pub async fn modify_instance(&self, args: &ModifyInstance) -> Result<()> {
        let url = self.make_url("/v1/ns/instance");
        let client = reqwest::Client::new();
        let resp = client.put(&url).form(args).send().await?;
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

    // TODO: FIXME String
    pub async fn query_instance(&self, args: &QueryInstance) -> Result<String> {
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

    pub async fn query_instance_detail(&self, args: &QueryInstanceDetail) -> Result<()> {
        Ok(())
    }

    pub async fn send_instance_beat(&self, args: &SendInstanceBeat) -> Result<()> {
        Ok(())
    }

    pub async fn create_service(&self, args: &CreateService) -> Result<()> {
        Ok(())
    }

    pub async fn update_service(&self, args: &UpdateService) -> Result<()> {
        Ok(())
    }

    pub async fn delete_service(&self, args: &DeleteService) -> Result<()> {
        Ok(())
    }

    pub async fn query_service(&self, args: &QueryService) -> Result<()> {
        Ok(())
    }
}

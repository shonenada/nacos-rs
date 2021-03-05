use anyhow::Result;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::structs::{RegisterInstance, UnregistryInstance};

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

    pub async fn registry_instance(&self, args: &RegisterInstance) -> Result<String> {
        let url = self.make_url("/v1/ns/instance");
        let client = reqwest::Client::new();
        let resp = client.post(&url).form(&args).send().await?;
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
                    "Failed to request {}, status coode is {}, body: {}l",
                    url,
                    status,
                    body
                ))
            }
        }
    }

    pub async fn unregistry_instance(&self, args: &UnregistryInstance) -> Result {
        let url = self.make_url("/v1/ns/instance");
        let client = reqwest::Client::new();
        let resp = client.delete(&url).form(&args).send().await?;
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

use anyhow::anyhow;
use anyhow::Result;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::NacosResponse;

#[derive(Clone, Debug)]
pub struct NacosConfig {
    scheme: String,
    host: String,
    port: u16,
    context_path: String,
}

#[derive(Clone, Debug)]
pub struct NacosHTTPClient {
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

    pub fn into_client(&self) -> NacosHTTPClient {
        NacosHTTPClient::new(self.clone())
    }
}

impl NacosHTTPClient {
    pub fn new(config: NacosConfig) -> Self {
        Self { config }
    }

    pub fn make_url(&self, sp: &str) -> String {
        self.config.make_url(sp)
    }

    pub async fn get_string<D>(&self, sp: &str, query: &D) -> Result<String>
    where
        D: Serialize,
    {
        let url = self.make_url(sp);
        let client = reqwest::Client::new();
        let resp = client.get(&url).query(&query).send().await?;
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

    pub async fn get_json<T>(&self, sp: &str) -> Result<NacosResponse<T>>
    where
        T: DeserializeOwned,
    {
        let url = self.make_url(sp);
        let resp = reqwest::get(&url).await?;
        let status = resp.status();
        match status {
            StatusCode::OK => {
                let body: NacosResponse<T> = resp.json().await?;
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

    pub async fn simple_post<D>(&self, sp: &str, params: &D) -> Result<()>
    where
        D: Serialize,
    {
        let url = self.make_url(sp);
        let client = reqwest::Client::new();
        let resp = client.post(&url).form(&params).send().await?;
        let status = resp.status();
        match status {
            StatusCode::OK => {
                let body: String = resp.text().await?;
                if body == "true" || body == "ok" {
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

    pub async fn simple_put<D>(&self, sp: &str, params: &D) -> Result<()>
    where
        D: Serialize,
    {
        let url = self.make_url(sp);
        let client = reqwest::Client::new();
        let resp = client.put(&url).form(&params).send().await?;
        let status = resp.status();
        match status {
            StatusCode::OK => {
                let body: String = resp.text().await?;
                if body == "true" || body == "ok" {
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

    pub async fn simple_delete<D>(&self, sp: &str, params: &D) -> Result<()>
    where
        D: Serialize,
    {
        let url = self.make_url(sp);
        let client = reqwest::Client::new();
        let resp = client.delete(&url).form(&params).send().await?;
        let status = resp.status();
        match status {
            StatusCode::OK => {
                let body: String = resp.text().await?;
                if body == "true" || body == "ok" {
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

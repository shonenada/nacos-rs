use anyhow::Result;
use hyper::{body, Client};
use serde::{Deserialize, Serialize};

use crate::structs::Namespace;

#[derive(Debug, Serialize, Deserialize)]
struct CommonResponse<T> {
    code: u16,
    message: Option<String>,
    data: Vec<T>,
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
        let client = Client::new();
        let base_url = self.make_uri();
        let url = format!("{}/v1/console/namespaces", base_url);
        let uri = url.parse()?;
        let resp = client.get(uri).await?;
        let body_bytes = body::to_bytes(resp.into_body()).await?;
        let body = String::from_utf8(body_bytes.to_vec()).expect("response was not valid utf-8");
        let response: CommonResponse<Namespace> = serde_json::from_str(&body)?;
        Ok(response.data)
    }
}

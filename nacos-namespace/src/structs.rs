use anyhow::Result;
use hyper::Client;

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

    pub async fn list_namespaces(&self) -> Result<Vec<String>> {
        let client = Client::new();
        let base_url = self.make_uri();
        let url = format!("{}/v1/console/namespaces", base_url);
        let uri = url.parse()?;
        let mut resp = client.get(uri).await?;
        println!("Response: {}", resp.status());
        Ok(Vec::new())
    }
}

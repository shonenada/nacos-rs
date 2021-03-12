use anyhow::Result;

use crate::http_client::NacosHTTPClient;
use crate::namespace::structs::Namespace;
use crate::response::NacosResponse;

#[derive(Clone, Debug)]
pub struct NacosNamespace {
    client: NacosHTTPClient,
}

impl NacosNamespace {
    pub fn new(client: &NacosHTTPClient) -> Self {
        Self {
            client: client.clone(),
        }
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
        let params = [
            ("namespace", namespace_id),
            ("namespaceShowName", name),
            ("namespaceDesc", description),
        ];
        self.client
            .simple_put("/v1/console/namespaces", &params)
            .await
    }

    pub async fn delete_namespace(&self, namespace_id: &str) -> Result<()> {
        let params = [("namespaceId", namespace_id)];
        self.client
            .simple_delete("/v1/console/namespaces", &params)
            .await
    }
}

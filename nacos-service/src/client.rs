use anyhow::Result;
use reqwest::StatusCode;
use std::collections::HashMap;

use nacos_utils::NacosClient;

use crate::structs::*;

#[derive(Clone, Debug)]
pub struct NacosService {
    client: NacosClient,
}

impl NacosService {
    pub fn new(client: &NacosClient) -> Self {
        Self {
            client: client.clone(),
        }
    }

    fn make_url(&self, sp: &str) -> String {
        self.client.make_url(sp)
    }

    pub async fn register_instance(&self, args: &RegisterInstance) -> Result<()> {
        self.client.simple_post("/v1/ns/instance", args).await
    }

    pub async fn deregister_instance(&self, args: &DeregistryInstance) -> Result<()> {
        self.client.simple_delete("/v1/ns/instance", args).await
    }

    pub async fn modify_instance(&self, args: &ModifyInstance) -> Result<()> {
        self.client.simple_put("/v1/ns/instance", args).await
    }

    // TODO: FIXME String
    pub async fn query_instances(&self, args: &QueryInstance) -> Result<String> {
        self.client.get_string("/v1/ns/instance/list", args).await
    }

    // TODO: FIXME string
    pub async fn query_instance_detail(&self, args: &QueryInstanceDetail) -> Result<String> {
        self.client.get_string("/v1/ns/instance", args).await
    }

    pub async fn send_instance_beat(&self, args: &SendInstanceBeat) -> Result<()> {
        self.client.simple_put("/v1/ns/instance/beat", args).await
    }

    pub async fn create_service(&self, args: &CreateService) -> Result<()> {
        self.client.simple_post("/v1/ns/service", args).await
    }

    pub async fn update_service(&self, args: &UpdateService) -> Result<()> {
        self.client.simple_put("/v1/ns/instance", args).await
    }

    pub async fn delete_service(&self, args: &DeleteService) -> Result<()> {
        self.client.simple_delete("/v1/ns/instance", args).await
    }

    pub async fn query_service(&self, args: &QueryService) -> Result<String> {
        self.client.get_string("/v1/ns/service", args).await
    }

    pub async fn query_service_list(&self, args: &QueryServiceList) -> Result<String> {
        self.client.get_string("/v1/ns/service/list", args).await
    }

    pub async fn query_system_switch(&self) -> Result<String> {
        let params = HashMap::<String, String>::new();
        self.client
            .get_string("/v1/ns/operator/switches", &params)
            .await
    }

    pub async fn update_system_switch(&self, args: &UpdateSystemSwitch) -> Result<()> {
        self.client
            .simple_put("/v1/ns/operator/switches", args)
            .await
    }

    pub async fn query_system_metrics(&self) -> Result<String> {
        let params = HashMap::<String, String>::new();
        self.client
            .get_string("/v1/ns/operator/metrics", &params)
            .await
    }

    pub async fn query_server_list(&self, healthy: bool) -> Result<String> {
        self.client
            .get_string("/v1/ns/operator/servers", &[("healthy", healthy)])
            .await
    }

    pub async fn get_leader(&self) -> Result<String> {
        self.client
            .get_string("/v1/ns/raft/leader", &HashMap::<String, String>::new())
            .await
    }

    pub async fn update_instance_health_status(
        &self,
        args: &UpdateInstanceHealthStatus,
    ) -> Result<()> {
        self.client.simple_put("/v1/ns/health/instance", args).await
    }
}

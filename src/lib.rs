mod config;
mod http_client;
mod namespace;
mod response;
mod service;

pub use config::NacosConfigClient;
pub use http_client::NacosConfig;
pub use namespace::NacosNamespace;
pub use service::NacosService;

pub use service::structs;

pub struct NacosClient {
    config: NacosConfig,
}

impl NacosClient {
    pub fn new(scheme: &str, host: &str, port: u16) -> Self {
        Self {
            config: NacosConfig::new(scheme, host, port, "/nacos"),
        }
    }

    pub fn new_from_config(config: &NacosConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }

    pub fn config_client(&self) -> NacosConfigClient {
        NacosConfigClient::new(&self.config.into_client())
    }

    pub fn namespace_client(&self) -> NacosNamespace {
        NacosNamespace::new(&self.config.into_client())
    }

    pub fn service_client(&self) -> NacosService {
        NacosService::new(&self.config.into_client())
    }
}

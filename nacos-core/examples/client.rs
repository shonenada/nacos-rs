use nacos_core::NacosClient;

pub fn main() -> Result<(), &'static str> {
    let client = NacosClient::new("http", "localhost", 8848);
    let _config_client = client.config_client();
    Ok(())
}

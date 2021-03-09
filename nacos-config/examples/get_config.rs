use nacos_config::NacosConfigClient;
use nacos_utils::NacosConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let data_id = "data_id";
    let group = "DEFAULT";
    let tenant = "";
    let config = NacosConfig::new("http", "localhost", 8848, "/nacos");
    let client = NacosConfigClient::new(&config.into_client());
    let result = client.get_config(data_id, group, tenant).await?;
    println!("result: {}", result);
    Ok(())
}

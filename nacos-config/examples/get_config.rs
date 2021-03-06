use nacos_config::NacosConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let data_id = "data_id";
    let group = "DEFAULT";
    let tenant = "";
    let client = NacosConfig::new("http", "localhost", 8848, "/nacos");
    let result = client.get_config(data_id, group, tenant).await?;
    println!("result: {}", result);
    Ok(())
}

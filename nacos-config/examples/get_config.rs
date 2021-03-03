use nacos_config::NacosConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = NacosConfig::new("http", "localhost", 8848, "/nacos");
    let result = client.get_config("", "", "").await?;
    println!("result: {}", result);
    Ok(())
}

use anyhow::Result;

use nacos_namespace::NacosNamespace;

#[tokio::main]
async fn main() -> Result<()> {
    let client = NacosNamespace::new("http", "localhost", 8848, "/nacos");
    client.list_namespaces().await?;
    Ok(())
}

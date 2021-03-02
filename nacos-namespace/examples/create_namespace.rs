use nacos_namespace::NacosNamespace;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = NacosNamespace::new("http", "localhost", 8848, "/nacos");
    let namespace_id = "namespace_id".to_string();
    let namespace_name = "name".to_string();
    let description = "Description".to_string();
    client
        .create_namespace(namespace_id, namespace_name, description)
        .await?;
    Ok(())
}

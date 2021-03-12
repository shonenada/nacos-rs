use nacos::NacosConfig;
use nacos::NacosNamespace;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = NacosConfig::new("http", "localhost", 8848, "/nacos");
    let client = NacosNamespace::new(&config.into_client());
    let namespace_id = "namespace_id";
    let namespace_name = "name";
    let description = "Description";
    client
        .create_namespace(namespace_id, namespace_name, description)
        .await?;
    println!("Created");

    let namespaces = client.list_namespaces().await?;
    for ns in namespaces.iter() {
        println!("{} {}", ns.namespace, ns.namespace_show_name);
    }
    Ok(())
}

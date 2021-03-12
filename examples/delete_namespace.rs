use nacos::NacosConfig;
use nacos::NacosNamespace;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = NacosConfig::new("http", "localhost", 8848, "/nacos");
    let client = NacosNamespace::new(&config.into_client());
    let namespace_id = "namespace_id";
    client.delete_namespace(namespace_id).await?;
    println!("Deleted");

    let namespaces = client.list_namespaces().await?;
    for ns in namespaces.iter() {
        println!("{} {}", ns.namespace, ns.namespace_show_name);
    }
    Ok(())
}

use nacos_namespace::NacosNamespace;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = NacosNamespace::new("http", "localhost", 8848, "/nacos");
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

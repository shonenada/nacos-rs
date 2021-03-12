use nacos_core::NacosClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = NacosClient::new("http", "localhost", 8848);
    let namespace_client = client.namespace_client();
    let namespaces = namespace_client.list_namespaces().await?;
    for ns in namespaces.iter() {
        println!("{} {}", ns.namespace, ns.namespace_show_name);
    }
    Ok(())
}

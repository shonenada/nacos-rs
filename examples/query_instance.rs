use nacos::service::{structs::*, NacosService};
use nacos::NacosConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let service_name = "service_name".to_string();
    let namespace_id = "namespace_id".to_string();

    let config = NacosConfig::new("http", "localhost", 8848, "/nacos");
    let client = NacosService::new(&config.into_client());

    let query_args = QueryInstanceBuilder::default()
        .service_name(service_name)
        .namespace_id(namespace_id)
        .group_name("".to_string())
        .clusters("".to_string())
        .healthy_only(false)
        .build()
        .unwrap();
    let result = client.query_instances(&query_args).await?;
    println!("result: {}", result);

    Ok(())
}

use nacos_service::{structs::*, NacosService};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let service_name = "service_name".to_string();
    let namespace_id = "namespace_id".to_string();

    let client = NacosService::new("http", "localhost", 8848, "/nacos");
    let args = RegisterInstanceBuilder::default()
        .ip("localhost".to_string())
        .port(1080 as u16)
        .service_name(service_name.clone())
        .namespace_id(Some(namespace_id.clone()))
        .weight(100.0)
        .enabled(true)
        .healthy(true)
        .metadata(None)
        .cluster_name(None)
        .group_name(None)
        .ephemeral(false)
        .build()
        .unwrap();
    let register_result = client.register_instance(&args).await;
    println!(
        "Register: {}, {:?}",
        register_result.is_ok(),
        register_result
    );

    let query_args = QueryInstanceBuilder::default()
        .service_name(service_name)
        .namespace_id(namespace_id)
        .group_name("".to_string())
        .clusters("".to_string())
        .healthy_only(false)
        .build()
        .unwrap();
    let result = client.query_instance(&query_args).await?;
    println!("result: {}", result);

    Ok(())
}

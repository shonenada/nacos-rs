use std::{thread, time};

use nacos_config::NacosConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let data_id = "data_id";
    let group = "DEFAULT";
    let tenant = "";
    let client = NacosConfig::new("http", "localhost", 8848, "/nacos");
    let created_result = client
        .publish_config(
            data_id,
            group,
            "key=value\nkey2=value2",
            tenant,
            "Properties",
        )
        .await;
    println!("Published config: {}", created_result.is_ok());

    thread::sleep(time::Duration::from_secs(1));

    let result = client.get_config(data_id, group, tenant).await?;
    println!("result:\n{}", result);

    client.delete_config(data_id, group, tenant).await?;
    println!("Deleted config");

    let deleted_result = client.get_config(data_id, group, tenant).await;
    println!("result.is_err(): {}", deleted_result.is_err());

    Ok(())
}

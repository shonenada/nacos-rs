use std::{thread, time};

use nacos::NacosConfig;
use nacos::NacosConfigClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let data_id = "data_id";
    let group = "DEFAULT";
    let tenant = "";

    let config = NacosConfig::new("http", "localhost", 8848, "/nacos");
    let client = NacosConfigClient::new(&config.into_client());

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
    Ok(())
}

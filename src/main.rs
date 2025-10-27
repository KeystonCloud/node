use uuid::Uuid;

pub mod config;
pub mod gateway;

use config::settings::Settings;

#[tokio::main]
async fn main() {
    let settings = Settings::new().expect("Failed to load configuration");
    let node_id = Uuid::new_v4().to_string();
    let gateway_addr = settings.gateway.address;

    println!("---- Node started ----");
    println!("ID: {}", node_id);
    println!("Gateway: {}", gateway_addr);
    println!("----------------------");

    gateway::registration::send(
        gateway_addr.clone(),
        node_id.to_string(),
        settings.node.ip,
        settings.node.port,
        settings.node.registration_interval,
    )
    .await;

    let gateway_addr_cloned = gateway_addr.clone();
    tokio::spawn(async move {
        gateway::heartbeat::send(
            gateway_addr_cloned,
            node_id.to_string(),
            settings.node.heartbeat_interval,
        )
        .await;
    });

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}

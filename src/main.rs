use axum::{Router, routing::get};
use std::net::SocketAddr;
use uuid::Uuid;

pub mod api;
pub mod config;
pub mod core;
pub mod gateway;

use config::settings::Settings;

#[tokio::main]
async fn main() {
    let settings = Settings::new().expect("Failed to load configuration");
    let node_id = Uuid::new_v4().to_string();

    let node_id_clone = node_id.clone();
    let gateway_addr = settings.gateway.address.clone();
    let node_host = settings.node.host.clone();
    let node_port = settings.node.port;
    let registration_interval = settings.node.registration_interval;
    tokio::spawn(async move {
        gateway::registration::send(
            gateway_addr.clone(),
            node_id_clone.to_string(),
            node_host.clone(),
            node_port,
            registration_interval,
        )
        .await;

        gateway::heartbeat::send(
            gateway_addr.clone(),
            node_id_clone.to_string(),
            node_host.clone(),
            node_port,
            settings.node.heartbeat_interval,
            registration_interval,
        )
        .await;
    });

    let api_app_router = api::app::create_router();
    let app = Router::new()
        .route("/", get(root_handler))
        .nest("/api", api_app_router);

    let addr: SocketAddr = format!("{}:{}", settings.node.host, settings.node.port)
        .parse()
        .expect("Invalid address format");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("---- Node started ----");
    println!("ID: {}", node_id);
    println!("API: {}", addr);
    println!("Gateway: {}", settings.gateway.address);
    println!("----------------------");

    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> &'static str {
    "Node online."
}

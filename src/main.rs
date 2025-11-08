use axum::{Router, routing::get};
use std::{net::SocketAddr, path::Path};
use tokio::fs;
use uuid::Uuid;

pub mod api;
pub mod config;
pub mod core;
pub mod gateway;

use config::settings::Settings;

use crate::config::node::NodeIdentity;

#[tokio::main]
async fn main() {
    let settings = Settings::new().expect("[SETTINGS] Failed to load configuration.");
    let node_id = Uuid::new_v4().to_string();

    let api_app_router = api::app::create_router(settings.clone());
    let app = Router::new()
        .route("/", get(root_handler))
        .nest("/api", api_app_router);
    let addr: SocketAddr = format!("{}:{}", settings.node.host, settings.node.port)
        .parse()
        .expect("[WEBSERVER] Invalid address format.");

    println!("---------------------------- Node config ----------------------------");
    println!("ID: {}", node_id);
    println!("API: {}", addr);
    println!("Satellite API: {}", settings.satellite.api_host);
    println!("Satellite PEER ID: {:?}", settings.satellite.peer_id);
    println!("Satellite PEER HOST: {:?}", settings.satellite.peer_host);
    println!("Satellite PEER PORT: {:?}", settings.satellite.peer_port);
    println!("---------------------------------------------------------------------");

    match get_or_register_identity(&settings).await {
        Ok(identity) => {
            tokio::spawn(async move {
                gateway::heartbeat::send(&settings, identity).await;
            });
        }
        Err(e) => {
            println!("[INIT] Node registration failed: {}", e);
        }
    }

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("[WEBSERVER] TCP listener failed to bind.");

    println!("[WEBSERVER] Server starting...");
    axum::serve(listener, app)
        .await
        .expect("[WEBSERVER] Server start failed.");
}

async fn root_handler() -> &'static str {
    "Node online."
}

async fn load_identity_from_file(settings: &Settings) -> Option<NodeIdentity> {
    let identity_path = format!("{}/identity.json", settings.node.data_path);

    if !Path::new(&identity_path).exists() {
        return None;
    }
    match fs::read_to_string(&identity_path).await {
        Ok(content) => serde_json::from_str::<NodeIdentity>(&content).ok(),
        Err(_) => None,
    }
}

async fn get_or_register_identity(settings: &Settings) -> Result<NodeIdentity, String> {
    if let Some(node) = load_identity_from_file(settings).await {
        println!("[INIT] Local identify found. ID: {}", node.id);
        Ok(node)
    } else {
        gateway::registration::send(settings).await
    }
}

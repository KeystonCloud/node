use reqwest::Client;
use serde::Serialize;
use std::time::Duration;
use tokio::time::sleep;

use crate::config::{node::NodeIdentity, settings::Settings};

#[derive(Serialize)]
struct HeartbeatPayload<'a> {
    id: &'a str,
}

pub async fn send(settings: &Settings, node_identity: NodeIdentity) {
    let client = Client::new();
    let heartbeat_url = format!("{}/api/node/heartbeat", settings.satellite.api_host);

    loop {
        sleep(Duration::from_secs(settings.node.heartbeat_interval)).await;
        println!(
            "[HEARTBEAT] Sending the heartbeat with id={}",
            node_identity.id
        );

        let heartbeat_payload = HeartbeatPayload {
            id: &node_identity.id,
        };

        match client
            .post(&heartbeat_url)
            .json(&heartbeat_payload)
            .send()
            .await
        {
            Ok(resp) => {
                if resp.status().is_success() {
                    println!("[HEARTBEAT] Received by the gateway.");
                } else if resp.status() == reqwest::StatusCode::NOT_FOUND {
                    println!(
                        "[HEARTBEAT] Error: The Gateway doesn't know us. Attempting to re-register..."
                    );
                    let _ = crate::gateway::registration::send(settings).await;
                } else {
                    println!("[HEARTBEAT] Sending failed: {}", resp.status());
                }
            }
            Err(e) => {
                println!(
                    "[HEARTBEAT] Fatal error: Unable to contact the Gateway. {}",
                    e
                );
            }
        }
    }
}

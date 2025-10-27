use chrono::Utc;
use reqwest::Client;
use serde::Serialize;
use std::time::Duration;
use tokio::time::sleep;

use crate::gateway::registration;

#[derive(Serialize)]
struct HeartbeatPayload<'a> {
    id: &'a str,
    date: i64,
}

pub async fn send(
    addr: String,
    id: String,
    ip: String,
    port: u16,
    interval: u64,
    registration_interval: u64,
) {
    let client = Client::new();
    let heartbeat_url = format!("{}/api/nodes/heartbeat", addr);

    loop {
        sleep(Duration::from_secs(interval)).await;

        println!("[HEARTBEAT] Sending the heartbeat...");

        let heartbeat_payload = HeartbeatPayload {
            id: &id,
            date: Utc::now().timestamp(),
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
                    crate::gateway::registration::send(
                        addr.clone(),
                        id.clone(),
                        ip.clone(),
                        port,
                        registration_interval,
                    )
                    .await;
                } else {
                    println!("[HEARTBEAT] Sending failed: {}", resp.status());
                }
            }
            Err(e) => {
                println!(
                    "[HEARTBEAT] Fatal error: Unable to contact the Gateway. ({})",
                    e
                );
            }
        }
    }
}

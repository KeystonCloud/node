use reqwest::Client;
use serde::Serialize;
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;

pub mod config;

use config::settings::Settings;

#[derive(Serialize)]
struct RegisterPayload<'a> {
    id: &'a str,
    ip: &'a str,
    port: u16,
}

#[derive(Serialize)]
struct HeartbeatPayload<'a> {
    id: &'a str,
}

async fn send_register(addr: &String, register_payload: &RegisterPayload<'_>) {
    let client = Client::new();
    let register_url = format!("{}/api/nodes/register", addr);

    match client
        .post(&register_url)
        .json(register_payload)
        .send()
        .await
    {
        Ok(resp) => {
            if resp.status().is_success() {
                println!("[REGISTER] Successful registration with the Gateway!");
            } else {
                println!("[REGISTER] Registering failed: {}", resp.status());
            }
        }
        Err(e) => {
            println!(
                "[REGISTER] Fatal error: Unable to contact the Gateway. ({})",
                e
            );
        }
    }
}

async fn send_heartbeat(addr: &String, heartbeat_payload: &HeartbeatPayload<'_>, interval: u64) {
    let client = Client::new();
    let heartbeat_url = format!("{}/api/nodes/heartbeat", addr);

    loop {
        sleep(Duration::from_secs(interval)).await;

        println!("[HEARTBEAT] Sending the heartbeat...");

        match client
            .post(&heartbeat_url)
            .json(heartbeat_payload)
            .send()
            .await
        {
            Ok(resp) => {
                if resp.status().is_success() {
                    println!("[HEARTBEAT] Received.");
                } else if resp.status() == reqwest::StatusCode::NOT_FOUND {
                    println!(
                        "[HEARTBEAT] Error: The Gateway doesn't know us. Attempting to re-register..."
                    );
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

#[tokio::main]
async fn main() {
    let settings = Settings::new().expect("Failed to load configuration");
    let node_id = Uuid::new_v4().to_string();

    let node_ip = &settings.node.ip;
    let node_port = settings.node.port;
    let gateway_addr = &settings.gateway.address;
    let heartbeat_interval = settings.node.heartbeat_interval;

    println!("---- Node started ----");
    println!("ID: {}", node_id);
    println!("Gateway: {}", gateway_addr);
    println!("----------------------");

    let register_payload = RegisterPayload {
        id: &node_id,
        ip: node_ip,
        port: node_port,
    };
    send_register(gateway_addr, &register_payload).await;

    let heartbeat_payload = HeartbeatPayload { id: &node_id };
    send_heartbeat(gateway_addr, &heartbeat_payload, heartbeat_interval).await;
}

use reqwest::Client;
use serde::Serialize;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Serialize)]
struct RegisterPayload<'a> {
    id: &'a str,
    ip: &'a str,
    port: u16,
}

pub async fn send(addr: String, id: String, ip: String, port: u16, interval: u64) {
    let client = Client::new();
    let register_url = format!("{}/api/node/register", addr);
    let register_payload = RegisterPayload {
        id: &id,
        ip: &ip,
        port: port,
    };

    match client
        .post(&register_url)
        .json(&register_payload)
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

            sleep(Duration::from_secs(interval)).await;
            Box::pin(send(addr, id, ip, port, interval)).await;
        }
    }
}

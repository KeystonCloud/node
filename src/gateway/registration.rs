use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::{fs, time::sleep};

use crate::config::{node::NodeIdentity, settings::Settings};

#[derive(Serialize)]
struct RegisterPayload<'a> {
    owner_id: &'a str,
    name: &'a str,
    port: u16,
}

#[derive(Deserialize)]
struct RegisterResponse {
    data: NodeIdentity,
    error: Option<String>,
}

pub async fn send(settings: &Settings) -> Result<NodeIdentity, String> {
    let client = Client::new();
    let register_url = format!("{}/api/node/register", settings.satellite.api_host);
    let register_payload = RegisterPayload {
        owner_id: &settings.node.owner_id,
        name: &settings.node.name,
        port: settings.node.port,
    };

    match client
        .post(&register_url)
        .json(&register_payload)
        .send()
        .await
    {
        Ok(resp) => {
            if !resp.status().is_success() {
                println!("[REGISTER] Registering failed: {}", resp.status());
                return Err(format!("Registering failed: {}", resp.status()));
            }

            match resp.json::<RegisterResponse>().await {
                Ok(resp_identity) => {
                    if let Some(error) = resp_identity.error {
                        println!("[REGISTER] Registering error: {}", error);
                        return Err(format!("Registering error: {}", error));
                    }

                    println!("[REGISTER] Successfully registered with Satellite.");

                    match serde_json::to_string_pretty(&resp_identity.data) {
                        Err(e) => {
                            println!(
                                "[REGISTER] Fatal error: Unable to serialize identity data. ({})",
                                e
                            );
                            return Err(format!("Unable to serialize identity data: {}", e));
                        }
                        Ok(identity) => {
                            let data_path = settings.node.data_path.clone();
                            let identity_path = format!("{}/identity.json", data_path);

                            if let Err(_) = fs::create_dir_all(data_path).await {
                                return Err("Unable to create data directory.".to_string());
                            }
                            if let Err(_) = fs::write(&identity_path, identity).await {
                                return Err("Unable to save identity to file.".to_string());
                            }

                            println!(
                                "[REGISTER] Successfully identity saved in {}",
                                identity_path
                            );
                            Ok(resp_identity.data)
                        }
                    }
                }
                Err(e) => {
                    println!(
                        "[REGISTER] Fatal error: Unable to parse Satellite response. ({})",
                        e
                    );
                    Err(format!("Unable to parse Gateway response: {}", e))
                }
            }
        }
        Err(e) => {
            println!(
                "[REGISTER] Fatal error: Unable to contact the Gateway. ({})",
                e
            );

            sleep(Duration::from_secs(settings.node.registration_interval)).await;
            Box::pin(send(settings)).await
        }
    }
}

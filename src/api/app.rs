use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::post};
use reqwest::Client;
use serde::Deserialize;

use crate::core::json::SimpleJsonResponse;

pub fn create_router() -> Router {
    Router::new().route("/deploy", post(deploy_app))
}

#[derive(Deserialize, Debug)]
pub struct AppDeployPayload {
    name: String,
    cid: String,
}

pub async fn deploy_app(Json(payload): Json<AppDeployPayload>) -> impl IntoResponse {
    println!("[APP] New app deployment received.");

    let client = Client::new();
    let ipfs_url = format!("http://localhost:5001/api/v0/pin/add?arg={}", payload.cid);

    match client.post(&ipfs_url).send().await {
        Ok(resp) => {
            if resp.status().is_success() {
                println!("[APP] App \"{}\" deployed!", payload.name);
                (
                    StatusCode::OK,
                    Json(SimpleJsonResponse {
                        message: format!("App \"{}\" deployed!", payload.name),
                    }),
                )
            } else {
                eprintln!(
                    "[APP] App \"{}\" deployment failed: {}",
                    payload.name,
                    resp.status()
                );
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(SimpleJsonResponse {
                        message: format!(
                            "App \"{}\" deployment failed: {}",
                            payload.name,
                            resp.status()
                        ),
                    }),
                )
            }
        }
        Err(e) => {
            eprintln!(
                "[APP] App \"{}\" deployment fatal error: {}",
                payload.name, e
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SimpleJsonResponse {
                    message: format!("App \"{}\" deployment fatal error: {}", payload.name, e),
                }),
            )
        }
    }
}

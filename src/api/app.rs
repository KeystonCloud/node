use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::post};
use serde::Deserialize;

use crate::core::json::SimpleJsonResponse;

pub fn create_router() -> Router {
    Router::new().route("/deploy", post(deploy_app))
}

#[derive(Deserialize, Debug)]
pub struct AppDeployPayload {
    name: String,
}

pub async fn deploy_app(Json(payload): Json<AppDeployPayload>) -> impl IntoResponse {
    println!("[APP] New app deployment received.");

    (
        StatusCode::OK,
        Json(SimpleJsonResponse {
            message: format!("Deploy request for app {} received", payload.name),
        }),
    )
}

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GatewayConfig {
    pub address: String,
}

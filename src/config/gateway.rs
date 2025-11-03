use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct GatewayConfig {
    pub address: String,
    pub peer_host: String,
    pub peer_port: u16,
    pub peer_id: String,
}

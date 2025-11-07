use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct SatelliteConfig {
    pub api_host: String,
    pub peer_host: String,
    pub peer_port: u16,
    pub peer_id: String,
}

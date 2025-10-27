use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NodeConfig {
    pub port: u16,
    pub ip: String,
    pub registration_interval: u64,
    pub heartbeat_interval: u64,
}

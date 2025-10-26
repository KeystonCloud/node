use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NodeConfig {
    pub port: u16,
    pub ip: String,
    pub heartbeat_interval: u64,
}

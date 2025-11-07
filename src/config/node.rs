use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct NodeConfig {
    pub owner_id: String,
    pub name: String,
    pub port: u16,
    pub host: String,
    pub registration_interval: u64,
    pub heartbeat_interval: u64,
    pub ipfs_host: String,
    pub data_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeIdentity {
    pub id: String,
}

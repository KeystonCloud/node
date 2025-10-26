use serde::Deserialize;

use crate::config::{gateway::GatewayConfig, node::NodeConfig};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub node: NodeConfig,
    pub gateway: GatewayConfig,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let config_builder = config::Config::builder()
            .add_source(config::File::with_name("config/default"))
            .add_source(config::Environment::with_prefix("NODE").separator("__"))
            .build()?;

        config_builder.try_deserialize()
    }
}

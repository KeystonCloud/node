use serde::Deserialize;

use crate::config::{node::NodeConfig, satellite::SatelliteConfig};

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub node: NodeConfig,
    pub satellite: SatelliteConfig,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let config_builder = config::Config::builder()
            .add_source(config::File::with_name("config/default"))
            .add_source(config::Environment::with_prefix("KC").separator("__"))
            .build()?;

        config_builder.try_deserialize()
    }
}

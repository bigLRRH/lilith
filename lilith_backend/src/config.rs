use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub database_url: String,
    pub server_addr: SocketAddr,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        dotenvy::dotenv().ok();

        let s = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        s.try_deserialize()
    }
}

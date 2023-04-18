use crate::conf::env_num;
use std::env;

/// Basic configuration for a server.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub addr: String,
    pub port: u16,
}

impl ServerConfig {
    /// Initialize the configuration with the env variables `HOST`
    /// (otherwise default_host) and `PORT` (otherwise use default_port).
    pub fn init(default_host: &str, default_port: u16) -> ServerConfig {
        let addr = env::var("HOST").unwrap_or(default_host.to_string());
        let port = env_num::<u16>("PORT", default_port);
        ServerConfig { addr, port }
    }
}

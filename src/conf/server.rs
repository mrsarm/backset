use crate::conf::env_num;
use std::env;

/// Basic configuration for an HTTP server.
#[derive(Debug, Clone)]
pub struct HttpServerConfig {
    pub addr: String,
    pub port: u16,
    pub uri: String,
}

impl HttpServerConfig {
    /// Initialize the configuration with the env variables `HOST`
    /// (otherwise default_host) and `PORT` (otherwise use default_port).
    pub fn init(default_host: &str, default_port: u16) -> HttpServerConfig {
        let addr = env::var("HOST").unwrap_or(default_host.to_string());
        let port = env_num::<u16>("PORT", default_port);
        let uri = env::var("APP_URI").unwrap_or("".to_string());
        HttpServerConfig { addr, port, uri }
    }
}

use crate::conf::env_num;
use std::env;

/// Basic configuration for an HTTP server.
#[derive(Debug, Clone)]
pub struct HttpServerConfig {
    /// Host address, may be set by the `HOST` environment variable,
    /// requests will be limited to the address passed. Setting
    /// it to "0" means requests can be received from anywhere.
    pub addr: String,
    /// Host port, may be set by the `PORT` environment variable
    pub port: u16,
    /// API URI (e.g. "/api"), may be set by the `APP_URI` environment variable
    pub uri: String,
    /// Final URL parsed: "http://{addr}:{port}/{uri}"
    pub url: String,
}

impl HttpServerConfig {
    /// Initialize the configuration with the env variables `HOST`
    /// (otherwise default_host) and `PORT` (otherwise use default_port),
    /// and the env variable `APP_URI` is used to se the `uri`, otherwise
    /// defaulted to empty string.
    pub fn init(default_host: &str, default_port: u16) -> HttpServerConfig {
        let addr = env::var("HOST").unwrap_or(default_host.to_string());
        let port = env_num::<u16>("PORT", default_port);
        let uri = env::var("APP_URI").unwrap_or("".to_string());
        let url = format!("http://{}{}{}/",
                          if addr == "0" { "localhost" } else { &addr },
                          if port == 80 { "".to_string() } else { format!(":{}", port) },
                          if uri.is_empty() { "".to_string() } else { format!("/{}", uri) });
        HttpServerConfig { addr, port, uri, url }
    }
}

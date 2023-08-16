use crate::conf::db::DbConfig;
use crate::conf::env::Environment;
use crate::conf::server::HttpServerConfig;
use log::{debug, log, Level};
use std::env;
use std::fmt::Debug;

/// `Config` is responsible of the configuration
/// of the server, reading the settings from environment
/// variables and .env file if exists, and configuring
/// the logging system.
///
/// It does not setup anything related with the data layer
/// other than the DB string connection, see the `app_state`
/// module for that.
/// The web layer configuration is configured by `app_server`.
#[derive(Debug, Clone)]
pub struct Config {
    // Add more conf here
    pub env: Environment,
    pub server: HttpServerConfig,
    pub db: DbConfig,
}

impl Config {
    /// Initialize config and the environment will be set with the `APP_ENV`
    /// environment variable.
    ///
    /// The port number is get from the `PORT` env variable, otherwise
    /// defaulted to `default_port`.
    pub fn init(default_port: u16) -> Config {
        Self::init_for(default_port, None)
    }

    /// Initialize config with the environment passed, if `None`, the environment
    /// will be set with the `APP_ENV` environment variable.
    ///
    /// The port number is get from the `PORT` env variable, otherwise
    /// defaulted to `default_port`.
    pub fn init_for(default_port: u16, environment: Option<Environment>) -> Config {
        debug!("⚙️  Configuring Backset ...");
        let env = environment.unwrap_or_else(Environment::from_app_env);
        let log_level = match env {
            Environment::Test => Level::Debug,
            _ => Level::Info,
        };
        log!(log_level, "⚙️  Environment set to {env}");
        let db = DbConfig::init_for(&env);
        let server = HttpServerConfig::init("127.0.0.1", default_port);
        Config { env, server, db }
    }
}

impl ToString for Config {
    /// This `to_string()` implementation prints out all the config
    /// values in `.env` format, using as key the environment variable
    /// used to set-up the config, even if the configuration was
    /// set in another way, e.g. using a default value.
    fn to_string(&self) -> String {
        format!(
r#"# The following items are the environment variables and its values from
# the OS, from an .env file, or the default value used by **Backset**.
#
# APP_URL --> {}
#
APP_ENV={}
APP_URI="{}"
HOST={}
PORT={}
DATABASE_URL="{}"
MIN_CONNECTIONS={}
MAX_CONNECTIONS={}
ACQUIRE_TIMEOUT_SEC={}
IDLE_TIMEOUT_SEC={}
TEST_BEFORE_ACQUIRE={}
RUST_LOG="{}""#,
            self.server.url,
            self.env,
            self.server.uri,
            self.server.addr,
            self.server.port,
            self.db.database_url,
            self.db.min_connections,
            self.db.max_connections,
            self.db.acquire_timeout.as_secs(),
            self.db.idle_timeout.as_secs(),
            self.db.test_before_acquire,
            env::var("RUST_LOG").unwrap_or("".to_string())
            )
    }
}

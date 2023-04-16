use crate::conf::db::DbConfig;
use crate::conf::env::Environment;
use crate::conf::env_num;
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
    pub port: u16,
    pub addr: String,
    pub db: DbConfig,
}

impl Config {
    pub fn init(default_port: u16) -> Config {
        Self::init_for(default_port, None)
    }

    pub fn init_for(default_port: u16, environment: Option<Environment>) -> Config {
        debug!("⚙️  Configuring Backset server ...");
        let env = environment.unwrap_or_else(Environment::from_app_env);
        let log_level = match env {
            Environment::Test => Level::Debug,
            _ => Level::Info,
        };
        log!(log_level, "⚙️  Environment set to {env}");
        let port = env_num::<u16>("PORT", default_port);
        let addr = env::var("HOST").unwrap_or("127.0.0.1".to_owned());
        let db = DbConfig::init_for(&env);
        Config { env, port, addr, db }
    }
}

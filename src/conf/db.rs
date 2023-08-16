use crate::conf::Environment;
use crate::conf::{env_bool, env_num};
use std::env;
use std::time::Duration;


/// Settings used to establish a connection with a database. All the values
/// can be initialized with [`DbConfig::init_for`] method, that uses
/// environment variables to setup most of them, otherwise some have a
/// default value.
#[derive(Debug, Clone)]
pub struct DbConfig {
    /// Database URL, initialized with the `DATABASE_URL` env
    pub database_url: String,
    /// Min connections created at start-up, value set with `MIN_CONNECTIONS` env,
    /// default 1
    pub min_connections: u32,
    /// Max connections allowed, value set with `MAX_CONNECTIONS` env,
    /// default 10
    pub max_connections: u32,
    /// Time allowed to acquire a connection, value set with `ACQUIRE_TIMEOUT_SEC` env,
    /// default 2 sec
    pub acquire_timeout: Duration,
    /// Time a connection can be idle, value set with `IDLE_TIMEOUT_SEC` env,
    /// default 2 sec
    pub idle_timeout: Duration,
    /// Whether to test before test the connection at start-up or not,
    /// value set with `TEST_BEFORE_ACQUIRE` env, default to false
    pub test_before_acquire: bool,
}

impl DbConfig {
    pub fn init_for(env: &Environment) -> Self {
        let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let database_url = if *env == Environment::Test && !url.ends_with("_test") {
            format!("{url}_test")
        } else {
            url
        };
        let min_connections = env_num::<u32>("MIN_CONNECTIONS", 1);
        let max_connections = env_num::<u32>("MAX_CONNECTIONS", 10);
        let acquire_timeout = Duration::from_secs(env_num::<u64>("ACQUIRE_TIMEOUT_SEC", 2));
        let idle_timeout = Duration::from_secs(env_num::<u64>("IDLE_TIMEOUT_SEC", 2));
        let test_before_acquire = env_bool("TEST_BEFORE_ACQUIRE", false);
        DbConfig {
            database_url,
            min_connections,
            max_connections,
            acquire_timeout,
            idle_timeout,
            test_before_acquire,
        }
    }
}

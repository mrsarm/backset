use crate::conf::Environment;
use crate::conf::{env_bool, env_num};
use std::env;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub database_url: String,
    pub min_connections: u32,
    pub max_connections: u32,
    pub acquire_timeout: Duration,
    pub idle_timeout: Duration,
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
        let max_connections = env_num::<u32>("MAX_CONNECTIONS", 10);
        let min_connections = env_num::<u32>("MIN_CONNECTIONS", 1);
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

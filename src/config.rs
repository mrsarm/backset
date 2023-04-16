use crate::BACKSET_PORT;
use log::{debug, log, Level};
use std::env;
use std::fmt::Debug;
use std::str::FromStr;
use std::time::Duration;
use strum_macros::{Display, EnumString};

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
    // Add more config here
    pub env: Environment,
    pub port: u16,
    pub addr: String,
    pub db: DbConfig,
}

/// The possible runtime environment for our application.
#[derive(Debug, Default, Display, PartialEq, EnumString, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum Environment {
    #[default]
    Local,
    Test,
    Stage,
    Production,
}

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub database_url: String,
    pub min_connections: u32,
    pub max_connections: u32,
    pub acquire_timeout: Duration,
    pub idle_timeout: Duration,
    pub test_before_acquire: bool,
}

impl Config {
    pub fn init() -> Config {
        Self::init_for(None)
    }

    pub fn init_for(environment: Option<Environment>) -> Config {
        debug!("⚙️ Configuring Backset server ...");
        let env = environment.unwrap_or_else(|| {
            let app_env = env::var("APP_ENV");
            match app_env {
                Err(_) => Environment::default(),
                Ok(e) => Environment::from_str(e.as_str())
                    .unwrap_or_else(|_| panic!("APP_ENV invalid value \"{e}\"")),
            }
        });
        let log_level = match env {
            Environment::Test => Level::Debug,
            _ => Level::Info,
        };
        log!(log_level, "⚙️ Environment set to {env}");
        let port = Self::_parse_num::<u16>("PORT", BACKSET_PORT);
        let addr = env::var("HOST").unwrap_or("127.0.0.1".to_owned());
        let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let database_url = if env == Environment::Test && !url.ends_with("_test") {
            format!("{url}_test")
        } else {
            url
        };
        let max_connections = Self::_parse_num::<u32>("MAX_CONNECTIONS", 10);
        let min_connections = Self::_parse_num::<u32>("MIN_CONNECTIONS", 1);
        let acquire_timeout =
            Duration::from_secs(Self::_parse_num::<u64>("ACQUIRE_TIMEOUT_SEC", 2));
        let idle_timeout = Duration::from_secs(Self::_parse_num::<u64>("IDLE_TIMEOUT_SEC", 2));
        let test_before_acquire = Self::_parse_bool("TEST_BEFORE_ACQUIRE", false);
        Config {
            env,
            port,
            addr,
            db: DbConfig {
                database_url,
                min_connections,
                max_connections,
                acquire_timeout,
                idle_timeout,
                test_before_acquire,
            },
        }
    }

    fn _parse_bool(env_name: &'static str, default_value: bool) -> bool {
        env::var(env_name)
            .map(|v| match v.as_str() {
                "0" => "false".to_owned(),
                "1" => "true".to_owned(),
                _ => v.to_lowercase(),
            })
            .map(|v| {
                v.parse::<bool>()
                    .unwrap_or_else(|_| panic!("{env_name} invalid boolean \"{v}\""))
            })
            .unwrap_or(default_value)
    }

    fn _parse_num<A: FromStr>(env_name: &'static str, default_value: A) -> A
    where
        <A as FromStr>::Err: Debug,
    {
        env::var(env_name)
            .map(|v| {
                v.parse::<A>()
                    .unwrap_or_else(|_| panic!("{env_name} invalid number \"{v}\""))
            })
            .unwrap_or(default_value)
    }
}

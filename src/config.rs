use std::env;
use std::fmt::Debug;
use std::str::FromStr;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Config {
    // Add more config here
    pub port: u16,
    pub addr: String,
    pub db: DbConfig,
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
        let port = Self::_parse_num::<u16>("PORT", 8558);
        let addr = env::var("HOST").unwrap_or("127.0.0.1".to_owned());
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let max_connections = Self::_parse_num::<u32>("MAX_CONNECTIONS", 10);
        let min_connections = Self::_parse_num::<u32>("MIN_CONNECTIONS", 1);
        let acquire_timeout = Duration::from_secs(Self::_parse_num::<u64>("ACQUIRE_TIMEOUT_SEC", 2));
        let idle_timeout = Duration::from_secs(Self::_parse_num::<u64>("IDLE_TIMEOUT_SEC", 2));
        let test_before_acquire = Self::_parse_bool("TEST_BEFORE_ACQUIRE", false);
        Config {
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
            .map(|v| v.parse::<bool>().expect(&(format!("{env_name} invalid boolean \"{v}\""))))
            .unwrap_or(default_value)
    }

    fn _parse_num<A: FromStr>(env_name: &'static str, default_value: A) -> A
        where <A as FromStr>::Err: Debug
    {
        env::var(env_name)
            .map(|v| v.parse::<A>().expect(&(format!("{env_name} invalid number \"{v}\""))))
            .unwrap_or(default_value)
    }
}

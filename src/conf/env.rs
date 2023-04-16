use std::env;
use std::fmt::Debug;
use std::str::FromStr;
use strum_macros::{Display, EnumString};

/// The possible runtime environments for our application.
#[derive(Debug, Default, Display, PartialEq, EnumString, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum Environment {
    #[default]
    Local,
    Test,
    Stage,
    Production,
}

impl Environment {
    pub fn from_app_env() -> Self {
        let app_env = env::var("APP_ENV");
        match app_env {
            Err(_) => Environment::default(),
            Ok(e) => Environment::from_str(e.as_str())
                .unwrap_or_else(|_| panic!("APP_ENV invalid value \"{e}\"")),
        }
    }
}

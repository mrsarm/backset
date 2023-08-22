use anyhow::{Context, Result};
use std::env;
use std::fmt::Debug;
use std::str::FromStr;
use strum_macros::{Display, EnumString};

/// Possible runtime environments for an application.
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
    /// Get the value from the environment variable
    /// `APP_ENV`.
    /// It raise an error if the string doesn't match a possible environment.
    pub fn from_app_env() -> Result<Self> {
        let app_env = env::var("APP_ENV");
        match app_env {
            Err(_) => Ok(Environment::default()),
            Ok(env) => Environment::from_str(env.as_str())
                .with_context(|| format!("APP_ENV invalid value \"{env}\"")),
        }
    }
}

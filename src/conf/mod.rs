//! The conf package allows to manage the basic configurations to
//! run a server with a database connection, configuring it
//! through normal constructions like `Config { ... }`, or through
//! pre-defined environment variables, like `APP_ENV` to define
//! the environment (local, test, prod..), or `DATABASE_URL`
//! that sets the DB string connection.

mod config;
mod env;

pub mod db;
pub mod server;
pub use crate::conf::config::Config;
pub use crate::conf::env::Environment;

use std::env::var;
use std::fmt::Debug;
use std::str::FromStr;

/// Read boolean environment variable, accepting "0" or "false" as false
/// values, and "1" or "true" values as true.
pub fn env_bool(env_name: &'static str, default_value: bool) -> Result<bool, String> {
    var(env_name)
        .map(|v| match v.as_str() {
            "0" => "false".to_owned(),
            "1" => "true".to_owned(),
            _ => v.to_lowercase(),
        })
        .map(|v| {
            v.parse::<bool>()
                .map_err(|_| format!("{env_name} invalid boolean \"{v}\""))
        })
        .unwrap_or(Ok(default_value))
}

/// Get a parsable value from an env value like a number,
/// otherwise return `default_value`.
pub fn env_parsable<A: FromStr>(env_name: &'static str, default_value: A) -> Result<A, String>
where
    <A as FromStr>::Err: Debug,
{
    var(env_name)
        .map(|v| {
            v.parse::<A>()
                .map_err(|_| format!("{env_name} invalid number \"{v}\""))
        })
        .unwrap_or(Ok(default_value))
}

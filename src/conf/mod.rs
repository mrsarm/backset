mod config;
mod env;

pub mod db;
pub use crate::conf::config::Config;
pub use crate::conf::env::Environment;

use std::env::var;
use std::fmt::Debug;
use std::str::FromStr;

pub fn env_bool(env_name: &'static str, default_value: bool) -> bool {
    var(env_name)
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

pub fn env_num<A: FromStr>(env_name: &'static str, default_value: A) -> A
where
    <A as FromStr>::Err: Debug,
{
    var(env_name)
        .map(|v| {
            v.parse::<A>()
                .unwrap_or_else(|_| panic!("{env_name} invalid number \"{v}\""))
        })
        .unwrap_or(default_value)
}

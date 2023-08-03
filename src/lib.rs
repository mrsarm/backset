#[macro_use]
extern crate lazy_static;

pub mod app_cmd;
pub mod app_server;
pub mod app_state;
pub mod conf;
pub mod core;
pub mod errors;
pub mod health;
pub mod page;
pub mod tenants;

// Has to match Cargo.toml > version
// TODO add in CI script to check both versions match
pub static BACKSET_VERSION: &str = "0.1.0";

/// Default HTTP port used by Backset
pub const BACKSET_PORT: u16 = 8558;

/// Default pagination size
pub static PAGE_SIZE: i64 = 50;

pub mod app_state;
pub mod app_server;
pub mod config;
pub mod errors;
pub mod health;
pub mod tenants;

// Has to match Cargo.toml > version
// TODO add in CI script to check both versions match
pub static BACKSET_VERSION: &str = "0.1.0";

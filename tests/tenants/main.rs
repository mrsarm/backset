use actix_web::web::Data;
use actix_contrib_rest::app_state::AppState;
use backset::BACKSET_PORT;
use log::{error, LevelFilter};
use server_env_config::Config;
use server_env_config::env::Environment;
use std::process::exit;
use std::sync::Once;

mod api_tests;

static INIT: Once = Once::new();

pub async fn initialize() -> Data<AppState> {
    INIT.call_once(|| {
        dotenv::dotenv().ok(); // read conf from .env file if available
        env_logger::builder()
            .filter(Some("backset::conf"), LevelFilter::Warn)
            .filter(Some("backset::app_state"), LevelFilter::Warn)
            .init();
    });
    let config = Config::init_for(BACKSET_PORT, Some(Environment::Test))
        .unwrap_or_else(|error| {
            error!("{error}");
            exit(1);
        });
    let data = AppState::init(config.clone()).await
        .unwrap_or_else(|error| {
            error!("{error}");
            exit(1);
        });
    Data::new(data)
}

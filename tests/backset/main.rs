use actix_contrib_rest::app_state::AppState;
use actix_http::Request;
use actix_web::http::header::{Accept, ContentType};
use actix_web::test::TestRequest;
use actix_web::web::Data;
use backset::BACKSET_PORT;
use log::{error, LevelFilter};
use serde::Serialize;
use server_env_config::env::Environment;
use server_env_config::Config;
use std::process::exit;
use std::sync::Once;

mod health_api_tests;
mod tenants_api_tests;

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

pub fn get(path: &str) -> Request {
    TestRequest::get()
        .uri(path)
        .insert_header(Accept::json())
        .to_request()
}

pub fn post(path: &str, data: impl Serialize) -> Request {
    TestRequest::post()
        .uri(path)
        .insert_header(Accept::json())
        .insert_header(ContentType::json())
        .set_json(data)
        .to_request()
}

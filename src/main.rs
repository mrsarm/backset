mod app_state;
mod config;
mod errors;
mod health;
mod tenants;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use actix_web_validator::JsonConfig;
use app_state::AppState;
use config::Config;
use dotenv::dotenv;
use errors::json_error_handler;
use log::info;
use tenants::api as tenants_api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    info!("🚀 Starting Backset server ...");

    let config = Config::init();
    let state = AppState::new(config).await;

    HttpServer::new(move || {
        let cors = Cors::default()
            //.allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::ACCEPT]);
        App::new()
            .app_data(web::Data::new(state.clone()))
            .app_data(JsonConfig::default().error_handler(json_error_handler))
            .configure(health::config)
            .configure(tenants_api::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

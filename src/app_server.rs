use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web_validator::JsonConfig;
use crate::app_state::AppState;

use crate::config::Config;
use crate::errors::json_error_handler;
use crate::health;
use crate::tenants::api as tenants_api;

pub struct AppServer {
    pub server: Server,
    pub addr: String,
    pub port: u16,
}

impl AppServer {
    pub async fn build(config: Config) -> Result<Self, anyhow::Error> {
        let port: u16 = config.port;
        let addr = config.addr.clone();
        let state = AppState::new(config.clone()).await;

        let server = HttpServer::new(move || {
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
        .bind((addr.clone(), port))?
        .run();

        Ok(AppServer { server, addr, port })
    }
}
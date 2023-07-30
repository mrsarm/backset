use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::web::{Data, ServiceConfig};
use actix_web::{App, HttpServer};
use actix_web_validator::{JsonConfig, QueryConfig};
use log::info;

use crate::app_state::AppState;
use crate::conf::Config;
use crate::errors::json_error_handler;
use crate::health;
use crate::tenants::api as tenants_api;

pub struct AppServer {
    pub server: Server,
    pub addr: String,
    pub port: u16,
}

impl AppServer {
    pub async fn build(config: Config, app_version: &str) -> Result<Self, anyhow::Error> {
        let addr = config.server.addr.clone();
        let port: u16 = config.server.port;
        let uri = config.server.uri.clone();
        info!(
            "🚀 Starting Backset server v{} at http://{}:{}/{} ...",
            app_version, addr, port, uri
        );
        let state = AppState::new(config).await;

        let server = HttpServer::new(move || {
            let config_app = Self::config_app(Data::new(state.clone()));
            App::new()
                .service(web::scope(uri.as_str()).configure(config_app))
                .wrap(Logger::default())
        })
        .bind((addr.clone(), port))?
        .run();

        Ok(AppServer { server, addr, port })
    }

    pub fn config_app(data: Data<AppState>) -> Box<dyn Fn(&mut ServiceConfig)> {
        Box::new(move |conf: &mut ServiceConfig| {
            conf.app_data(data.clone())
                .app_data(JsonConfig::default().error_handler(json_error_handler))
                .app_data(QueryConfig::default().error_handler(json_error_handler))
                .configure(health::config)
                .configure(tenants_api::config);
        })
    }
}

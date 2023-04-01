use actix_web::dev::Server;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::{Data, ServiceConfig};
use actix_web_validator::JsonConfig;
use log::info;

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
        info!("🚀 Starting Backset server ...");
        let port: u16 = config.port;
        let addr = config.addr.clone();
        let state = AppState::new(config.clone()).await;

        let server = HttpServer::new(move || {
            App::new()
                .configure(Self::config_app(Data::new(state.clone())))
                .wrap(Logger::default())
        })
        .bind((addr.clone(), port))?
        .run();

        Ok(AppServer { server, addr, port })
    }

    pub fn config_app(data: Data<AppState>) -> Box<dyn Fn(&mut ServiceConfig)> {
        Box::new(move |conf: &mut ServiceConfig| {
            conf
                .app_data(data.clone())
                .app_data(JsonConfig::default().error_handler(json_error_handler))
                .configure(health::config)
                .configure(tenants_api::config);
        })
    }
}
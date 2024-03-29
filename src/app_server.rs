//! Build and run the HTTP server.

use actix_contrib_logger::middleware::Logger;
use actix_contrib_rest::app_state::AppState;
use actix_contrib_rest::response::json_error_handler;
use actix_web::dev::Server;
use actix_web::middleware::Compress;
use actix_web::web;
use actix_web::web::{Data, ServiceConfig};
use actix_web::{App, HttpServer};
use actix_web_validator::{JsonConfig, QueryConfig};
use awc::http::StatusCode;
use log::{error, info, Level};
use server_env_config::server::HttpServerConfig;
use server_env_config::Config;
use std::process::exit;

use crate::routes;

/// Build and run the HTTP server.
pub struct AppServer {
    pub server: Server,
    pub addr: String,
    pub port: u16,
}

impl AppServer {
    /// Build the web server. After calling the method, to pause the thread
    /// of the app and listen for connections:
    /// ```example
    /// let app = AppServer::build(config, "0.1.0").await?;
    /// app.server.await?;
    /// ```
    pub async fn build(config: Config, app_version: &str) -> Result<Self, anyhow::Error> {
        let HttpServerConfig { addr, port, uri, url } = config.server.clone();
        info!("🚀 Starting Backset server v{} at {} ...", app_version, url);
        let state = AppState::init(config).await.unwrap_or_else(|error| {
            error!("{error}");
            exit(1);
        });

        let server = HttpServer::new(move || {
            let config_app = Self::config_app(Data::new(state.clone()));
            let logger = Logger::default()
                .custom_level(|status| {
                    if status.is_server_error() {
                        Level::Error
                    } else if status == StatusCode::NOT_FOUND {
                        Level::Warn
                    } else {
                        Level::Info
                    }
                });
            App::new()
                .service(web::scope(uri.as_str()).configure(config_app))
                .wrap(Compress::default())
                .wrap(logger)
        })
        .bind((addr.clone(), port))?
        .run();

        Ok(AppServer { server, addr, port })
    }

    /// Setup the app, receiving the state that will be passed
    /// to all endpoints handlers methods that are configured.
    /// This method is called by [`AppServer::build()`], but
    /// can be also called in integrations tests initialization code.
    pub fn config_app(data: Data<AppState>) -> Box<dyn Fn(&mut ServiceConfig)> {
        Box::new(move |conf: &mut ServiceConfig| {
            conf.app_data(data.clone())
                .app_data(JsonConfig::default().error_handler(json_error_handler))
                .app_data(QueryConfig::default().error_handler(json_error_handler))
                .configure(routes::config);
        })
    }
}

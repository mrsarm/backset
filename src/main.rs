mod config;
mod health;
mod tenants;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use config::Config;
use dotenv::dotenv;
use log::{error, info};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use tenants::{api as tenants_api};

pub struct AppState {
    db: Pool<Postgres>,
    env: Config,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = Config::init();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            info!("Connection to the database successful");
            pool
        }
        Err(err) => {
            error!("Failed to connect to the database: {:?}", err);
            std::process::exit(2);
        }
    };

    info!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            //.allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::ACCEPT]);
        App::new()
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
                env: config.clone(),
            }))
            .configure(health::config)
            .configure(tenants_api::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

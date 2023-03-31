mod app_state;
mod app_server;
mod config;
mod errors;
mod health;
mod tenants;

use app_state::AppState;
use app_server::AppServer;
use config::Config;
use dotenv::dotenv;
use log::info;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();

    info!("⚙️ Configuring Backset server ...");
    let config = Config::init();
    info!("🚀 Starting Backset server ...");
    let app = AppServer::build(config).await?;
    app.server.await?;
    Ok(())
}

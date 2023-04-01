use backset::app_server::AppServer;
use backset::config::Config;
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

use backset::app_server::AppServer;
use backset::config::Config;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok(); // read config from .env file if available
    env_logger::init();
    let config = Config::init();
    let app = AppServer::build(config).await?;
    app.server.await?;
    Ok(())
}

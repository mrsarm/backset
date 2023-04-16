use backset::app_server::AppServer;
use backset::conf::Config;
use backset::{BACKSET_PORT, BACKSET_VERSION};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok(); // read conf from .env file if available
    env_logger::init();
    let config = Config::init(BACKSET_PORT);
    let app = AppServer::build(config, BACKSET_VERSION).await?;
    app.server.await?;
    Ok(())
}

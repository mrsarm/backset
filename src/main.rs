use backset::app_server::AppServer;
use backset::config::Config;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::init();
    let app = AppServer::build(config).await?;
    app.server.await?;
    Ok(())
}

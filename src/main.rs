//! Backset REST Service store elements, sets, and relations between the two.

use backset::app_cmd::{AppCmd, Args, Commands};
use backset::app_server::AppServer;
use backset::conf::Config;
use backset::{BACKSET_PORT, BACKSET_VERSION};
use std::process::exit;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::init();
    let config = Config::init(BACKSET_PORT);
    match &args.command {
        Commands::Run => {
            // HTTP REST Server
            let app = AppServer::build(config, BACKSET_VERSION).await?;
            app.server.await?;
        }
        _ => {
            // Command-line tool
            let app = AppCmd::build(config).await;
            app.run(&args.command).await?;
            exit(0);
        }
    }
    Ok(())
}

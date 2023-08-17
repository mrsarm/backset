//! Backset REST Service stores elements, sets, and relationships between them.

use backset::app_args::{Args, Commands};
use backset::app_cmd::AppCmd;
use backset::app_server::AppServer;
use backset::conf::Config;
use backset::{BACKSET_PORT, BACKSET_VERSION};
use std::process::exit;
use log::error;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::init();
    let config = Config::init(BACKSET_PORT).unwrap_or_else(|error| {
        error!("{error}");
        exit(1);
    });
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

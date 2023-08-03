//! Backset REST Service store elements, sets, and relations between the two.

use backset::app_cmd::AppCmd;
use backset::app_server::AppServer;
use backset::conf::Config;
use backset::page::QuerySearch;
use backset::{BACKSET_PORT, BACKSET_VERSION};
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use env_logger::Target;
use log::LevelFilter;
use std::io::Write;
use std::process::exit;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::init();
    let config = Config::init(BACKSET_PORT);
    match args.command {
        Commands::List { object } => {
            let app = AppCmd::build(config).await;
            match object {
                Objects::Tenants { query, lines } => {
                    app.list_tenants(QuerySearch {
                        q: query,
                        offset: 0,
                        page_size: lines,
                        sort: Some("id".to_string()),
                    })
                    .await?;
                }
            }
            exit(0);
        }
        Commands::Run => {
            let app = AppServer::build(config, BACKSET_VERSION).await?;
            app.server.await?;
        }
    }
    Ok(())
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

impl Args {
    /// Initialize dotenv, and output parser and logging systems
    pub fn init() -> Self {
        dotenv().ok(); // read conf from .env file if available
        let args = Args::parse();
        match &args.command {
            // Commands use normal stdout, command-like style for output
            Commands::List { object: _ } => {
                env_logger::builder()
                    .target(Target::Stdout)
                    .filter(Some("backset::conf"), LevelFilter::Warn)
                    .filter(Some("backset::app_state"), LevelFilter::Warn)
                    .format(|buf, record| writeln!(buf, "{}", record.args()))
                    .init();
            }
            // The server defaults output to stderr, with more info than commands
            Commands::Run => env_logger::init(),
        }
        args
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Run the Backset HTTP Server
    Run,
    /// List objects
    List {
        #[command(subcommand)]
        object: Objects,
    },
    //TODO more coming soon...
}

#[derive(Subcommand, strum_macros::Display)]
enum Objects {
    /// List tenants
    Tenants {
        /// Text to filter tenants
        #[arg(short = 'q', long)]
        query: Option<String>,

        /// Max number of results to display
        #[arg(short = 'n', long, default_value_t = 1000, value_name = "MAX")]
        lines: i64,
    },
}

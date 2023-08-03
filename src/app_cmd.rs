use crate::app_state::AppState;
use crate::conf::Config;
use crate::core::Result;
use crate::page::QuerySearch;
use crate::tenants::model::Tenant;
use clap::{Parser, Subcommand};
use env_logger::Target;
use log::{error, info, LevelFilter};
use std::io::Write;
use std::process::exit;

/// App class to command line instructions, instead of the HTTP server
pub struct AppCmd {
    pub state: AppState,
}

impl AppCmd {
    pub async fn build(config: Config) -> Self {
        let state = AppState::new(config).await;
        AppCmd { state }
    }

    pub async fn run(&self, cmd: &Commands) -> Result<()> {
        if let Commands::List { object: Objects::Tenants { query, lines } } = cmd {
            // List tenants
            self.list_tenants(query, *lines).await?;
        } else {
            // It should not get to this point
            error!("Unexpected command !");
            exit(1);
        };
        Ok(())
    }

    async fn list_tenants(&self, query: &Option<String>, lines: i64) -> Result<()> {
        let mut tx = self.state.get_tx().await?;
        let tenants = Tenant::find(&mut tx, &QuerySearch {
            q: query.clone(),
            offset: 0,
            page_size: lines,
            sort: Some("id".to_string()),
        })
        .await?;
        self.state.commit_tx(tx).await?;
        for tenant in tenants.iter() {
            info!("{}: {}", tenant.id, tenant.name);
        }
        Ok(())
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

impl Args {
    /// Initialize dotenv, and output parser and logging systems
    pub fn init() -> Self {
        dotenv::dotenv().ok(); // read conf from .env file if available
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
pub enum Commands {
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
pub enum Objects {
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

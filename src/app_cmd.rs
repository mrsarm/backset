use crate::app_state::AppState;
use crate::conf::Config;
use crate::core::Result;
use crate::errors::AppError;
use crate::page::QuerySearch;
use crate::stream::read_body;
use crate::tenants::model::Tenant;
use actix_web::http::header::Accept;
use awc::Client;
use clap::{Parser, Subcommand};
use env_logger::Target;
use log::{error, info, Level, LevelFilter};
use serde::Deserialize;
use std::env::var;
use std::io::Write;
use std::process::exit;
use std::str::FromStr;

/// App class to command line instructions, instead of the HTTP server
pub struct AppCmd {
    pub state: AppState,
}

#[derive(Deserialize)]
pub struct HealthPayload {
    #[serde(default = "String::new")]
    pub service: String,
    pub version: Option<String>,
}

impl AppCmd {
    pub async fn build(config: Config) -> Self {
        let state = AppState::new(config).await;
        AppCmd { state }
    }

    pub async fn run(&self, cmd: &Commands) -> Result<()> {
        if let Commands::List {
            object: Objects::Tenants { query, lines },
        } = cmd
        {
            // List tenants
            self.list_tenants(query, *lines).await?;
        } else if let Commands::Health = cmd {
            self.healthcheck().await?;
        } else {
            // It should not get to this point
            error!("Unexpected command {} !", cmd);
            exit(1);
        };
        Ok(())
    }

    async fn healthcheck(&self) -> Result<()> {
        let client = Client::default();
        let health_url = format!("{}{}", &self.state.config.server.url, "health");
        info!("Checking backset at {} ...", health_url);
        let mut res = client
            .get(&health_url)
            .insert_header(Accept::json())
            .send()
            .await
            .unwrap_or_else(|e| {
                error!("{}", e);
                exit(7);
            });

        if res.status().is_success() {
            let val = res
                .json::<HealthPayload>()
                .await
                .map_err(|e| AppError::Unexpected(e.into()))?;
            match val.service.as_str() {
                // Check it's backset and not another service listening in the same URL
                "backset" => {
                    info!("{}", res.status());
                    info!("Backset version: {}", val.version.as_deref().unwrap_or(""));
                }
                _ => {
                    error!("Unexpected response, looks like it's not backset.");
                }
            }
        } else {
            error!(
                "{}\nResponse:\n{}",
                res.status(),
                read_body(res.body()).await?
            );
            exit(2);
        }
        Ok(())
    }

    async fn list_tenants(&self, query: &Option<String>, lines: i64) -> Result<()> {
        let mut tx = self.state.get_tx().await?;
        let tenants = Tenant::find(
            &mut tx,
            &QuerySearch {
                q: query.clone(),
                offset: 0,
                page_size: lines,
                sort: Some("id".to_string()),
            },
        )
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
        let log_level = var("LOG_LEVEL").unwrap_or("INFO".to_string());
        let level_filter = LevelFilter::from_str(log_level.as_str()).unwrap_or(LevelFilter::Info);
        match &args.command {
            // Commands use normal stdout, command-like style for output
            Commands::Health | Commands::List { object: _ } => env_logger::builder()
                .target(Target::Stdout)
                .filter_level(level_filter)
                .filter(Some("backset::conf"), LevelFilter::Warn)
                .filter(Some("backset::conf"), LevelFilter::Warn)
                .filter(Some("backset::app_state"), LevelFilter::Warn)
                .format(|buf, record| {
                    let level = record.level();
                    match &level {
                        Level::Debug | Level::Info => writeln!(buf, "{}", record.args()),
                        _ => writeln!(buf, "{}: {}", level, record.args()),
                    }
                })
                .init(),
            // The server defaults output to stdout, with more info than commands
            Commands::Run => env_logger::builder()
                .target(Target::Stdout)
                .filter_level(level_filter)
                .init(),
        }
        args
    }
}

#[derive(Subcommand, strum_macros::Display)]
pub enum Commands {
    /// Run the Backset HTTP Server
    Run,
    /// Check the HTTP Server health (whether it's running)
    Health,
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

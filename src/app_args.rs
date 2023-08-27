use clap::{Parser, Subcommand};
use env_logger::Target;
use log::{Level, LevelFilter};
use std::env::var;
use std::io::Write;
use std::str::FromStr;

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
            // The server defaults output to stdout, with more info than commands
            Commands::Run => env_logger::builder()
                .target(Target::Stdout)
                .filter_level(level_filter)
                .init(),
            // Commands use normal stdout, command-like style for output
            _ => env_logger::builder()
                .target(Target::Stdout)
                .filter_level(level_filter)
                .filter(Some("server_env_config"), LevelFilter::Warn)
                .filter(Some("actix_contrib_rest::app_state"), LevelFilter::Warn)
                .format(|buf, record| {
                    let level = record.level();
                    match &level {
                        Level::Debug | Level::Info => writeln!(buf, "{}", record.args()),
                        _ => writeln!(buf, "{}: {}", level, record.args()),
                    }
                })
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
        object: ListObjects,
    },
    Create {
        #[command(subcommand)]
        object: CreateObjects,
    },
}

#[derive(Subcommand, strum_macros::Display)]
pub enum ListObjects {
    /// List tenants
    Tenants {
        /// Text to filter tenants
        #[arg(short = 'q', long)]
        query: Option<String>,

        /// Max number of results to display
        #[arg(short = 'n', long, default_value_t = 1000, value_name = "MAX")]
        lines: i64,
    },
    /// List all ENVIRONMENT_VARIABLE=current_value used by the server
    Envs,
}

#[derive(Subcommand, strum_macros::Display)]
pub enum CreateObjects {
    /// Create tenant
    Tenant {
        /// The id of the new tenant
        #[clap(value_name = "ID")]
        id: String,

        /// The name of the new tenant
        #[arg(short = 'n', long)]
        name: String,
    },
}

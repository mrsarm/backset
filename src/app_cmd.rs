use crate::app_args::{Commands, CreateObjects, ListObjects};
use crate::tenants::model::{Tenant, TenantPayload};
use actix_contrib_rest::app_state::AppState;
use actix_contrib_rest::query::QuerySearch;
use actix_contrib_rest::result::AppError;
use actix_contrib_rest::result::Result;
use actix_contrib_rest::stream::read_body;
use actix_web::http::header::Accept;
use awc::Client;
use log::{error, info};
use serde::Deserialize;
use server_env_config::Config;
use std::env;
use std::process::exit;
use sqlx::Connection;
use validator::Validate;

/// App state class for command line instructions, instead of the HTTP server.
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
    pub async fn build(config: Config) -> core::result::Result<Self, String> {
        let state = AppState::new(config);
        Ok(AppCmd { state })
    }

    pub async fn run(&self, cmd: &Commands) -> Result<()> {
        match cmd {
            Commands::Health => {
                self.healthcheck().await?;
            }
            Commands::List { object: ListObjects::Envs } => {
                self.list_envs();
            }
            Commands::List { object: ListObjects::Tenants { query, lines } } => {
                self.list_tenants(query, *lines).await?;
            }
            Commands::Create { object: CreateObjects::Tenant { id, name } } => {
                self.create_tenant(id, name).await?;
            }
            Commands::Run => {
                // It should not get to this point
                error!("Unexpected run command");
                exit(1);
            }
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
                exit(1);
            });

        if res.status().is_success() {
            let val = res
                .json::<HealthPayload>()
                .await
                .map_err(|e| AppError::Unexpected(e.into()))?;
            match val.service.as_str() {
                // Check it is backset and not another service listening in the same URL
                "backset" => {
                    info!("{}", res.status());
                    info!("Backset version: {}", val.version.as_deref().unwrap_or(""));
                }
                _ => {
                    error!("Unexpected response, looks like it's not backset.");
                    exit(1);
                }
            }
        } else {
            error!(
                "{}\nResponse:\n{}",
                res.status(),
                read_body(res.body()).await?
            );
            exit(1);
        }
        Ok(())
    }

    async fn list_tenants(&self, query: &Option<String>, lines: i64) -> Result<()> {
        let mut conn = self.state.get_conn().await?;
        let mut tx = Connection::begin(&mut conn).await.map_err(AppError::DB)?;
        let tenants = Tenant::find(
            &mut tx,
            &QuerySearch {
                q: query.clone(),
                offset: 0,
                page_size: lines,
                sort: Some("id".to_string()),
                include_total: Some(false),
            },
        )
        .await?;
        self.state.commit_tx(tx).await?;
        for tenant in tenants.iter() {
            info!("{}: {}", tenant.id, tenant.name);
        }
        Ok(())
    }

    async fn create_tenant(&self, id: &str, name: &str) -> Result<()> {
        let tenant = TenantPayload {
            id: id.to_string(),
            name: name.to_string(),
        };
        tenant.validate().map_err(|e| AppError::Validation(e.to_string()))?;
        let mut conn = self.state.get_conn().await?;
        let mut tx = Connection::begin(&mut conn).await.map_err(AppError::DB)?;
        Tenant::insert(&mut tx, tenant).await?;
        self.state.commit_tx(tx).await?;
        info!("Tenant \"{}\" created.", id);
        Ok(())
    }

    fn list_envs(&self) {
        info!(
r"# The following items are the environment variables and its values from
# the OS, from an .env file, or the default value used by **Backset**.
#");
        info!("{}", self.state.config.to_string());
        info!("LOG_LEVEL={}", env::var("LOG_LEVEL").unwrap_or("INFO".to_string()));
        info!("RUST_LOG=\"{}\"", env::var("RUST_LOG").unwrap_or("".to_string()));
    }
}

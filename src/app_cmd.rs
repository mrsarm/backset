use crate::app_args::{Commands, Objects};
use crate::app_state::AppState;
use crate::conf::Config;
use crate::core::Result;
use crate::errors::AppError;
use crate::page::QuerySearch;
use crate::stream::read_body;
use crate::tenants::model::Tenant;
use actix_web::http::header::Accept;
use awc::Client;
use log::{error, info};
use serde::Deserialize;
use std::process::exit;

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
    pub async fn build(config: Config) -> Self {
        let state = AppState::new(config).await;
        AppCmd { state }
    }

    pub async fn run(&self, cmd: &Commands) -> Result<()> {
        match cmd {
            Commands::Health => {
                self.healthcheck().await?;
            }
            Commands::List { object: Objects::Envs } => {
                self.list_envs();
            }
            Commands::List { object: Objects::Tenants { query, lines } } => {
                self.list_tenants(query, *lines).await?;
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
        let mut tx = self.state.get_tx().await?;
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

    fn list_envs(&self) {
        info!("{}", self.state.config.to_string());
    }
}

use crate::config::Config;
use crate::errors::BacksetError;
use log::{error, info};
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Postgres, Transaction};

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub env: Config,
}

impl AppState {
    pub async fn new(config: Config) -> AppState {
        let pool = match PgPoolOptions::new()
            .max_connections(config.max_connections)
            .connect(&config.database_url)
            .await
        {
            Ok(pool) => {
                info!("Connection to the database successful");
                pool
            }
            Err(err) => {
                error!("Failed to connect to the database: {:?}", err);
                std::process::exit(2);
            }
        };
        AppState {
            pool,
            env: config.clone(),
        }
    }

    pub async fn get_tx(&self) -> Result<Transaction<'static, Postgres>, BacksetError> {
        Ok(self.pool.begin().await.map_err(BacksetError::PGError)?)
    }
}

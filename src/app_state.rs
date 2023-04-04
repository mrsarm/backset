use crate::config::Config;
use crate::errors::BacksetError;
use log::{debug, error};
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Postgres, Transaction};


///! Struct that holds the app configurations
///! and the connection pool to the database.
///! Each API method should receive an AppState
///! argument as following:
///!
///! ```
///! #[post("...")]
///! async fn method_name(
///!     app: web::Data<AppState>,
///!     ...
///! ) -> Result<HttpResponse, ...> {
///! ```
#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub env: Config,
}

impl AppState {
    /// Receive the configuration and initialize
    /// the app state, like the pool connection to the DB.
    /// This method normally is used once at startup time
    /// by the web framework (Actix).
    pub async fn new(config: Config) -> AppState {
        let pool = match Self::_get_pool(&config) {
            Ok(pool) => {
                // The connection is lazy, so not sure whether the connection will work
                debug!("Connection to the database looks good");
                pool
            }
            Err(err) => {
                // Errors like wrongly parsed URLs arrive here
                error!("Failed to connect to the database: {:?}", err);
                std::process::exit(2);
            }
        };
        AppState {
            pool,
            env: config,
        }
    }

    /// Get a Transaction object to be used to transact with the DB.
    /// Once used #commit_tx() should be used to release the TX.
    pub async fn get_tx(&self) -> Result<Transaction<'static, Postgres>, BacksetError> {
        self.pool.begin().await.map_err(BacksetError::DB)
    }

    /// Commit the transaction passed.
    pub async fn commit_tx(
        &self, tx:
        Transaction<'static, Postgres>
    ) -> Result<(), BacksetError> {
        tx.commit().await.map_err(BacksetError::DB)?;
        Ok(())
    }

    /// Rollback the transaction passed.
    // TODO: should receive the error that caused the TX to be
    //       canceled, and "re-through" after rolling back
    #[allow(dead_code)]
    pub async fn rollback_tx(
        &self,
        tx: Transaction<'static, Postgres>
    ) -> Result<(), BacksetError> {
        tx.rollback().await.map_err(BacksetError::DB)?;
        Ok(())
    }

    fn _get_pool(config: &Config) -> Result<PgPool, BacksetError> {
        PgPoolOptions::new()
            .max_connections(config.db.max_connections)
            .min_connections(config.db.min_connections)
            .acquire_timeout(config.db.acquire_timeout)
            .idle_timeout(config.db.idle_timeout)
            .test_before_acquire(config.db.test_before_acquire)
            .connect_lazy(&config.db.database_url)
            .map_err(BacksetError::DB)
    }
}

use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use validator::Validate;
use crate::errors::BacksetError;

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Tenant {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize, Validate)]
pub struct TenantPayload {
    #[validate(length(min = 3))]
    pub name: String,
}

impl Tenant {
    pub async fn insert(
        tx: &mut Transaction<'_, Postgres>,
        tenant_form: TenantPayload,
    ) -> Result<Tenant, BacksetError> {
        let tenant = sqlx::query_as!(
                Tenant,
                "INSERT INTO tenants (name) VALUES ($1) RETURNING *",
                tenant_form.name)
            .fetch_one(tx)
            .await
            .map_err(BacksetError::DB)?;
        Ok(tenant)
    }

    pub async fn get(
        tx: &mut Transaction<'_, Postgres>,
        id: i64,
    ) -> Result<Option<Tenant>, BacksetError> {
        let tenant = sqlx::query_as!(
                Tenant,
                "SELECT id, name FROM tenants WHERE id = $1", id
            )
            .fetch_optional(tx)
            .await
            .map_err(BacksetError::DB)?;
        Ok(tenant)
    }
}

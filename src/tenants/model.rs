use crate::errors::BacksetError;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use validator::Validate;

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Tenant {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize, Validate)]
pub struct TenantPayload {
    #[validate(length(min = 3, max = 50))]
    pub name: String,
}

impl Tenant {
    pub async fn insert(
        tx: &mut Transaction<'_, Postgres>,
        tenant_form: TenantPayload,
    ) -> Result<Tenant, BacksetError> {
        let res = sqlx::query!(
                "SELECT EXISTS(SELECT id FROM tenants WHERE name = $1)",
                tenant_form.name)
            .fetch_one(&mut *tx)
            .await
            .map_err(BacksetError::DB)?;
        if res.exists.unwrap_or(false) {
            return Err(BacksetError::Validation("Tenant already exists".to_owned()));
        }
        let tenant = sqlx::query_as!(
                Tenant,
                "INSERT INTO tenants (name) VALUES ($1) RETURNING *",
                tenant_form.name)
            .fetch_one(&mut *tx)
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
            .fetch_optional(&mut *tx)
            .await
            .map_err(BacksetError::DB)?;
        Ok(tenant)
    }

    pub async fn delete(
        tx: &mut Transaction<'_, Postgres>,
        id: i64,
    ) -> Result<u64, BacksetError> {
        let result = sqlx::query!("DELETE FROM tenants WHERE id = $1", id)
            .execute(&mut *tx)
            .await
            .map_err(BacksetError::DB)?;

        Ok(result.rows_affected())
    }
}

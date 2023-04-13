use crate::core::{Result, Tx};
use crate::errors::AppError;
use crate::page::QuerySearch;
use serde::{Deserialize, Serialize};
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
    pub async fn insert(tx: &mut Tx<'_>, tenant_form: TenantPayload) -> Result<Tenant> {
        let res = sqlx::query!(
                "SELECT EXISTS(SELECT id FROM tenants WHERE name = $1)",
                tenant_form.name)
            .fetch_one(&mut *tx)
            .await
            .map_err(AppError::DB)?;
        if res.exists.unwrap_or(false) {
            return Err(AppError::StaticValidation("Tenant already exists"));
        }
        let tenant = sqlx::query_as!(
                Tenant,
                "INSERT INTO tenants (name) VALUES ($1) RETURNING *",
                tenant_form.name)
            .fetch_one(&mut *tx)
            .await
            .map_err(AppError::DB)?;
        Ok(tenant)
    }

    pub async fn get(tx: &mut Tx<'_>, id: i64) -> Result<Option<Tenant>> {
        let tenant = sqlx::query_as!(
                Tenant,
                "SELECT id, name FROM tenants WHERE id = $1", id
            )
            .fetch_optional(&mut *tx)
            .await
            .map_err(AppError::DB)?;
        Ok(tenant)
    }

    pub async fn count(tx: &mut Tx<'_>) -> Result<i64> {
        let count = sqlx::query!("SELECT COUNT(*) AS count FROM tenants")
            .fetch_one(&mut *tx)
            .await
            .map_err(AppError::DB)?;
        Ok(count.count.unwrap_or(0))
    }

    pub async fn find(tx: &mut Tx<'_>, query: &QuerySearch) -> Result<Vec<Tenant>> {
        let tenants = sqlx::query_as!(
                Tenant,
                "SELECT * FROM tenants LIMIT $1 OFFSET $2",
                query.page_size,
                query.offset,
            )
            .fetch_all(&mut *tx)
            .await
            .map_err(AppError::DB)?;
        Ok(tenants)
    }

    pub async fn delete(tx: &mut Tx<'_>, id: i64) -> Result<u64> {
        let result = sqlx::query!("DELETE FROM tenants WHERE id = $1", id)
            .execute(&mut *tx)
            .await
            .map_err(AppError::DB)?;

        Ok(result.rows_affected())
    }
}

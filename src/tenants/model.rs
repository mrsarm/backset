use sqlx::{Postgres, Transaction};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Tenant {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize)]
pub struct TenantForm {
    pub name: String,
}


impl Tenant {
    pub async fn insert(
        tx: &mut Transaction<'_, Postgres>,
        tenant_form: TenantForm,
    ) -> Result<Tenant, sqlx::Error> {
        let tenant = sqlx::query_as!(
            Tenant,
            "INSERT INTO tenants (name) VALUES ($1) RETURNING *",
            tenant_form.name
        )
        .fetch_one(tx)
        .await?;    // TODO transform here to BacksetError::PGError
        Ok(tenant)
    }

    pub async fn get(
        tx: &mut Transaction<'_, Postgres>,
        id: i64
    ) -> Result<Option<Tenant>, sqlx::Error> {
        let tenant = sqlx::query_as!(
            Tenant,
            "SELECT id, name FROM tenants WHERE id = $1", id
        )
        .fetch_optional(tx)
        .await?;    // TODO transform here to BacksetError::PGError
        Ok(tenant)
    }
}
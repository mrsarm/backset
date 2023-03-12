use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Tenant {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TenantForm {
    pub name: String,
}


impl Tenant {
    pub async fn insert(
        pool: &Pool<Postgres>,
        tenant_form: TenantForm,
    ) -> Result<Tenant, sqlx::Error> {
        let tenant = sqlx::query_as!(
            Tenant,
            "INSERT INTO tenants (name) VALUES ($1) RETURNING *",
            tenant_form.name
        )
        .fetch_one(pool)
        .await?;
        Ok(tenant)
    }
}
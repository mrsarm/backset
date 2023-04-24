use crate::core::{Result, Tx};
use crate::errors::AppError;
use crate::page::QuerySearch;
use serde::{Deserialize, Serialize};
use validator::Validate;

lazy_static! {
    static ref SORT_ALLOWED: Vec<&'static str> = vec!["name"];
}

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

    pub async fn count(tx: &mut Tx<'_>, q: Option<&str>) -> Result<i64> {
        let sql;
        let query = match q {
            None => {
                sql = format!("SELECT COUNT(*) FROM tenants");
                sqlx::query_as(sql.as_str())
            }
            Some(q) => {
                let name_like = format!("%{q}%");
                sql = format!(r#"
                SELECT COUNT(*)
                  FROM tenants
                  WHERE name ILIKE $1
                "#);
                sqlx::query_as(sql.as_str()).bind(name_like)
            }
        };
        let count: (i64,) = query.fetch_one(&mut *tx)
            .await
            .map_err(AppError::DB)?;
        Ok(count.0)
    }

    pub async fn find(tx: &mut Tx<'_>, query: &QuerySearch) -> Result<Vec<Tenant>> {
        let order = query.sort_as_order_by_args(&SORT_ALLOWED, "name");
        let sql;
        let query = match query.q.as_deref() {
            None => {
                sql = format!("SELECT * FROM tenants ORDER BY {order} LIMIT $1 OFFSET $2");
                sqlx::query_as(sql.as_str())
                    .bind(query.page_size)
                    .bind(query.offset)
            }
            Some(q) => {
                let name_like = format!("%{q}%");
                sql = format!(r#"
                SELECT *
                  FROM tenants
                  WHERE name ILIKE $1
                  ORDER BY {order} LIMIT $2 OFFSET $3
                "#);
                sqlx::query_as(sql.as_str())
                    .bind(name_like)
                    .bind(query.page_size)
                    .bind(query.offset)
            }
        };
        let tenants = query.fetch_all(&mut *tx)
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

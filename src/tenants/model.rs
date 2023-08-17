use crate::core::{Result, Tx};
use crate::errors::AppError;
use crate::query::QuerySearch;
use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use validator::{Validate, ValidationError};

lazy_static! {
    static ref ID_VALID: Regex = Regex::new(r"^[a-z][a-z0-9\-]+$").unwrap();
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Tenant {
    pub id: String,
    pub name: String,
    pub created_at: NaiveDateTime,
}

fn validate_forbidden_list(tenant_id: &str) -> core::result::Result<(), ValidationError> {
    if tenant_id == "tenants" || tenant_id == "health" {
        // id cannot collide with endpoint paths
        return Err(ValidationError {
            code: Cow::from("forbidden_id"),
            message: Some(Cow::from("Forbidden tenant id.")),
            params: HashMap::new(),
        });
    }
    Ok(())
}

#[derive(Deserialize, Validate)]
pub struct TenantPayload {
    #[validate(length(min = 3, max = 40))]
    #[validate(custom = "validate_forbidden_list")]
    #[validate(regex(
        path = "ID_VALID",
        code = "invalid_id",
        message = "Tenant id can only contains letters in lower case, numbers or the \"-\" symbol"
    ))]
    pub id: String,
    #[validate(length(min = 3, max = 80))]
    pub name: String,
}

impl Tenant {
    pub async fn insert(tx: &mut Tx<'_>, tenant_form: TenantPayload) -> Result<Tenant> {
        let res = sqlx::query!(
                "SELECT EXISTS(SELECT id FROM tenants WHERE id = $1)",
                tenant_form.id)
            .fetch_one(&mut **tx)
            .await
            .map_err(AppError::DB)?;
        if res.exists.unwrap_or(false) {
            return Err(AppError::Validation(
                format!("Tenant with id \"{}\" already exists.", tenant_form.id))
            );
        }
        let res = sqlx::query!(
                "SELECT EXISTS(SELECT id FROM tenants WHERE name = $1)",
                tenant_form.name)
            .fetch_one(&mut **tx)
            .await
            .map_err(AppError::DB)?;
        if res.exists.unwrap_or(false) {
            return Err(AppError::Validation(
                format!("Tenant with name \"{}\" already exists.", tenant_form.name))
            );
        }
        let tenant = sqlx::query_as!(
                Tenant,
                "INSERT INTO tenants (id, name, created_at) VALUES ($1, $2, NOW()) RETURNING *",
                tenant_form.id,
                tenant_form.name)
            .fetch_one(&mut **tx)
            .await
            .map_err(AppError::DB)?;
        Ok(tenant)
    }

    pub async fn get(tx: &mut Tx<'_>, id: &str) -> Result<Option<Tenant>> {
        let tenant = sqlx::query_as!(
                Tenant,
                "SELECT id, name, created_at FROM tenants WHERE id = $1", id
            )
            .fetch_optional(&mut **tx)
            .await
            .map_err(AppError::DB)?;
        Ok(tenant)
    }

    pub async fn count(tx: &mut Tx<'_>, q: Option<&str>) -> Result<i64> {
        let query = match q {
            None => sqlx::query_as("SELECT COUNT(*) FROM tenants"),
            Some(q) => sqlx::query_as(
                r#"
                SELECT COUNT(*)
                  FROM tenants
                  WHERE id ILIKE $1 OR name ILIKE $1
                "#)
                .bind(format!("%{q}%")),
        };
        let count: (i64,) = query.fetch_one(&mut **tx)
            .await
            .map_err(AppError::DB)?;
        Ok(count.0)
    }

    pub async fn find(tx: &mut Tx<'_>, query: &QuerySearch) -> Result<Vec<Tenant>> {
        let order = query.sort_as_order_by_args(&["id", "name", "created_at"], "id");
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
                sql = format!(
                    r#"
                SELECT *
                  FROM tenants
                  WHERE id ILIKE $1 OR name ILIKE $1
                  ORDER BY {order} LIMIT $2 OFFSET $3
                    "#
                );
                sqlx::query_as(sql.as_str())
                    .bind(name_like)
                    .bind(query.page_size)
                    .bind(query.offset)
            }
        };
        let tenants = query.fetch_all(&mut **tx)
            .await
            .map_err(AppError::DB)?;
        Ok(tenants)
    }

    pub async fn delete(tx: &mut Tx<'_>, id: &str) -> Result<u64> {
        let result = sqlx::query!("DELETE FROM tenants WHERE id = $1", id)
            .execute(&mut **tx)
            .await
            .map_err(AppError::DB)?;

        Ok(result.rows_affected())
    }
}

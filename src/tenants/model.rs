use actix_contrib_rest::db::Tx;
use actix_contrib_rest::query::QuerySearch;
use actix_contrib_rest::result::{AppError, Result};
use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgQueryResult;
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
        message = "tenant id can only contains letters in lower case, numbers or the \"-\" symbol"
    ))]
    pub id: String,
    #[validate(length(min = 3, max = 80))]
    pub name: String,
}

#[derive(Deserialize, Validate)]
pub struct TenantPayloadEdition {
    #[validate(length(min = 3, max = 80))]
    pub name: String,
}

impl Tenant {
    pub async fn insert(tx: &mut Tx<'_>, tenant_form: TenantPayload) -> Result<Tenant> {
        let exists = Self::exists(&mut *tx, tenant_form.id.as_str()).await?;
        if exists {
            return Err(AppError::ResourceAlreadyExists {
                resource: "tenant",
                attribute: "id",
                value: tenant_form.id
            });
        }
        let id = Self::get_id_by_name(&mut *tx, tenant_form.name.as_str()).await?;
        if id.is_some() {
            return Err(AppError::ResourceAlreadyExists {
                resource: "tenant",
                attribute: "name",
                value: tenant_form.name,
            });
        }
        let tenant = sqlx::query_as::<_, Tenant>(
                "INSERT INTO tenants (id, name, created_at) VALUES ($1, $2, NOW()) RETURNING *",
            )
            .bind(tenant_form.id.as_str())
            .bind(tenant_form.name.as_str())
            .fetch_one(&mut **tx)
            .await
            .map_err(AppError::DB)?;
        Ok(tenant)
    }

    pub async fn save(
        tx: &mut Tx<'_>,
        tid: &str,
        tenant_form: TenantPayloadEdition
    ) -> Result<Tenant> {
        let name = tenant_form.name.as_str();
        let res: Option<(String,)> = sqlx::query_as(
            "SELECT id FROM tenants WHERE id <> $1 AND name = $2")
            .bind(tid)
            .bind(name)
            .fetch_optional(&mut **tx)
            .await
            .map_err(AppError::DB)?;
        if let Some((duplicated_id,)) = res {
            return Err(AppError::Validation(
                Some("tenant_name_exists"),
                format!("name \"{name}\" already taken by tenant with id \"{duplicated_id}\"")
            ));
        }
        let element = sqlx::query_as::<_, Tenant>(
            "INSERT INTO tenants (id, name, created_at) \
            VALUES ($1, $2, NOW()) \
            ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name
            RETURNING *",
        )
            .bind(tid)
            .bind(name)
            .fetch_one(&mut **tx)
            .await
            .map_err(AppError::DB)?;
        Ok(element)
    }

    pub async fn exists(tx: &mut Tx<'_>, tid: &str) -> Result<bool> {
        let res: (bool,) = sqlx::query_as(
                "SELECT EXISTS(SELECT id FROM tenants WHERE id = $1)")
            .bind(tid)
            .fetch_one(&mut **tx)
            .await
            .map_err(AppError::DB)?;
        Ok(res.0)
    }

    pub async fn exists_or_fail(tx: &mut Tx<'_>, tid: &str) -> Result<()> {
        let tenant_exists = Tenant::exists(tx, tid).await?;
        if !tenant_exists {
            return Err(AppError::ResourceNotFound {
                resource: "tenant",
                attribute: "id",
                value: tid.to_string(),
            });
        }
        Ok(())
    }

    pub async fn get_id_by_name(tx: &mut Tx<'_>, name: &str) -> Result<Option<String>> {
        let res: Option<(String,)> = sqlx::query_as("SELECT id FROM tenants WHERE name = $1")
            .bind(name)
            .fetch_optional(&mut **tx)
            .await
            .map_err(AppError::DB)?;
        Ok(res.map(|r| r.0))
    }

    pub async fn get(tx: &mut Tx<'_>, tid: &str) -> Result<Option<Tenant>> {
        let tenant: Option<Tenant> = sqlx::query_as(
                "SELECT id, name, created_at FROM tenants WHERE id = $1")
            .bind(tid)
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

    pub async fn has_elements(tx: &mut Tx<'_>, tid: &str) -> Result<bool> {
        let res: (bool,) = sqlx::query_as(
                "SELECT EXISTS(SELECT id FROM elements WHERE tid = $1)"
            )
            .bind(tid)
            .fetch_one(&mut **tx)
            .await
            .map_err(AppError::DB)?;
        Ok(res.0)
    }

    pub async fn delete(tx: &mut Tx<'_>, tid: &str, force: bool) -> Result<u64> {
        let has_to_delete_elements = if force {
            true
        } else {
            let has_el = Self::has_elements(&mut *tx, tid).await?;
            if has_el {
                return Err(AppError::StaticValidation(
                    "cannot delete tenant with elements",
                ));
            }
            false
        };
        let mut rows_affected: u64 = 0;
        if has_to_delete_elements {
            let res: PgQueryResult = sqlx::query("DELETE FROM elements WHERE tid = $1")
                .bind(tid)
                .execute(&mut **tx)
                .await
                .map_err(AppError::DB)?;
            rows_affected += res.rows_affected();
        }
        let res: PgQueryResult = sqlx::query("DELETE FROM tenants WHERE id = $1")
            .bind(tid)
            .execute(&mut **tx)
            .await
            .map_err(AppError::DB)?;
        rows_affected += res.rows_affected();

        Ok(rows_affected)
    }
}

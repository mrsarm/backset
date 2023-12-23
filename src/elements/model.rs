use actix_contrib_rest::db::Tx;
use actix_contrib_rest::query::QuerySearch;
use actix_contrib_rest::result::{AppError, Result};
use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use rand::random;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sqlx::postgres::PgQueryResult;
use sqlx::types::Json;
use validator::Validate;

use crate::tenants::model::Tenant;

lazy_static! {
    // Base64 URL characters (except =) and some others like \~@-.:+
    static ref ID_VALID: Regex = Regex::new(r"^(?i)[a-z0-9_~@\\/][a-z0-9_\\~@\-\.\:+]*$").unwrap();
}

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Element {
    pub id: String,
    #[serde(skip_serializing)]
    pub tid: String,
    #[serde(flatten)]
    pub data: Json<Map<String, Value>>,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Validate)]
pub struct ElementPayload {
    #[validate(length(min = 1, max = 256))]
    #[validate(regex(
        path = "ID_VALID",
        code = "invalid_id",
        message = "id can only contains letters, numbers or the symbols \\_~@-.:+, \
        and must starts with a letter or number, or the symbols \\_~@"
    ))]
    pub id: Option<String>,
    #[serde(flatten)]
    pub data: Json<Map<String, Value>>,
}

impl Element {
    pub async fn insert(tx: &mut Tx<'_>, tid: &str, el_form: ElementPayload) -> Result<Element> {
        Tenant::exists_or_fail(tx, tid).await?;
        if el_form.data.as_ref().contains_key("created_at") {
            return Err(AppError::StaticValidation(
                "cannot provide reserved attribute \"created_at\""));
        }
        let id = match el_form.id {
            None => random::<u64>().to_string(),
            Some(_id) => {
                let exists = Self::exists(tx, tid, _id.as_str()).await?;
                if exists {
                    return Err(AppError::ResourceAlreadyExists {
                        resource: "element",
                        attribute: "id",
                        value: _id
                    });
                }
                _id
            }
        };
        let element = sqlx::query_as::<_, Element>(
            "INSERT INTO elements (tid, id, data, created_at) \
            VALUES ($1, $2, $3, NOW()) RETURNING *",
            )
            .bind(tid)
            .bind(id.as_str())
            .bind(el_form.data)
            .fetch_one(&mut **tx)
            .await
            .map_err(AppError::DB)?;
        Ok(element)
    }

    pub async fn exists(tx: &mut Tx<'_>, tid: &str, id: &str) -> Result<bool> {
        let res: (bool,) = sqlx::query_as(
            "SELECT EXISTS(SELECT id FROM elements WHERE tid = $1 AND id = $2)")
            .bind(tid)
            .bind(id)
            .fetch_one(&mut **tx)
            .await
            .map_err(AppError::DB)?;
        Ok(res.0)
    }

    pub async fn get(tx: &mut Tx<'_>, tid: &str, id: &str) -> Result<Option<Element>> {
        Tenant::exists_or_fail(tx, tid).await?;
        let element: Option<Element> = sqlx::query_as(
            "SELECT * FROM elements WHERE tid = $1 AND id = $2")
            .bind(tid)
            .bind(id)
            .fetch_optional(&mut **tx)
            .await
            .map_err(AppError::DB)?;
        Ok(element)
    }

    pub async fn delete(tx: &mut Tx<'_>, tid: &str, id: &str) -> Result<u64> {
        Tenant::exists_or_fail(tx, tid).await?;
        let res: PgQueryResult = sqlx::query(
                "DELETE FROM elements WHERE tid = $1 AND id = $2")
            .bind(tid)
            .bind(id)
            .execute(&mut **tx)
            .await
            .map_err(AppError::DB)?;

        Ok(res.rows_affected())
    }

    pub async fn count(tx: &mut Tx<'_>, tid: &str) -> Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM elements WHERE tid = $1"
        )
            .bind(tid)
            .fetch_one(&mut **tx)
            .await
            .map_err(AppError::DB)?;
        Ok(count.0)
    }

    pub async fn find(tx: &mut Tx<'_>, tid: &str, query: &QuerySearch) -> Result<Vec<Element>> {
        let query = sqlx::query_as(
                r#"
            SELECT *
            FROM elements
            WHERE tid = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
                "#
            )
            .bind(tid)
            .bind(query.page_size)
            .bind(query.offset);
        let elements: Vec<Element> = query.fetch_all(&mut **tx)
            .await
            .map_err(AppError::DB)?;
        Ok(elements)
    }
}

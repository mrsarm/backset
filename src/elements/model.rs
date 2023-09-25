use actix_contrib_rest::db::Tx;
use actix_contrib_rest::result::{AppError, Result};
use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use rand::random;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sqlx::types::Json;
use validator::Validate;

use crate::tenants::model::Tenant;

lazy_static! {
    static ref ID_VALID: Regex = Regex::new(r"^(?i)[a-z0-9][a-z0-9\-]+[a-z0-9]$").unwrap();
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
    #[validate(length(min = 2, max = 40))]
    #[validate(regex(
        path = "ID_VALID",
        code = "invalid_id",
        message = "id can only contains letters, numbers or the \"-\" symbol, \
        and must starts and ends with a letter or number"
    ))]
    pub id: Option<String>,
    #[serde(flatten)]
    pub data: Json<Map<String, Value>>,
}

impl Element {
    pub async fn insert(tx: &mut Tx<'_>, tid: &str, el_form: ElementPayload) -> Result<Element> {
        let tenant_exists = Tenant::exists(tx, tid).await?;
        if !tenant_exists {
            return Err(AppError::ResourceNotFound {
                resource: "tenant",
                attribute: "id",
                value: tid.to_string(),
            });
        }
        let id = match el_form.id {
            None => random::<u64>().to_string(),
            Some(_id) => {
                let res = sqlx::query!(
                "SELECT EXISTS(SELECT id FROM elements WHERE id = $1)", _id)
                    .fetch_one(&mut **tx)
                    .await
                    .map_err(AppError::DB)?;
                if res.exists.unwrap_or(false) {
                    return Err(AppError::Validation(
                        format!("Element with id \"{}\" already exists.", _id))
                    );
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
}

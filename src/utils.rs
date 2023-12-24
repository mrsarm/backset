use actix_contrib_rest::result::{AppError, Result};
use serde_json::{Map, Value};
use sqlx::types::Json;

pub fn reject_created_at(data: &Json<Map<String, Value>>) -> Result<()> {
    if data.contains_key("created_at") {
        return Err(AppError::StaticValidation(
            "cannot provide reserved attribute \"created_at\""));
    }
    Ok(())
}

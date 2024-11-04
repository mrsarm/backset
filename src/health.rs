use actix_web::{get, HttpResponse, Responder};
use serde_json::json;
use std::sync::LazyLock;

use crate::BACKSET_VERSION;

static HEALTH_CHECK: LazyLock<String> = LazyLock::new(|| {
    json!({
        "status": "UP",
        "service": "backset",
        "version": BACKSET_VERSION,
    }).to_string()
});

#[get("")]
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(HEALTH_CHECK.as_str())
}

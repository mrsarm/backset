use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

use crate::BACKSET_VERSION;

lazy_static! {
    static ref HEALTH_CHECK: String = json!({
        "status": "UP",
        "service": "backset",
        "version": BACKSET_VERSION,
    })
    .to_string();
}

#[get("")]
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(HEALTH_CHECK.as_str())
}

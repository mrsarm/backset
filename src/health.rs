use actix_web::{get, web, HttpResponse, Responder};
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
async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(HEALTH_CHECK.as_str())
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/health").service(health_check_handler);
    conf.service(scope);
}

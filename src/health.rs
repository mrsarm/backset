use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

#[get("")]
async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "UP",
        "service": "backset"
    }))
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/health").service(health_check_handler);
    conf.service(scope);
}

use crate::app_state::AppState;
use crate::errors::BacksetError;
use crate::tenants::model::{Tenant, TenantPayload};
use actix_web::{delete, get, post, web, HttpResponse};
use actix_web_validator::Json;

#[post("")]
async fn create(
    app: web::Data<AppState>,
    tenant_form: Json<TenantPayload>,
) -> Result<HttpResponse, BacksetError> {
    let mut tx = app.get_tx().await?;

    let tenant = Tenant::insert(&mut tx, tenant_form.0).await?;

    app.commit_tx(tx).await?;
    Ok(HttpResponse::Created().json(tenant))
}

#[get("{id}")]
async fn read(
    app: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, BacksetError> {
    let mut tx = app.get_tx().await?;

    let tenant = Tenant::get(&mut tx, id.into_inner()).await?;

    app.commit_tx(tx).await?;
    match tenant {
        Some(t) => Ok(HttpResponse::Ok().json(t)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

// TODO migration in process
// #[get("")]
// async fn list(
//     app: web::Data<AppState>,
// ) -> impl Responder {
//     let ids = sqlx::query!("SELECT id FROM tenants")
//         .fetch(&app.pool)
//         .map_ok(|record| record.id)
//         .try_collect::<Vec<_>>()
//         .await?;
//     Ok(Json(ids))
// }

#[delete("{id}")]
async fn delete(
    app: web::Data<AppState>,
    id: web::Path<i64>,
) -> Result<HttpResponse, BacksetError> {
    let mut tx = app.get_tx().await?;

    let rows_deleted = Tenant::delete(&mut tx, id.into_inner()).await?;

    app.commit_tx(tx).await?;
    match rows_deleted {
        0 => Ok(HttpResponse::NotFound().finish()),
        _ => Ok(HttpResponse::NoContent().finish()),
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/tenants").service(create).service(delete).service(read);
    conf.service(scope);
}

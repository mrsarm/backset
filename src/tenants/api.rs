use crate::errors::BacksetError;
use crate::tenants::model::{Tenant, TenantPayload};
use crate::AppState;
use actix_web::{get, post, web, HttpResponse};
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

// #[delete("/<id>")]
// async fn delete(mut db: Connection<Db>, id: i64) -> Result<Option<()>> {
//     let result = sqlx::query!("DELETE FROM tenants WHERE id = ?", id)
//         .execute(&mut *db)
//         .await?;
//     Ok((result.rows_affected() == 1).then(|| ()))
// }
//
// #[delete("/")]
// async fn destroy(mut db: Connection<Db>) -> Result<()> {
//     sqlx::query!("DELETE FROM tenants").execute(&mut *db).await?;
//     Ok(())
// }

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/tenants").service(create).service(read);
    conf.service(scope);
}

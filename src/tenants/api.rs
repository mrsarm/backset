use actix_web::{error, Error, HttpResponse, post, Responder, web};
use serde::Deserialize;
use crate::AppState;
use crate::tenants::model::{Tenant, TenantForm};

//TODO better catch serialization errors, and errors in general
#[post("")]
async fn create(
    app: web::Data<AppState>,
    tenant_form: web::Json<TenantForm>,
) -> Result<impl Responder, Error> {
    let tenant = Tenant::insert(&app.pool, tenant_form.0)
        .await
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(tenant))
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
//
// #[get("/<id>")]
// async fn read(
//     app: web::Data<AppState>,
//     id: i64
// ) -> Option<Json<Tenant>> {
//     sqlx::query!("SELECT id, name FROM tenants WHERE id = ?", id)
//         .fetch_one(&app.pool)
//         .map_ok(|r| Json(Tenant { id: Some(r.id), name: r.name }))
//         .await
//         .ok()
// }
//
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
    let scope = web::scope("/tenants").service(create);
    conf.service(scope);
}

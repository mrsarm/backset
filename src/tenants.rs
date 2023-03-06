use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
//use serde_json::json;
use crate::AppState;

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct Tenant {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TenantForm {
    pub name: String,
}

//TODO better catch serialization errors, and errors in general
#[post("")]
async fn create(
    tenant_form: web::Json<TenantForm>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        Tenant,
        "INSERT INTO tenants (name) VALUES ($1) RETURNING *",
        tenant_form.name
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(tenant) => HttpResponse::Created().json(tenant),
        // TODO better error handling, return Result instead
        Err(e) =>
            HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}))
    }
}

// TODO migration in process
// #[get("")]
// async fn list(
//     data: web::Data<AppState>,
// ) -> impl Responder {
//     let ids = sqlx::query!("SELECT id FROM tenants")
//         .fetch(&data.db)
//         .map_ok(|record| record.id)
//         .try_collect::<Vec<_>>()
//         .await?;
//     Ok(Json(ids))
// }
//
// #[get("/<id>")]
// async fn read(mut db: Connection<Db>, id: i64) -> Option<Json<Tenant>> {
//     sqlx::query!("SELECT id, name FROM tenants WHERE id = ?", id)
//         .fetch_one(&mut *db)
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

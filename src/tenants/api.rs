use crate::app_state::AppState;
use crate::core::HttpResult;
use crate::page::{Page, QuerySearch};
use crate::tenants::model::{Tenant, TenantPayload};
use actix_web::web::{Data, Path};
use actix_web::{delete, get, post, web, HttpResponse};
use actix_web_validator::{Json, Query};

#[post("")]
async fn create(app: Data<AppState>, tenant_form: Json<TenantPayload>) -> HttpResult {
    let mut tx = app.get_tx().await?;

    let tenant = Tenant::insert(&mut tx, tenant_form.0).await?;

    app.commit_tx(tx).await?;
    Ok(HttpResponse::Created().json(tenant))
}

#[get("{id}")]
async fn read(app: Data<AppState>, id: Path<i64>) -> HttpResult {
    let mut tx = app.get_tx().await?;

    let tenant = Tenant::get(&mut tx, id.into_inner()).await?;

    app.commit_tx(tx).await?;
    match tenant {
        Some(t) => Ok(HttpResponse::Ok().json(t)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[get("")]
async fn list(app: Data<AppState>, query: Query<QuerySearch>) -> HttpResult {
    let query = query.into_inner();
    let mut tx = app.get_tx().await?;
    let total = Tenant::count(&mut tx).await?;
    let tenants = match total {
        0 => Page::empty(),
        _ => {
            let data = Tenant::find(&mut tx, &query).await?;
            Page::with_data(data, total, query.offset)
        }
    };
    app.commit_tx(tx).await?;
    Ok(HttpResponse::Ok().json(tenants))
}

#[delete("{id}")]
async fn delete(app: Data<AppState>, id: Path<i64>) -> HttpResult {
    let mut tx = app.get_tx().await?;

    let rows_deleted = Tenant::delete(&mut tx, id.into_inner()).await?;

    app.commit_tx(tx).await?;
    match rows_deleted {
        0 => Ok(HttpResponse::NotFound().finish()),
        _ => Ok(HttpResponse::NoContent().finish()),
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/tenants")
        .service(create)
        .service(delete)
        .service(list)
        .service(read);
    conf.service(scope);
}

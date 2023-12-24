use actix_contrib_rest::app_state::AppState;
use actix_contrib_rest::page::Page;
use actix_contrib_rest::query::QuerySearch;
use actix_contrib_rest::result::HttpResult;
use actix_web::web::{Data, Path};
use actix_web::{delete, get, post, put, HttpResponse};
use actix_web_validator::{Json, Query};

use crate::elements::model::{Element, ElementPayload};
use crate::tenants::model::Tenant;

#[post("{tid}")]
async fn create(
    app: Data<AppState>,
    tid: Path<String>,
    el_form: Json<ElementPayload>,
) -> HttpResult {
    let mut tx = app.get_tx().await?;

    let el = Element::insert(&mut tx, tid.as_str(), el_form.0).await?;

    app.commit_tx(tx).await?;
    Ok(HttpResponse::Created().json(el))
}

#[get("{tid}/{id}")]
async fn read(app: Data<AppState>, path: Path<(String, String)>) -> HttpResult {
    let mut tx = app.get_tx().await?;

    let element = Element::get(
        &mut tx,
        path.as_ref().0.as_str(),
        path.as_ref().1.as_str()
    ).await?;

    app.commit_tx(tx).await?;
    match element {
        Some(el) => Ok(HttpResponse::Ok().json(el)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[get("{tid}")]
async fn list(
    app: Data<AppState>,
    tid: Path<String>,
    query: Query<QuerySearch>
) -> HttpResult {
    let mut tx = app.get_tx().await?;
    Tenant::exists_or_fail(&mut tx, tid.as_str()).await?;
    let query = query.into_inner();
    let total = if query.include_total.unwrap_or(true) {
        Some(Element::count(&mut tx, tid.as_str()).await?)
    } else {
        None
    };
    let elements = match total {
        Some(0) => Page::empty(),
        _ => {
            let data = Element::find(&mut tx, tid.as_str(), &query).await?;
            Page::with_data(data, total, query.offset)
        }
    };
    app.commit_tx(tx).await?;
    Ok(HttpResponse::Ok().json(elements))
}

#[put("{tid}/{id}")]
async fn put(
    app: Data<AppState>,
    path: Path<(String, String)>,
    el_form: Json<ElementPayload>,
) -> HttpResult {
    let mut tx = app.get_tx().await?;

    let el = Element::save(
        &mut tx,
        path.as_ref().0.as_str(),
        path.as_ref().1.as_str(),
        el_form.0
    ).await?;

    app.commit_tx(tx).await?;
    Ok(HttpResponse::Ok().json(el))
}

#[delete("{tid}/{id}")]
async fn delete(app: Data<AppState>, path: Path<(String, String)>) -> HttpResult {
    let mut tx = app.get_tx().await?;

    let rows_deleted = Element::delete(
        &mut tx,
        path.as_ref().0.as_str(),
        path.as_ref().1.as_str()
    ).await?;

    app.commit_tx(tx).await?;
    match rows_deleted {
        0 => Ok(HttpResponse::NotFound().finish()),
        _ => Ok(HttpResponse::NoContent().finish()),
    }
}

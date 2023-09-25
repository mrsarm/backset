use actix_contrib_rest::app_state::AppState;
use actix_contrib_rest::result::HttpResult;
use actix_web::web::{Data, Path};
use actix_web::{post, HttpResponse};
use actix_web_validator::Json;

use crate::elements::model::{Element, ElementPayload};

#[post("{tid}")]
async fn create(
    app: Data<AppState>,
    tid: Path<String>,
    el_form: Json<ElementPayload>
) -> HttpResult {
    let mut tx = app.get_tx().await?;

    let tenant = Element::insert(&mut tx, tid.as_str(), el_form.0).await?;

    app.commit_tx(tx).await?;
    Ok(HttpResponse::Created().json(tenant))
}

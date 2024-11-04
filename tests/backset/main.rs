use actix_contrib_rest::app_state::AppState;
use actix_http::Request;
use actix_web::http::header::{Accept, ContentType};
use actix_web::test::TestRequest;
use actix_web::web::Data;
use async_once_cell::OnceCell;
use backset::tenants::model::{Tenant, TenantPayload};
use backset::BACKSET_PORT;
use log::{error, LevelFilter};
use rand::random;
use serde::Serialize;
use server_env_config::env::Environment;
use server_env_config::Config;
use sqlx::Connection;
use std::process::exit;
use std::sync::{Arc, LazyLock, Once};

mod health_api_tests;
mod elements_api_tests;
mod tenants_api_tests;

static INIT: Once = Once::new();

pub async fn initialize() -> Data<AppState> {
    INIT.call_once(|| {
        dotenv::dotenv().ok(); // read conf from .env file if available
        env_logger::builder()
            .filter(Some("backset::conf"), LevelFilter::Warn)
            .filter(Some("backset::app_state"), LevelFilter::Warn)
            .init();
    });
    let config = Config::init_for(BACKSET_PORT, Some(Environment::Test))
        .unwrap_or_else(|error| {
            error!("{error}");
            exit(1);
        });
    let data = AppState::init(config.clone()).await
        .unwrap_or_else(|error| {
            error!("{error}");
            exit(1);
        });
    Data::new(data)
}

static TENANT_ID_ARC: LazyLock<Arc<OnceCell<(u16, u16, u16)>>> = LazyLock::new(|| {
    Arc::new(OnceCell::new())
});

/// Create a few tenants
pub async fn initialize_tenant<'a>(state: &Data<AppState>) -> &'a (u16, u16, u16) {
    TENANT_ID_ARC.get_or_init(async {
        let tid1 = create_tenant(state).await;
        let tid2 = create_tenant(state).await;
        let tid3 = create_tenant(state).await;
        (tid1, tid2, tid3,)
    }).await
}

pub async fn create_tenant(state: &Data<AppState>) -> u16 {
    let tid = random::<u16>();
    let tenant = TenantPayload {
        id: tid.to_string(),
        name: format!("{tid} API"),
    };
    let mut conn = state.get_conn().await.unwrap_or_else(|error| {
        error!("{error}");
        exit(1);
    });
    let mut tx = Connection::begin(&mut conn).await.unwrap_or_else(|error| {
        error!("{error}");
        exit(1);
    });
    Tenant::insert(&mut tx, tenant).await.unwrap_or_else(|error| {
        error!("{error}");
        exit(1);
    });
    state.commit_tx(tx).await.unwrap_or_else(|error| {
        error!("{error}");
        exit(1);
    });
    tid
}

pub fn get(path: &str) -> Request {
    TestRequest::get()
        .uri(path)
        .insert_header(Accept::json())
        .to_request()
}

pub fn post(path: &str, data: impl Serialize) -> Request {
    TestRequest::post()
        .uri(path)
        .insert_header(Accept::json())
        .insert_header(ContentType::json())
        .set_json(data)
        .to_request()
}

pub fn put(path: &str, data: impl Serialize) -> Request {
    TestRequest::put()
        .uri(path)
        .insert_header(Accept::json())
        .insert_header(ContentType::json())
        .set_json(data)
        .to_request()
}

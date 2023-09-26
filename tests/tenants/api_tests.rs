#[cfg(test)]
mod tests {
    use actix_contrib_rest::app_state::AppState;
    use actix_contrib_rest::page::Page;
    use actix_contrib_rest::result::ValidationErrorPayload;
    use actix_contrib_rest::test::assert_status;
    use actix_http::Request;
    use actix_web::http::header::{Accept, ContentType};
    use actix_web::http::StatusCode;
    use actix_web::test::{call_service, init_service, try_read_body_json, TestRequest};
    use actix_web::web::Data;
    use actix_web::App;
    use backset::app_server::AppServer;
    use backset::tenants::model::Tenant;
    use backset::{BACKSET_PORT, PAGE_SIZE};
    use dotenv::dotenv;
    use log::{error, LevelFilter};
    use rand::random;
    use serde::Serialize;
    use serde_json::json;
    use server_env_config::env::Environment;
    use server_env_config::Config;
    use std::error::Error;
    use std::process::exit;
    use std::sync::Once;

    static INIT: Once = Once::new();

    pub async fn initialize() -> Data<AppState> {
        INIT.call_once(|| {
            dotenv().ok(); // read conf from .env file if available
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

    #[actix_web::test]
    async fn test_health_get() {
        let state = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let req = _get("/health");
        let resp = call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_unregistered_route_get_404() {
        let state = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let req = _get("/not-a-registered-route");
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_tenants_post() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let _id = random::<u32>();
        let id = format!("ten-{_id}");
        let name = format!("The Tenant Name {_id}");
        let req = _post(json!({ "id": id, "name": name }));
        let resp = call_service(&app, req).await;
        let body = assert_status(resp, StatusCode::CREATED).await;
        let tenant: Tenant = serde_json::from_slice(&body).unwrap();
        assert_eq!(tenant.id, id);
        assert_eq!(tenant.name, name);
        Ok(())
    }

    #[actix_web::test]
    async fn test_tenants_post_and_get() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let _id = random::<u32>();
        let id = format!("another-{_id}");
        let name = format!("Another Name {_id}");
        let req = _post(json!({ "id": id, "name": name }));
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
        let tenant: Tenant = try_read_body_json(resp).await?;
        let req = _get(format!("/tenants/{}", tenant.id).as_str());
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let tenant: Tenant = try_read_body_json(resp).await?;
        assert_eq!(tenant.id, id);
        assert_eq!(tenant.name, name);
        Ok(())
    }

    #[actix_web::test]
    async fn test_tenants_get_paginated() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let _id = random::<u32>();
        let _id_prefix = format!("paginated-{_id}");
        for i in 0..80 {
            let id = format!("{_id_prefix}-{i}");
            let name = format!("Paginated {_id} {i}");
            let req = _post(json!({ "id": id, "name": name }));
            let resp = call_service(&app, req).await;
            assert_status(resp, StatusCode::CREATED).await;
        }
        let req = _get(format!("/tenants?q={_id_prefix}").as_str());
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let page: Page<Tenant> = try_read_body_json(resp).await?;
        assert!(
            page.total == Some(80),
            "page.total = {:?}, expected 80",
            page.total
        );
        assert_eq!(page.offset, 0);
        assert_eq!(page.page_size, PAGE_SIZE);
        assert_eq!(page.data.len() as i64, PAGE_SIZE);

        let req = _get("/tenants?page_size=5&offset=10");
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let page: Page<Tenant> = try_read_body_json(resp).await?;
        assert!(
            page.total >= Some(80),
            "page.total = {:?}, expected > 80",
            page.total
        );
        assert_eq!(page.offset, 10);
        assert_eq!(page.page_size, 5);
        assert_eq!(page.data.len() as i64, 5);
        Ok(())
    }

    #[actix_web::test]
    async fn test_tenants_do_not_include_total() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let _id = random::<u32>();
        let _id_prefix = format!("paginated-without-total-{_id}");
        for i in 0..5 {
            let id = format!("{_id_prefix}-{i}");
            let name = format!("Paginated Without Total {_id} {i}");
            let req = _post(json!({ "id": id, "name": name }));
            let resp = call_service(&app, req).await;
            assert_status(resp, StatusCode::CREATED).await;
        }
        let req = _get(
            format!("/tenants?q={_id_prefix}&include_total=false").as_str()
        );
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let page: Page<Tenant> = try_read_body_json(resp).await?;
        assert!(
            page.total.is_none(),
            "page.total = {:?}, expected None",
            page.total
        );
        assert_eq!(page.offset, 0);
        assert_eq!(page.page_size, 5);
        assert_eq!(page.data.len() as i64, 5);
        Ok(())
    }

    #[actix_web::test]
    async fn test_tenants_search() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let rand_for_test = random::<u32>();
        let rand_classic = format!("Classic data {rand_for_test}");
        let _id = random::<u32>();
        for i in 0..3 {
            let id = format!("classic-{rand_for_test}-{_id}-{i}");
            let name = format!("{rand_classic} {rand_for_test} {i} {_id}");
            let req = _post(json!({ "id": id, "name": name }));
            call_service(&app, req).await;
        }
        let rand_new = format!("NEW data {rand_for_test}");
        let _id = random::<u32>();
        for i in 0..3 {
            let id = format!("new-{rand_for_test}-{_id}-{i}");
            let name = format!("{rand_new} {i} {}", random::<u32>());
            let req = _post(json!({ "id": id, "name": name }));
            call_service(&app, req).await;
        }
        let req = _get(format!("/tenants?q={rand_for_test}").as_str());
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let page: Page<Tenant> = try_read_body_json(resp).await?;
        assert_eq!(page.total, Some(6));
        let q = rand_new.replace(" ", "%20"); //TODO improve query string parsing
        let req = _get(format!("/tenants?q={q}").as_str());
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let page: Page<Tenant> = try_read_body_json(resp).await?;
        assert_eq!(page.total, Some(3));
        let req = _get(format!("/tenants?q={rand_for_test}&page_size=5&sort=-name").as_str());
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let page: Page<Tenant> = try_read_body_json(resp).await?;
        assert_eq!(page.total, Some(6));
        assert_eq!(page.offset, 0);
        assert_eq!(page.page_size, 5);
        assert!(page
            .data
            .get(0)
            .map(|t| t.name.as_str())
            .unwrap_or("Not Found")
            .starts_with("NEW data"));
        Ok(())
    }

    #[actix_web::test]
    async fn test_tenants_post_already_exists() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let _id = random::<u32>();
        let id = format!("first-{_id}");
        let same_name = format!("First Time Name {_id}");
        let req = _post(json!({ "id": id, "name": same_name }));
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
        let req = _post(json!({
            "id": format!("id-{}", random::<u32>()),
            "name": same_name // same name again
        }));
        let resp = call_service(&app, req).await;
        let body = assert_status(resp, StatusCode::BAD_REQUEST).await;
        let error: ValidationErrorPayload = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            error.error,
            format!("Tenant with name \"{same_name}\" already exists.")
        );
        let req = _post(json!({
            "id": id,   // same id
            "name": format!("Some Name {}", random::<u32>())
        }));
        let resp = call_service(&app, req).await;
        let body = assert_status(resp, StatusCode::BAD_REQUEST).await;
        let error: ValidationErrorPayload = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            error.error,
            format!("Tenant with id \"{id}\" already exists.")
        );
        Ok(())
    }

    #[actix_web::test]
    async fn test_tenants_field_validations() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let req = _post(json!({
            "id": format!("validate-{}", random::<u32>()),
            "name": "Sr"
        }));
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let error: ValidationErrorPayload = try_read_body_json(resp).await?;
        assert_eq!(error.error, "Validation error");
        match error.field_errors {
            None => assert!(false, "field_errors shouldn't not be None"),
            Some(errors) => {
                assert_eq!(errors.len(), 1);
                match errors.get("name") {
                    None => assert!(false, "field_errors should contain \"name\""),
                    Some(field_errors) => {
                        assert_eq!(field_errors.len(), 1);
                        assert_eq!(&field_errors[0].code, "length");
                        match field_errors[0].params.get("min") {
                            None => assert!(false, "field_errors.params should contain \"min\""),
                            Some(v) => assert_eq!(v.to_string(), "3"),
                        }
                    }
                }
                assert_ne!(errors.get("name"), None);
            }
        }
        Ok(())
    }

    #[actix_web::test]
    async fn test_tenants_get_404() {
        let state = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let req = _get("/tenants/123456789"); // tenant record doesn't exist
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_tenants_delete() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let id = format!("to-delete-{}", random::<u32>());
        let name = format!("To Be Deleted {}", random::<u32>());
        let req = _post(json!({ "id": id, "name": name }));
        let resp = call_service(&app, req).await;
        let body = assert_status(resp, StatusCode::CREATED).await;
        let tenant: Tenant = serde_json::from_slice(&body).unwrap();
        let req = TestRequest::delete()
            .uri(format!("/tenants/{}", tenant.id).as_str())
            .to_request();
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);
        // trying to get the deleted element returns 404
        let req = _get(format!("/tenants/{}", tenant.id).as_str());
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        Ok(())
    }

    #[actix_web::test]
    async fn test_tenants_delete_404() {
        let state = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let req = TestRequest::delete()
            .uri("/tenants/123456789") // does not exist
            .to_request();
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    fn _post(data: impl Serialize) -> Request {
        TestRequest::post()
            .uri("/tenants")
            .insert_header(Accept::json())
            .insert_header(ContentType::json())
            .set_json(data)
            .to_request()
    }

    fn _get(path: &str) -> Request {
        TestRequest::get()
            .uri(path)
            .insert_header(Accept::json())
            .to_request()
    }
}

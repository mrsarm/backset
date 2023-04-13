#[cfg(test)]
mod tests {
    use actix_http::Request;
    use actix_web::http::header::{Accept, ContentType};
    use actix_web::http::StatusCode;
    use actix_web::test::{call_service, init_service, try_read_body_json, TestRequest};
    use actix_web::web::Data;
    use actix_web::App;
    use backset::app_server::AppServer;
    use backset::app_state::AppState;
    use backset::config::{Config, Environment};
    use backset::errors::ValidationErrorPayload;
    use backset::page::Page;
    use backset::PAGE_SIZE;
    use backset::tenants::model::Tenant;
    use dotenv::dotenv;
    use rand::Rng;
    use serde::Serialize;
    use serde_json::json;
    use std::error::Error;
    use std::sync::Once;

    static INIT: Once = Once::new();

    pub async fn initialize() -> Data<AppState> {
        INIT.call_once(|| {
            dotenv().ok(); // read config from .env file if available
            env_logger::init();
        });
        let config = Config::init_for(Environment::Test);
        let data = AppState::new(config.clone()).await;
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
        let name = format!("The Tenant Name {}", rand::thread_rng().gen::<u32>());
        let req = _post(json!({ "name": name }));
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
        let tenant: Tenant = try_read_body_json(resp).await?;
        assert_eq!(tenant.name, name);
        Ok(())
    }

    #[actix_web::test]
    async fn test_tenants_post_and_get() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let name = format!("Another Name {}", rand::thread_rng().gen::<u32>());
        let req = _post(json!({ "name": name }));
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
        let tenant: Tenant = try_read_body_json(resp).await?;
        let req = _get(format!("/tenants/{}", tenant.id).as_str());
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let tenant: Tenant = try_read_body_json(resp).await?;
        assert_eq!(tenant.name, name);
        Ok(())
    }

    #[actix_web::test]
    async fn test_tenants_get_paginated() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        for i in 0..79 {
            let name = format!("Paginated {} {}", i, rand::thread_rng().gen::<u32>());
            let req = _post(json!({ "name": name }));
            let resp = call_service(&app, req).await;
            assert_eq!(resp.status(), StatusCode::CREATED);
        }
        let req = _get(format!("/tenants").as_str());
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let page: Page<Tenant> = try_read_body_json(resp).await?;
        assert!(page.total >= 80, "page.total = {}, expected > 80", page.total);
        assert_eq!(page.offset, 0);
        assert_eq!(page.page_size, PAGE_SIZE);
        assert_eq!(page.data.len() as i64, PAGE_SIZE);

        let req = _get(format!("/tenants?page_size=5&offset=10").as_str());
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let page: Page<Tenant> = try_read_body_json(resp).await?;
        assert!(page.total >= 80, "page.total = {}, expected > 80", page.total);
        assert_eq!(page.offset, 10);
        assert_eq!(page.page_size,5);
        assert_eq!(page.data.len() as i64, 5);
        Ok(())
    }

    #[actix_web::test]
    async fn test_tenants_post_already_exists() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let same_name = format!("First Time Name {}", rand::thread_rng().gen::<u32>());
        let req = _post(json!({ "name": same_name }));
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
        let req = _post(json!({ "name": same_name })); // same name again
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let error: ValidationErrorPayload = try_read_body_json(resp).await?;
        assert_eq!(error.error, "Tenant already exists");
        Ok(())
    }

    #[actix_web::test]
    async fn test_tenants_field_validations() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let req = _post(json!({ "name": "Sr" }));
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
        let name = format!("To Be Deleted {}", rand::thread_rng().gen::<u32>());
        let req = _post(json!({ "name": name }));
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
        let tenant: Tenant = try_read_body_json(resp).await?;
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

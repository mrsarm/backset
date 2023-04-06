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
    use backset::tenants::model::Tenant;
    use dotenv::dotenv;
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
        let req = _post(json!({
            "name": "The Tenant Name"
        }));
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
        let tenant: Tenant = try_read_body_json(resp).await?;
        assert_eq!(tenant.name, "The Tenant Name");
        Ok(())
    }

    #[actix_web::test]
    async fn test_tenants_post_and_get() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let req = _post(json!({
            "name": "Another Name"
        }));
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
        let tenant: Tenant = try_read_body_json(resp).await?;
        let req = _get(format!("/tenants/{}", tenant.id).as_str());
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let tenant: Tenant = try_read_body_json(resp).await?;
        assert_eq!(tenant.name, "Another Name");
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

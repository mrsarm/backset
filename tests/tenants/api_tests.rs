#[cfg(test)]
mod tests {
    use actix_web::{http::header::Accept, App};
    use actix_web::http::StatusCode;
    use actix_web::test::{call_service, init_service, TestRequest};
    use actix_web::web::Data;
    use backset::app_server::AppServer;
    use backset::app_state::AppState;
    use backset::config::Config;
    use dotenv::dotenv;
    use std::sync::Once;

    static INIT: Once = Once::new();

    pub async fn initialize() -> (Config, Data<AppState>) {
        INIT.call_once(|| {
            dotenv().ok();  // read config from .env file if available
            env_logger::init();
        });
        let config = Config::init();
        let data = AppState::new(config.clone()).await;
        (config, Data::new(data))
    }

    #[actix_web::test]
    async fn test_health_get() {
        let (_, state) = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let req = TestRequest::get()
            .uri("/health")
            .insert_header(Accept::json())
            .to_request();
        let resp = call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_tenants_get_404() {
        let (_, state) = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let req = TestRequest::get()
            .uri("/tenants/123456789")
            .insert_header(Accept::json())
            .to_request();
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}

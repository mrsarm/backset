
#[cfg(test)]
mod tests {
    use actix_web::{http::header::Accept, test, web, App};
    use actix_web::middleware::Logger;
    use actix_web_validator::JsonConfig;
    use backset::config::Config;
    use backset::app_state::AppState;
    use backset::errors::json_error_handler;
    use backset::health;
    use dotenv::dotenv;

    #[actix_web::test]
    async fn test_health_get() {
        dotenv().ok();
        let config = Config::init();
        let state = AppState::new(config.clone()).await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(state.clone()))
                .app_data(JsonConfig::default().error_handler(json_error_handler))
                .configure(health::config)
                .wrap(Logger::default())
        ).await;
        let req = test::TestRequest::get()
            .uri("/health")
            .insert_header(Accept::json())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}

#[cfg(test)]
mod tests {
    use actix_web::{http::header::Accept, test, App};
    use actix_web::web::Data;
    use backset::app_server::AppServer;
    use backset::config::Config;
    use backset::app_state::AppState;

    #[actix_web::test]
    async fn test_health_get() {
        let config = Config::init();
        let state = AppState::new(config.clone()).await;
        let app = test::init_service(
            App::new().configure(AppServer::config_app(Data::new(state.clone())))
        ).await;
        let req = test::TestRequest::get()
            .uri("/health")
            .insert_header(Accept::json())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
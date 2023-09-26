#[cfg(test)]
mod tests {
    use actix_web::test::{call_service, init_service};
    use actix_web::App;
    use actix_web::http::StatusCode;
    use backset::app_server::AppServer;
    use crate::{get, initialize};

    #[actix_web::test]
    async fn test_health_get() {
        let state = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let req = get("/health");
        let resp = call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_unregistered_route_get_404() {
        let state = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let req = get("/not-a-registered-route");
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}

#[cfg(test)]
mod tests {
    use crate::{get, post, initialize};
    use actix_contrib_rest::page::Page;
    use actix_contrib_rest::result::{DeletedCount, ValidationErrorPayload};
    use actix_contrib_rest::test::assert_status;
    use actix_web::http::StatusCode;
    use actix_web::test::{call_service, init_service, try_read_body_json, TestRequest};
    use actix_web::App;
    use backset::app_server::AppServer;
    use backset::tenants::model::Tenant;
    use backset::PAGE_SIZE;
    use rand::random;
    use serde_json::json;
    use std::error::Error;

    #[actix_web::test]
    async fn test_tenants_post() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let _id = random::<u32>();
        let id = format!("ten-{_id}");
        let name = format!("The Tenant Name {_id}");
        let req = post("/tenants", json!({ "id": id, "name": name }));
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
        let req = post("/tenants", json!({ "id": id, "name": name }));
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
        let tenant: Tenant = try_read_body_json(resp).await?;
        let req = get(format!("/tenants/{}", tenant.id).as_str());
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let tenant: Tenant = try_read_body_json(resp).await?;
        assert_eq!(tenant.id, id);
        assert_eq!(tenant.name, name);
        Ok(())
    }

    #[actix_web::test]
    async fn test_tenant_not_found() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let _id = random::<u32>();
        let req = get(format!("/tenants/does-not-exist-{_id}").as_str());
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
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
            let req = post("/tenants", json!({ "id": id, "name": name }));
            let resp = call_service(&app, req).await;
            assert_status(resp, StatusCode::CREATED).await;
        }
        let req = get(format!("/tenants?q={_id_prefix}").as_str());
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

        let req = get("/tenants?page_size=5&offset=10");
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
            let req = post("/tenants", json!({ "id": id, "name": name }));
            let resp = call_service(&app, req).await;
            assert_status(resp, StatusCode::CREATED).await;
        }
        let req = get(
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
            let req = post("/tenants", json!({ "id": id, "name": name }));
            call_service(&app, req).await;
        }
        let rand_new = format!("NEW data {rand_for_test}");
        let _id = random::<u32>();
        for i in 0..3 {
            let id = format!("new-{rand_for_test}-{_id}-{i}");
            let name = format!("{rand_new} {i} {}", random::<u32>());
            let req = post("/tenants", json!({ "id": id, "name": name }));
            call_service(&app, req).await;
        }
        let req = get(format!("/tenants?q={rand_for_test}").as_str());
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let page: Page<Tenant> = try_read_body_json(resp).await?;
        assert_eq!(page.total, Some(6));
        let q = rand_new.replace(" ", "%20"); //TODO improve query string parsing
        let req = get(format!("/tenants?q={q}").as_str());
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let page: Page<Tenant> = try_read_body_json(resp).await?;
        assert_eq!(page.total, Some(3));
        let req = get(format!("/tenants?q={rand_for_test}&page_size=5&sort=-name").as_str());
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
        let req = post("/tenants", json!({ "id": id, "name": same_name }));
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
        let req = post("/tenants", json!({
            "id": format!("id-{}", random::<u32>()),
            "name": same_name // same name again
        }));
        let resp = call_service(&app, req).await;
        let body = assert_status(resp, StatusCode::BAD_REQUEST).await;
        let error: ValidationErrorPayload = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            error.error,
            format!("tenant with name \"{same_name}\" already exists")
        );
        let req = post("/tenants", json!({
            "id": id,   // same id
            "name": format!("Some Name {}", random::<u32>())
        }));
        let resp = call_service(&app, req).await;
        let body = assert_status(resp, StatusCode::BAD_REQUEST).await;
        let error: ValidationErrorPayload = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            error.error,
            format!("tenant with id \"{id}\" already exists")
        );
        Ok(())
    }

    #[actix_web::test]
    async fn test_tenants_field_validations() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let req = post("/tenants", json!({
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
        let req = get("/tenants/123456789"); // tenant record doesn't exist
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_tenants_delete() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let tid = format!("to-delete-{}", random::<u32>());
        let name = format!("To Be Deleted {}", random::<u32>());
        let req = post("/tenants", json!({ "id": tid, "name": name }));
        let resp = call_service(&app, req).await;
        let body = assert_status(resp, StatusCode::CREATED).await;
        let tenant: Tenant = serde_json::from_slice(&body).unwrap();
        let req = TestRequest::delete()
            .uri(format!("/tenants/{}", tenant.id).as_str())
            .to_request();
        let resp = call_service(&app, req).await;
        let body = assert_status(resp, StatusCode::OK).await;
        let deleted_count: DeletedCount = serde_json::from_slice(&body).unwrap();
        assert_eq!(deleted_count.deleted, 1);
        // trying to get the deleted element returns 404
        let req = get(format!("/tenants/{}", tenant.id).as_str());
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        Ok(())
    }

    #[actix_web::test]
    async fn test_tenants_delete_with_elements_error() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let tid = format!("not-delete-with-elements-{}", random::<u32>());
        let name = format!("Not Delete with Elements {}", random::<u32>());
        let req = post("/tenants", json!({ "id": tid, "name": name }));
        let resp = call_service(&app, req).await;
        let body = assert_status(resp, StatusCode::CREATED).await;

        // Add at least one element
        let el_name = format!("Element to Not Delete {}", random::<u32>());
        let req = post(format!("/{}", tid).as_str(), json!({ "name": el_name }));
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);

        let tenant: Tenant = serde_json::from_slice(&body).unwrap();
        let req = TestRequest::delete()
            .uri(format!("/tenants/{}", tenant.id).as_str())
            .to_request();
        let resp = call_service(&app, req).await;
        let body = assert_status(resp, StatusCode::BAD_REQUEST).await;
        let error: ValidationErrorPayload = serde_json::from_slice(&body).unwrap();
        assert_eq!(error.error, "cannot delete tenant with elements");
        Ok(())
    }

    #[actix_web::test]
    async fn test_tenants_delete_with_elements_force() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let tid = format!("force-delete-with-elements-{}", random::<u32>());
        let name = format!("Force Delete with Elements {}", random::<u32>());
        let req = post("/tenants", json!({ "id": tid, "name": name }));
        let resp = call_service(&app, req).await;
        let body = assert_status(resp, StatusCode::CREATED).await;

        // Add at least one element
        let el_name = format!("Element to Delete {}", random::<u32>());
        let req = post(format!("/{}", tid).as_str(), json!({ "name": el_name }));
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);

        let tenant: Tenant = serde_json::from_slice(&body).unwrap();
        let req = TestRequest::delete()
            .uri(format!("/tenants/{}?force=true", tenant.id).as_str())
            .to_request();
        let resp = call_service(&app, req).await;
        let body = assert_status(resp, StatusCode::OK).await;
        let deleted_count: DeletedCount = serde_json::from_slice(&body).unwrap();
        assert_eq!(deleted_count.deleted, 2);
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
}

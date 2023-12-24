#[cfg(test)]
mod tests {
    use crate::{get, post, put, create_tenant, initialize, initialize_tenant};
    use actix_contrib_rest::page::Page;
    use actix_contrib_rest::result::ValidationErrorPayload;
    use actix_contrib_rest::test::assert_status;
    use actix_web::http::StatusCode;
    use actix_web::test::{call_service, init_service, try_read_body_json, TestRequest};
    use actix_web::App;
    use backset::app_server::AppServer;
    use backset::elements::model::ElementPayload;
    use backset::PAGE_SIZE;
    use rand::random;
    use serde_json::json;
    use std::error::Error;

    #[actix_web::test]
    async fn test_element_post() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let tids: &(u16, u16, u16) = initialize_tenant(&state).await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let _id = random::<u32>();
        let id = format!("el-{_id}");
        let val = format!("Element from tenant {_id}");
        let req = post(format!("/{}", tids.0).as_str(), json!({
            "id": id,
            "val": val,         // Testing is capable of store
            "bool": true,       // any type of data
            "numb": 1234,
            "nil": None::<u32>,
            "arr": [
                "str",
                0,              // including arrays
                {"a": 1,},      // and nested objects
            ]
        }));
        let resp = call_service(&app, req).await;
        let body = assert_status(resp, StatusCode::CREATED).await;
        let el: ElementPayload = serde_json::from_slice(&body).unwrap();
        assert_eq!(el.id, Some(id));
        assert_eq!(el.data.get("val"),  Some(&json!(val)));
        assert_eq!(el.data.get("bool"), Some(&json!(true)));
        assert_eq!(el.data.get("numb"), Some(&json!(1234)));
        assert_eq!(el.data.get("nil"),  Some(&json!(None::<u32>)));
        assert_eq!(el.data.get("arr"),  Some(&json!(["str",0,{"a": 1,},])));
        assert!(el.data.get("created_at").is_some());
        Ok(())
    }

    #[actix_web::test]
    async fn test_elements_post_and_get() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let tids: &(u16, u16, u16) = initialize_tenant(&state).await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let _id = random::<u32>();
        let id = format!("another-{_id}");
        let name = format!("Another Element {_id}");
        let req = post(format!("/{}", tids.1).as_str(), json!({ "id": id, "name": name }));
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
        let req = get(&format!("/{}/{}", tids.1, id));
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let el: ElementPayload = try_read_body_json(resp).await?;
        assert_eq!(el.id, Some(id));
        assert_eq!(el.data.get("name"),  Some(&json!(name)));
        assert!(el.data.get("created_at").is_some());
        Ok(())
    }

    #[actix_web::test]
    async fn test_elements_post_with_auto_id() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let tids: &(u16, u16, u16) = initialize_tenant(&state).await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let req = post(&format!("/{}", tids.0), json!({
            "name": "Element with id assigned automatically",
        }));
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
        let el: ElementPayload = try_read_body_json(resp).await?;
        assert!(el.id.is_some());
        // now check with the id generated we can get the element created
        let req = get(format!("/{}/{}", tids.0, el.id.unwrap()).as_str());
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let el: ElementPayload = try_read_body_json(resp).await?;
        assert_eq!(el.data.get("name"),  Some(&json!("Element with id assigned automatically")));
        Ok(())
    }

    #[actix_web::test]
    async fn test_element_not_found() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let tids: &(u16, u16, u16) = initialize_tenant(&state).await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let _id = random::<u32>();
        // Tenant does exist, but not the element
        let req = get(&format!("/{}/does-not-exist-{}", tids.0, _id));
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        // Neither tenant or element exist
        let req = get(format!("/{}/does-not-exist-{}", tids.0 + 10, _id).as_str());
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        Ok(())
    }

    #[actix_web::test]
    async fn test_element_not_found_in_wrong_tenant() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let tids: &(u16, u16, u16) = initialize_tenant(&state).await;
        let tid = tids.1;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let _id = random::<u32>();
        let id = format!("el-{_id}");
        let name = format!("Element {_id} for Tenant {tid}");
        let req = post(&format!("/{tid}"), json!({ "id": id, "name": name }));
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
        // Now trying to retrieve the right element id from the wrong tenant returns 404
        let req = get(format!("/{}/{}", tids.0, id).as_str());
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        // using invalid tid also returns 404
        let req = get(format!("/does-not-exist-{}/{}", tids.0 + 10, id).as_str());
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        Ok(())
    }

    #[actix_web::test]
    async fn test_elements_get_paginated() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let tid = create_tenant(&state).await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let _id = random::<u32>();
        let _id_prefix = format!("paginated-{_id}");
        let req = get(&format!("/{tid}"));
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let page: Page<ElementPayload> = try_read_body_json(resp).await?;
        assert!(
            page.total == Some(0),
            "page.total = {:?}, expected 0",
            page.total
        );
        for i in 0..60 {
            let id = format!("{_id_prefix}-{i}");
            let name = format!("Paginated {_id} {i}");
            let req = post(&format!("/{tid}"), json!({ "id": id, "name": name }));
            let resp = call_service(&app, req).await;
            assert_status(resp, StatusCode::CREATED).await;
        }
        let req = get(&format!("/{tid}"));
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let page: Page<ElementPayload> = try_read_body_json(resp).await?;
        assert!(
            page.total == Some(60),
            "page.total = {:?}, expected 60",
            page.total
        );
        assert_eq!(page.offset, 0);
        assert_eq!(page.page_size, PAGE_SIZE);
        assert_eq!(page.data.len() as i64, PAGE_SIZE);

        let req = get(&format!("/{tid}?page_size=5&offset=10"));
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let page: Page<ElementPayload> = try_read_body_json(resp).await?;
        assert!(
            page.total == Some(60),
            "page.total = {:?}, expected > 60",
            page.total
        );
        assert_eq!(page.offset, 10);
        assert_eq!(page.page_size, 5);
        assert_eq!(page.data.len() as i64, 5);
        Ok(())
    }

    #[actix_web::test]
    async fn test_elements_get_paginated_not_found() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let _id = random::<u32>();
        let req = get(&format!("/tenant-does-not-exist-{_id}"));
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        Ok(())
    }

    #[actix_web::test]
    async fn test_elements_post_already_exists() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let tids: &(u16, u16, u16) = initialize_tenant(&state).await;
        let tid = tids.1;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let _id = random::<u32>();
        let id = format!("first-{_id}");
        let same_name = format!("First Time Name {_id}");
        let req = post(&format!("/{tid}"), json!({ "id": id, "val": same_name }));
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::CREATED);
        // use the same id for a new element under the same tenant is not allowed
        let req = post(&format!("/{tid}"), json!({ "id": id, "val": "Same id" }));
        let resp = call_service(&app, req).await;
        let body = assert_status(resp, StatusCode::BAD_REQUEST).await;
        let error: ValidationErrorPayload = serde_json::from_slice(&body).unwrap();
        assert_eq!(error.code, Some("already_exists".to_string()));
        assert_eq!(
            error.error,
            format!("element with id \"{id}\" already exists")
        );
        // If we use the same id under another tenants it works
        let req = post(&format!("/{}", tids.0), json!({ "id": id, "val": "Same id" }));
        let resp = call_service(&app, req).await;
        assert_status(resp, StatusCode::CREATED).await;
        Ok(())
    }

    #[actix_web::test]
    async fn test_elements_delete() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let tids: &(u16, u16, u16) = initialize_tenant(&state).await;
        let tid = tids.2;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let id = format!("to-delete-{}", random::<u32>());
        let name = format!("To Be Deleted {}", random::<u32>());
        let req = post(&format!("/{tid}"), json!({ "id": id, "name": name }));
        let resp = call_service(&app, req).await;
        assert_status(resp, StatusCode::CREATED).await;
        let req = TestRequest::delete().uri(&format!("/{tid}/{id}")).to_request();
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);
        // trying to get the deleted element returns 404
        let req = get(&format!("/{tid}/{id}"));
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        Ok(())
    }

    #[actix_web::test]
    async fn test_elements_put_and_get() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let tids: &(u16, u16, u16) = initialize_tenant(&state).await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let _id = random::<u32>();
        let id = format!("put-{_id}");
        let some_data = format!("El {_id}");
        let req = put(format!("/{}/{}", tids.1, id).as_str(), json!({ "some": some_data }));
        let resp = call_service(&app, req).await;
        let body = assert_status(resp, StatusCode::OK).await;
        let el: ElementPayload = serde_json::from_slice(&body).unwrap();
        assert_eq!(el.id, Some(id.clone()));
        assert_eq!(el.data.get("some"), Some(&json!(some_data)));
        assert!(el.data.get("created_at").is_some());
        let req = get(&format!("/{}/{}", tids.1, id));
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let el: ElementPayload = try_read_body_json(resp).await?;
        assert_eq!(el.id, Some(id.clone()));
        assert_eq!(el.data.get("some"), Some(&json!(some_data)));
        Ok(())
    }

    #[actix_web::test]
    async fn test_elements_put_over_existing_element() -> Result<(), Box<dyn Error>> {
        let state = initialize().await;
        let tids: &(u16, u16, u16) = initialize_tenant(&state).await;
        let app = init_service(App::new().configure(AppServer::config_app(state))).await;
        let _id = random::<u32>();
        let id = format!("post-put-{_id}");
        let some_data = format!("El {_id}");
        let req = post(format!("/{}", tids.1).as_str(), json!({ "some": some_data, "id": id }));
        let resp = call_service(&app, req).await;
        let body = assert_status(resp, StatusCode::CREATED).await;
        let el: ElementPayload = serde_json::from_slice(&body).unwrap();
        let created_at = el.data.get("created_at");
        assert!(created_at.is_some());
        let some_data_edited = format!("El {_id} edited");
        let req = put(format!("/{}/{}", tids.1, id).as_str(), json!({ "some": some_data_edited }));
        let resp = call_service(&app, req).await;
        let body = assert_status(resp, StatusCode::OK).await;
        let el: ElementPayload = serde_json::from_slice(&body).unwrap();
        // created_at still has the same value
        assert_eq!(el.data.get("created_at"), created_at);
        // some field changed
        assert_eq!(el.data.get("some"), Some(&json!(some_data_edited)));
        let req = get(&format!("/{}/{}", tids.1, id));
        let resp = call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let el: ElementPayload = try_read_body_json(resp).await?;
        assert_eq!(el.id, Some(id.clone()));
        assert_eq!(el.data.get("some"), Some(&json!(some_data_edited)));
        Ok(())
    }
}

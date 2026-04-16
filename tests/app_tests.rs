use phm::{
    App, GET,
    HttpMethod::{self, POST},
    Request, RequestError, Resolution,
};


#[tokio::test]
async fn test_static_vs_param_precedence() {
    let mut app = App::bind("127.0.0.1:0").await.unwrap();

    let y = |req: &mut Request, res: &mut Resolution| async move {
        
    
        Ok(())
    };

    app.add_route(GET, "/api/closed", vec![], y).await.unwrap();

    app.add_route(GET, "/api/:name", vec![], |_, _| async move {
        println!("param");
        Ok(())
    })
    .await
    .unwrap();

    let router = app.get_router();

    let node = router.get_route("/api/closed", GET).await.unwrap();
    let node_lock = node.write().await;

    let handler = node_lock.request_fn().unwrap().clone();

    let mut req = Request::parse();
    let mut res = Resolution::new();

    handler(&mut req, &mut res).await;

    // Expect "static", not "param"
}

#[tokio::test]
async fn test_param_extraction() {
    let mut app = App::bind("127.0.0.1:0").await.unwrap();

    app.add_route(GET, "/api/:name", vec![], |req, _| async move { Ok(()) })
        .await
        .unwrap();

    let router = app.get_router();

    let node = router.get_route("/api/john", GET).await.unwrap();
    let node_lock = node.write().await;

    let handler = node_lock.request_fn().unwrap().clone();

    let mut req = Request::parse();

    let mut res = Resolution::new();

    handler(&mut req, &mut res).await;
}

#[tokio::test]
async fn test_nested_params() {
    let mut app = App::bind("127.0.0.1:0").await.unwrap();

    app.add_route(
        GET,
        "/api/:version/users/:id",
        vec![],
        |req, _| async move { Ok(()) },
    )
    .await
    .unwrap();

    let router = app.get_router();

    let node = router.get_route("/api/v1/users/42", GET).await.unwrap();
    let node_lock = node.write().await;

    let handler = node_lock.request_fn().unwrap().clone();

    let mut req = Request::parse();

    let mut res = Resolution::new();

    handler(&mut req, &mut res).await;
}

#[tokio::test]
async fn test_method_routing() {
    let mut app = App::bind("127.0.0.1:0").await.unwrap();

    app.add_route(GET, "/api/item", vec![], |_, _| async move {
        println!("GET");
        Ok(())
    })
    .await
    .unwrap();

    app.add_route(POST, "/api/item", vec![], |_, _| async move {
        println!("POST");
        Ok(())
    })
    .await
    .unwrap();

    let router = app.get_router();

    assert!(router.get_route("/api/item", GET).await.is_ok());
    assert!(router.get_route("/api/item", POST).await.is_ok());
}

#[tokio::test]
async fn test_route_not_found() {
    let mut app = App::bind("127.0.0.1:0").await.unwrap();

    app.add_route(GET, "/api/test", vec![], |_, _| async move { Ok(()) })
        .await
        .unwrap();

    let router = app.get_router();

    let result = router.get_route("/api/unknown", GET).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_overlapping_routes() {
    let mut app = App::bind("127.0.0.1:0").await.unwrap();

    app.add_route(GET, "/api/:name/details", vec![], |_, _| async move {
        println!("details");
        Ok(())
    })
    .await
    .unwrap();

    app.add_route(GET, "/api/:name/:action", vec![], |_, _| async move {
        println!("generic");
        Ok(())
    })
    .await
    .unwrap();

    let router = app.get_router();

    let node = router.get_route("/api/john/details", GET).await.unwrap();
    let node_lock = node.write().await;

    let handler = node_lock.request_fn().unwrap().clone();

    let mut req = Request::parse();

    let mut res = Resolution::new();

    handler(&mut req, &mut res).await;

    // Should prefer "/api/:name/details"
}

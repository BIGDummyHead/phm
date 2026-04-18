use phm::{App, GET, HttpMethod::POST, HttpRequest, RequestError, Resolution};

#[tokio::test]
async fn test_router() {
    let app = App::bind("127.0.0.1:0").await.unwrap();

    let module = app.module("/api");

    module
        .get("/closed", vec![], |req, res| async move { Ok(()) })
        .await;

    let closed_route = app.get_router().get_route("/api/closed", GET).await;

    assert!(closed_route.is_ok(), "Could not find route");

    let closed_route = closed_route.unwrap();

    let route_node = closed_route.read().await;

    let req_fn = route_node.request_fn();

    assert!(req_fn.is_some(), "no request fn associated");

    let _ = req_fn.unwrap();

}

async fn start_test() {
    let app = App::bind("127.0.0.1:80").await.expect("failed to bind app");

    app.get("/api/closed", vec![], |req, res| async move { Ok(()) })
        .await;

    let running_app = app.start();
}

use phm::{App, GET, HttpMethod, Request, Resolution};

#[tokio::test]
async fn app_test() {
    let mut app = App::bind("127.0.0.1:8080").await.expect("could not bind");

    app.add_route(GET, "/api/closed", vec![], |req, res| async move {
        Ok(())
    }).await.expect("could not add api closed");

    let router = app.get_router();

    let node = router.get_route("/api/closed", GET).await.expect("FUCKKKK");

    let mut req = Request::parse();
    let mut res = Resolution::new();

    let node_lock = node.write().await;

    let req_fn = node_lock.request_fn().expect("FUCK AGAIN").clone();

    (*req_fn)(&mut req, &mut res).await;
}

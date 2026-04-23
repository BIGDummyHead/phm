use std::time::Duration;

use phm::{App, GET, HttpMethod::POST, HttpRequest, RequestError, Resolution};

#[tokio::test]
async fn start_test() {
    let mut app = App::bind("127.0.0.1:80").await.expect("failed to bind app");

    app.get("/api/closed/:id", vec![], |req, _res| async move {
        let id = req.variables().get_route_variable::<i32>("id");
        dbg!(id);
        Ok(())
    })
    .await;

    let running_app = app.start().await;

    tokio::time::sleep(Duration::from_secs(20)).await;

    let mut app = running_app.close().await;
}

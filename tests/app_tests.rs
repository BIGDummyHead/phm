use std::time::Duration;

use phm::{App, GET, HttpMethod::POST, HttpRequest, RequestError, Response};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    name: String,
    age: i32,
}

#[test]
fn start_test() {
    smol::block_on(async move {
        let mut app = App::bind("127.0.0.1:80").await.expect("failed to bind app");

        app.add_route(GET, "/api/:user_id", vec![], |req, res| {
            Box::pin(async move {
                let user = User {
                    name: String::from("Shawn"),
                    age: req.variables().get_route_variable::<i32>("user_id")?,
                };
                res.json(&user)?.status(200);
                Ok(())
            })
        })
        .await
        .expect("Failed");


        let running_app = app.start().await;

        loop {}
    });
}

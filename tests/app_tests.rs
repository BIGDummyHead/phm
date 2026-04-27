use phm::{App, middleware, request};

#[test]
fn start_test() {
    smol::block_on(async move {
        let app = App::bind("127.0.0.1:80").await.expect("failed to bind app");

        app.get(
            "/test",
            middleware!(),
            request!(|_req, res| {
                res.status(200).text("Hello world!");
                Ok(())
            }),
        )
        .await;

        app.start();

        loop {}
    });
}

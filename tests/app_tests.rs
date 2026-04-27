use std::pin::Pin;

use phm::{App, HttpRequest, RequestError, Response, middleware, request};

async fn some_request(req: &mut HttpRequest<'_>, res: &mut Response) -> Result<(), RequestError> {
    res.status(200).text("some text");
    Ok(())
}


// what the user's request will get transformed into
fn transformed_request<'a, 'b>(
    req: &'a mut HttpRequest<'b>,
    res: &'a mut Response,
) -> Pin<Box<dyn Future<Output = Result<(), RequestError>> + Send + 'a>> {
    Box::pin(async move {
        res.status(200).text("some text");
        Ok(())
    })
}

#[test]
fn start_test() {
    smol::block_on(async move {
        let app = App::bind("127.0.0.1:80").await.expect("failed to bind app");

        app.get(
            "/test",
            middleware!(),
            request!(|req, res| {
                res.status(200).text("Hello world!");
                Ok(())
            }),
        )
        .await;

        app.start();

        loop {}
    });
}

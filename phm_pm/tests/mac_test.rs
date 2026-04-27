#[cfg(test)]
mod tests {
    use phm::{
        App, Middleware, app::ClosedAppExt, middleware, web::ArcMiddlewareClosure,
    };
    use phm_pm::request;

    fn auth() -> ArcMiddlewareClosure {
        middleware(|_req, _res| Box::pin(async move { Middleware::Next }))
    }

    #[request(route = "/test", method = "GET", middleware(auth))]
    async fn test(_req: &mut HttpRequest<'_>, _res: Response) -> Result<(), RequestError> {
        Ok(())
    }

    #[test]
    fn test_add() -> () {
        smol::block_on(async move {
            let app = App::bind("127.0.0.1:80").await.expect("failed to bind");

            app.add_def(test).await.expect("failed to add route...");

            app.start();
        });
    }
}

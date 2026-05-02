#[cfg(test)]
#[phm_pm::postman_module]
mod tests {
    use std::io::stdin;

    use phm::{App, app::ClosedAppExt};
    use phm_pm::postman_info;

    postman_info!("API Collection");

    /// # post_user
    ///
    /// Allows you to create a user!
    #[phm_pm::postman]
    #[phm_pm::route(route = "/api/user", method = "POST")]
    #[allow(dead_code)]
    async fn post_user(_req: &mut HttpRequest<'_>, res: &mut Response) -> Result<(), RequestError> {
        res.status(200).text("You did it!");

        Ok(())
    }

    #[phm_pm::postman]
    #[phm_pm::route(route = "/api/user", method = "GET")]
    #[allow(dead_code)]
    async fn get_user(_req: &mut HttpRequest<'_>, res: &mut Response) -> Result<(), RequestError> {
        res.status(200).text("You found him it!");

        Ok(())
    }

    #[test]
    fn test() {
        smol::block_on(async move {
            let socket = "127.0.0.1:80";
            let app = App::bind(socket).await.expect("Failed to bind");

            let _ = app.add_def(get_user).await;
            let _ = app.add_def(post_user).await;

            let _ = app.start();

            println!("http://{socket}");
            println!("Press [ENTER] to exit");
            let mut buf = String::new();
            let _ = stdin().read_line(&mut buf);
        });
    }
}

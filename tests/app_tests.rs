use phm::{App, GET, Middleware, middleware};

#[test]
fn start_test() {
    smol::block_on(async move {
        let app = App::bind("127.0.0.1:80").await.expect("failed to bind app");

        let stop_1 = middleware(|req, _res| {
            Box::pin(async move {
                req.variables_mut().set_variable("user", "Shawn");
                Middleware::Next
            })
        });

        let stop_2 = middleware(|req, _res| {
            Box::pin(async move {
                let var = req
                    .variables()
                    .get_variable::<&str>("user")
                    .expect("Failed to get that var");
                println!("Var: {var}");
                Middleware::Next
            })
        });

        let stop_3 = middleware(|_req, res| {
            Box::pin(async move {
                res.status(500).text("Fuck off!");
                Middleware::Stop
            })
        });

        app.add_route(
            GET,
            "/:user_id",
            vec![stop_1, stop_2, stop_3],
            |_req, _res| {
                Box::pin(async move {
                    //res.json(&user)?.status(200);
                    Ok(())
                })
            },
        )
        .await
        .expect("Failed");

        app.start();

        loop {}
    });
}

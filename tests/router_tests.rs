use std::collections::HashMap;
use std::sync::Arc;

use phm::web::ArcRequestClosure;
use phm::{HttpMethod, Router, RouterError};

fn dummy_handler() -> ArcRequestClosure {
    Arc::new(|_req, _res| Box::pin(async { Ok(()) }))
}

#[test]
fn routes_sharing_prefix_both_resolve() {
    smol::block_on(async move {
        let router: Router<'static> = Router::new();

        router
            .add_route("/api/user", HttpMethod::POST, vec![], dummy_handler())
            .await
            .expect("first route should add");

        router
            .add_route("/api/user-signin", HttpMethod::POST, vec![], dummy_handler())
            .await
            .expect("second route should add");

        let mut vars = HashMap::new();

        let user_node = router
            .get_route("/api/user", HttpMethod::POST, &mut vars)
            .await
            .expect("/api/user should resolve");
        assert!(user_node.read().await.request_fn().is_some());

        let signin_node = router
            .get_route("/api/user-signin", HttpMethod::POST, &mut vars)
            .await
            .expect("/api/user-signin should resolve");
        assert!(signin_node.read().await.request_fn().is_some());
    });
}

#[test]
fn different_methods_on_same_path_coexist() {
    smol::block_on(async move {
        let router: Router<'static> = Router::new();

        router
            .add_route(
                "/api/user/:user_id",
                HttpMethod::GET,
                vec![],
                dummy_handler(),
            )
            .await
            .expect("GET should add");

        router
            .add_route(
                "/api/user/:user_id",
                HttpMethod::PATCH,
                vec![],
                dummy_handler(),
            )
            .await
            .expect("PATCH should add");

        router
            .add_route("/api/user", HttpMethod::POST, vec![], dummy_handler())
            .await
            .expect("POST /api/user should add");

        let mut vars = HashMap::new();

        let get_node = router
            .get_route("/api/user/abc", HttpMethod::GET, &mut vars)
            .await
            .expect("GET should resolve");
        assert!(get_node.read().await.request_fn().is_some());

        let patch_node = router
            .get_route("/api/user/abc", HttpMethod::PATCH, &mut vars)
            .await
            .expect("PATCH should resolve");
        assert!(patch_node.read().await.request_fn().is_some());

        let post_node = router
            .get_route("/api/user", HttpMethod::POST, &mut vars)
            .await
            .expect("POST should resolve");
        assert!(post_node.read().await.request_fn().is_some());
    });
}

#[test]
fn duplicate_route_returns_already_exist() {
    smol::block_on(async move {
        let router: Router<'static> = Router::new();

        router
            .add_route("/api/user", HttpMethod::POST, vec![], dummy_handler())
            .await
            .expect("first should succeed");

        let dup = router
            .add_route("/api/user", HttpMethod::POST, vec![], dummy_handler())
            .await;
        assert!(matches!(dup, Err(RouterError::AlreadyExist)));
    });
}

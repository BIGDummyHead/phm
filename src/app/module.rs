use crate::{
    App, HttpMethod,
    app::Closed,
    router::RouterError,
    web::{ArcMiddlewareClosure, RequestClosure, RequestFuture},
};

use crate::web::RequestBound;

/// # Module
///
/// A module that contains a base route.
///
/// This allows you to create a module like `/api` and then add routes on to the underlying app router.
pub struct Module<'app>
where
    'app: 'static,
{
    base_route: &'app str,
    app: &'app App<'app, Closed>,
}

macro_rules! http_module_fn {
    ($name:ident, $meth:expr) => {
        pub async fn $name<F>(
            &self,
            route: &'static str,
            middleware: Vec<ArcMiddlewareClosure>,
            req_fn: F,
        ) -> ()
        where
            F: for<'a> RequestBound<'a>,
        {
            let appended_route = self.clean_route(route.into());
            self.app
                .add_route($meth, appended_route.leak(), middleware, req_fn)
                .await
                .expect("invalid or empty route.");
        }
    };
}

impl<'app> Module<'app> {
    /// # New
    ///
    /// Creates a new module with a reference to the app and a base route to use.
    pub fn new(base_route: &'app str, app: &'app App<'app, Closed>) -> Module<'app> {
        Self { base_route, app }
    }

    // Cleans the route, ensuring that there is no overlapping '/' marker to be appended to the base.
    fn clean_route(&self, mut append: String) -> String {
        let base = self.base_route;

        if append.starts_with("/") && base.ends_with("/") {
            append.remove(0);
        }

        format!("{base}{append}")
    }

    /// # Add Route
    ///
    /// Works in the same manner as the app add route however, appends your base route to it.
    ///
    /// To see further documentation ensue the `App.rs`.    
    pub async fn add_route<F>(
        &self,
        method: HttpMethod,
        route: impl Into<String>,
        middleware: Vec<ArcMiddlewareClosure>,
        req_fn: F,
    ) -> Result<(), RouterError>
    where
        F: for<'a> RequestBound<'a>
    {
        let appended_route = self.clean_route(route.into());
        self.app
            .add_route(method, appended_route.leak(), middleware, req_fn)
            .await
    }

    // creates a get, post, put, and patch fn
    http_module_fn!(get, HttpMethod::GET);
    http_module_fn!(post, HttpMethod::POST);
    http_module_fn!(put, HttpMethod::PUT);
    http_module_fn!(patch, HttpMethod::PATCH);
    http_module_fn!(delete, HttpMethod::DELETE);
}

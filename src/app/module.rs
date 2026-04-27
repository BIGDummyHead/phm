use crate::{
    App, HttpMethod,
    app::Closed,
    router::RouterError,
    web::{ArcMiddlewareClosure, FutureClosureBound},
};

macro_rules! add_httpmethod {
    ($name:ident, $method:expr) => {
        /// Allows you to add a route with the corresponding `HttpMethod` fn based on it's name.
        ///
        /// ## Example
        ///
        /// ```rs
        /// app.get("/api/closed", vec![], |req, res| async move {Ok(())});
        /// ```
        ///
        /// ## Panics
        ///
        /// This function will panic if the route is invalid or already exists.
        pub async fn $name<F>(
            &self,
            route: impl Into<String>,
            middleware: Vec<ArcMiddlewareClosure>,
            request: F,
        ) -> ()
        where
            F: for<'f> FutureClosureBound<'f>,
        {
            Self::add_route(self, $method, route, middleware, request)
                .await
                .expect("route already exist or was invalid!");
        }
    };
}

pub struct Module<'a, 'app>
where
    'app: 'static,
{
    app: &'a App<'app, Closed>,
    base_rte: String,
}

impl<'a, 'app> Module<'a, 'app>
where
    'app: 'static,
{
    pub fn create(base_rte: impl Into<String>, app: &'a App<'app, Closed>) -> Module<'a, 'app> {
        Self {
            base_rte: base_rte.into(),
            app,
        }
    }

    fn combine_route(&self, add_route: impl Into<String>) -> &'static str {
        let mut add_route = add_route.into();

        if self.base_rte.ends_with('/') && add_route.starts_with('/') {
            add_route = add_route.chars().skip(1).collect();
        }

        let pref = if !add_route.starts_with('/') && !self.base_rte.ends_with('/') {
            "/"
        } else {
            ""
        };

        format!("{}{pref}{add_route}", self.base_rte).leak()
    }

    /// # Add Route
    ///
    /// Attempts to add the route to the router.
    ///
    /// ## Parameters
    ///
    /// `method`: The http method that is required.
    /// `route`: The full route to access the endpoint.
    /// `middleware`: A collection of `ArcMiddlewareClosure`, can be empty if none.
    /// `req_fn`: A closure that returns a future that falls under the `Fut` constraints.
    ///
    /// ## Example
    /// ```rs
    ///
    /// ```
    pub async fn add_route<F>(
        &self,
        method: HttpMethod,
        add_route: impl Into<String>,
        middleware: Vec<ArcMiddlewareClosure>,
        request: F,
    ) -> Result<(), RouterError>
    where
        F: for<'f> FutureClosureBound<'f>,
    {
        let route = self.combine_route(add_route);
        self.app.add_route(method, route, middleware, request).await
    }

    add_httpmethod!(get, HttpMethod::GET);
    add_httpmethod!(post, HttpMethod::POST);
    add_httpmethod!(patch, HttpMethod::PATCH);
    add_httpmethod!(put, HttpMethod::PUT);
    add_httpmethod!(delete, HttpMethod::DELETE);
}

use std::pin::Pin;

use async_trait::async_trait;

use crate::{
    App, HttpMethod, HttpRequest, RequestError, Response, app::Closed, router::RouterError,
    web::ArcMiddlewareClosure,
};

type RequestClosure = Box<
    dyn for<'a> Fn(
            &'a mut HttpRequest<'_>,
            &'a mut Response,
        ) -> Pin<Box<dyn Future<Output = Result<(), RequestError>> + Send + 'a>>
        + Send
        + Sync,
>;

/// # Route Definition
///
/// The route definition is a struct that stores information about a route that can be added to an [`App<'_, Closed>`].
pub struct RouteDefinition {
    route: &'static str,
    method: HttpMethod,
    middleware: Vec<ArcMiddlewareClosure>,
    req_fn: RequestClosure,
}

impl RouteDefinition {
    /// # New
    ///
    /// Create a new route definition with the following parameters:
    ///
    /// ## Parameters
    ///
    /// `route` : The route that the resource belongs to for example `/api/user/:user_id`
    /// `method` : The method that invokes the resource, example `GET`
    /// `middleware` : A collection of `ArcMiddlewareClosure`s see the `phm::middleware` function to create a middleware item. You may also leave a empty `vec![]` or `middleware!()`.
    /// `req_fn` : A request closure.
    pub fn new(
        route: &'static str,
        method: impl Into<HttpMethod>,
        middleware: Vec<ArcMiddlewareClosure>,
        req_fn: RequestClosure,
    ) -> Self {
        Self {
            route,
            method: method.into(),
            middleware,
            req_fn,
        }
    }

    /// # Add
    ///
    /// Consumes the route definition and inserts it into the given `App<'_, Closed>`
    pub async fn add(self, app: &App<'_, Closed>) -> Result<(), RouterError> {
        app.add_route(self.method, self.route, self.middleware, self.req_fn)
            .await
    }
}

#[async_trait]
pub trait ClosedAppExt {
    async fn add_def(&self, cls: impl Fn() -> RouteDefinition + Send) -> Result<(), RouterError>;
}

#[async_trait]
impl<'app> ClosedAppExt for App<'app, Closed> {
    async fn add_def(&self, cls: impl Fn() -> RouteDefinition + Send) -> Result<(), RouterError> {
        cls().add(self).await
    }
}

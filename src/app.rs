//! # App
//!
//! Defines the lifecycle typestate for an application. An [`App`] begins in
//! the [`Closed`] state (bound to a socket, routes may be added) and can be
//! transitioned into the [`Running`] state by calling
//! [`App::start`]. The running state continues accepting connections in a
//! background thread until [`Running::close`](running::Running) is awaited.

use std::sync::Arc;

use crate::{
    HttpMethod, HttpRequest, Response,
    router::{Router, RouterError},
    web::{ArcMiddlewareClosure, ArcRequestClosure, FutureClosureBound, http_request::Parsers},
};

mod module;
mod running;

pub use module::Module;
pub use running::Running;
use smol::{
    lock::RwLock,
    net::{AsyncToSocketAddrs, TcpListener},
};

/// Marker state indicating the application is bound to a socket but not yet
/// accepting connections. Routes and parser settings may be configured while
/// in this state.
pub struct Closed {
    http_parser: Option<Parsers>,
}

/// The top-level application, parameterised by its lifecycle state `T`
/// (either [`Closed`] or [`Running`]).
///
/// The `'app` lifetime scopes route strings and router internals; it is
/// required to be `'static` in practice so that the listener thread can
/// safely hold onto router data.
pub struct App<'app, T>
where
    'app: 'static,
{
    client: Arc<TcpListener>,
    state: T,
    router: Arc<RwLock<Router<'app>>>,
}

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
            route: &'static str,
            middleware: Vec<ArcMiddlewareClosure>,
            request: F,
        ) -> ()
        where
            F: for<'a> FutureClosureBound<'a>,
        {
            Self::add_route(self, $method, route, middleware, request)
                .await
                .expect("route already exist or was invalid!");
        }
    };
}

impl<'app> App<'app, Closed>
where
    'app: 'static,
{
    /// # Bind
    ///
    /// Binds a `TcpListener` to the given `SocketAddrs`.
    ///
    /// ## Returns
    ///
    /// This function will return a new instance of an `App` that is in a closed state if binding is successful.
    ///
    /// This function will return an error if the binding failed.
    ///
    /// ## Notes
    ///
    /// It is important to note that the `TcpListener` will not start accepting clients until `start` is called.
    pub async fn bind<A>(addr: A) -> Result<App<'app, Closed>, std::io::Error>
    where
        A: AsyncToSocketAddrs,
    {
        let bind_result = TcpListener::bind(addr).await?;

        Ok(App {
            client: Arc::new(bind_result),
            router: Arc::new(RwLock::new(Router::new())),
            state: Closed { http_parser: None },
        })
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
    /// `request`: A closure that returns a future that falls under the `Fut` constraints.
    ///
    /// ## Example
    /// ```rs
    ///
    /// ```
    pub async fn add_route<F>(
        &self,
        method: HttpMethod,
        route: &'static str,
        middleware: Vec<ArcMiddlewareClosure>,
        request: F,
    ) -> Result<(), RouterError>
    where
        F: for<'a> FutureClosureBound<'a>,
    {
        let request: ArcRequestClosure =
            Arc::new(move |req: &mut HttpRequest, res: &mut Response| request(req, res));
        self.router
            .write()
            .await
            .add_route(route, method, middleware, request)
            .await
    }

    /// # Set 404
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
    pub async fn set_404<F>(&self, request: F)
    where
        F: for<'a> FutureClosureBound<'a>,
    {
        let request: ArcRequestClosure =
            Arc::new(move |req: &mut HttpRequest, res: &mut Response| request(req, res));
        self.router.write().await.set_404(request).await;
    }

    add_httpmethod!(get, HttpMethod::GET);
    add_httpmethod!(post, HttpMethod::POST);
    add_httpmethod!(patch, HttpMethod::PATCH);
    add_httpmethod!(put, HttpMethod::PUT);
    add_httpmethod!(delete, HttpMethod::DELETE);

    /// # Module
    ///
    /// Create a new module that can be used to create routes with base paths.
    ///
    /// For example:
    ///
    /// ```rust
    /// let api_module = app.module("/api");
    ///
    /// api_module.get("/test", middleware!(), |req, res| Box::pin(async move{ Ok(()) })).await;
    /// ```
    pub fn module(&self, base_rte: impl Into<String>) -> Module<'_, 'app> {
        Module::create(base_rte, self)
    }

    /// Set the parser to use for parsing incoming HTTP Request.
    pub fn set_parser(&mut self, selected: Parsers) -> () {
        self.state.http_parser = Some(selected);
    }

    /// # Start
    ///
    /// Attempts to start the application.
    ///
    /// Returns a running instance of the application.
    pub fn start(self) -> App<'app, Running> {
        App::run(self)
    }
}

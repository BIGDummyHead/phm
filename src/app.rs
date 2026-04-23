use std::sync::Arc;

use tokio::{
    net::{TcpListener, ToSocketAddrs},
    sync::RwLock,
};

use crate::{
    HttpMethod, HttpRequest, Resolution,
    router::{Router, RouterError},
    web::{
        ArcMiddlewareClosure, ArcRequestClosure, RequestClosure, RequestFuture,
        http_request::Parsers,
    },
};

mod module;
mod running;

pub use module::Module;
pub use running::Running;

/// The app state is closed.
pub struct Closed {
    http_parser: Option<Parsers>,
}

pub struct App<'app, T>
where 'app : 'static {
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
        pub async fn $name<Cls, Fut>(
            &self,
            route: &'static str,
            middleware: Vec<ArcMiddlewareClosure>,
            req_fn: Cls,
        ) -> ()
        where
            Fut: RequestFuture + 'static,
            Cls: RequestClosure<Fut> + Send + Sync + 'static,
        {
            Self::add_route(self, $method, route, middleware, req_fn)
                .await
                .expect("route already exist or was invalid!");
        }
    };
}

impl<'app> App<'app, Closed>
where 'app : 'static {
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
        A: ToSocketAddrs,
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
    /// `req_fn`: A closure that returns a future that falls under the `Fut` constraints.
    ///
    /// ## Example
    /// ```rs
    ///
    /// ```
    pub async fn add_route<Cls, Fut>(
        &self,
        method: HttpMethod,
        route: &'static str,
        middleware: Vec<ArcMiddlewareClosure>,
        req_fn: Cls,
    ) -> Result<(), RouterError>
    where
        Fut: RequestFuture + 'static,
        Cls: RequestClosure<Fut> + Send + Sync + 'static,
    {
        let req_fn: ArcRequestClosure =
            Arc::new(move |req: &'static mut HttpRequest, res: &'static mut Resolution| Box::pin(req_fn(req, res)));
        self.router
            .write()
            .await
            .add_route(route, method, middleware, req_fn)
            .await
    }

    add_httpmethod!(get, HttpMethod::GET);
    add_httpmethod!(post, HttpMethod::POST);
    add_httpmethod!(patch, HttpMethod::PATCH);
    add_httpmethod!(put, HttpMethod::PUT);
    add_httpmethod!(delete, HttpMethod::DELETE);

    /// Set the parser to use for parsing incoming HTTP Request.
    pub fn set_parser(&mut self, selected: Parsers) -> () {
        self.state.http_parser = Some(selected);
    }

    /// # Module
    ///
    /// Creates a new module that with the given base route.
    ///
    /// ## Example
    ///
    /// ```rs
    /// let mut app = ...
    ///
    /// let mut module = app.module("/api");
    ///
    /// module.add_route(HttpMethod::GET, "/closed", vec![], |req, res| async move { Ok(()) });
    /// ```
    pub fn module(&'app self, base_route: &'app str) -> Module<'app> {
        Module::new(base_route, self)
    }

    /// # Start
    ///
    /// Attempts to start the application.
    ///
    /// Returns a running instance of the application.
    pub async fn start(self) -> App<'app, Running> {
        App::running(self).await
    }
}

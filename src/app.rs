use std::sync::Arc;

use tokio::net::{TcpListener, ToSocketAddrs};

use crate::{
    HttpMethod, Request, Resolution, router::{Router, RouterError}, web::{
        ArcMiddlewareClosure, ArcRequestClosure, PinnedRequestFuture, RequestClosure,
        RequestFnResult, RequestFuture, request::RequestError,
    }
};

/// The app state is running and can be closed.
pub struct Running;

/// The app state is closed.
pub struct Closed;
pub struct App<'app, T> {
    client: TcpListener,
    phantom: std::marker::PhantomData<&'app T>,
    router: Router<'app>
}

impl<'app> App<'app, Closed> {
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

        Ok(App::<Closed> {
            client: bind_result,
            router: Router::new(),
            phantom: std::marker::PhantomData,
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
        &mut self,
        method: HttpMethod,
        route: &'static str,
        middleware: Vec<ArcMiddlewareClosure>,
        req_fn: Cls
    ) -> Result<(), RouterError>
    where 
    Fut : RequestFuture + 'static,
    Cls : RequestClosure<Fut> + 'static {
        let req_fn: ArcRequestClosure = Arc::new(move |req: &mut Request, res: &mut Resolution| Box::pin(req_fn(req, res)));
        self.router.add_route(route, method, middleware, req_fn).await
    }

    pub fn get_router(&self) -> &Router<'app> {
        &self.router
    }
}

//! # Web
//!
//! HTTP protocol layer for the crate: request and response types, status
//! codes, HTTP methods, middleware machinery, and the trait bounds /
//! type-aliases that make async closures usable as handlers.

pub mod file_body;
mod http_method;
pub mod http_request;
mod middleware;
mod response;

use std::{pin::Pin, sync::Arc};

pub use http_method::HttpMethod;
pub use http_request::HttpRequest;
pub use middleware::{Middleware, middleware};
pub use response::*;

pub use crate::web::http_request::RequestError;

/// The type that is returned by all Request futures.
pub type RequestFnResult = Result<(), RequestError>;

/// The pinned, boxed, `Send` future produced by a request handler closure.
/// Requests resolve to a [`RequestFnResult`].
pub type PinnedRequestFuture<'a> = Pin<Box<dyn Future<Output = RequestFnResult> + Send + 'a>>;

/// An `Arc`-wrapped, dyn-compatible request handler closure that takes a
/// mutable reference to the [`HttpRequest`] and [`Response`] and returns a
/// [`PinnedRequestFuture`]. This is the stored form of every handler in
/// the router.
pub type ArcRequestClosure = Arc<
    dyn for<'a> Fn(&'a mut HttpRequest, &'a mut Response) -> PinnedRequestFuture<'a> + Send + Sync,
>;

/// Used for the app routing
///
/// Ensures that the passed in dyn fn (closure) has the parameters (req, res) and return a boxed future that is Send and lives for at least as long as the variables itself.
pub trait FutureClosureBound<'a>:
    Fn(
        &'a mut HttpRequest,
        &'a mut Response,
    ) -> Pin<Box<dyn Future<Output = RequestFnResult> + Send + 'a>>
    + Send
    + Sync
    + 'static
{
}

// default implementation so that closures can be used.
impl<'a, T> FutureClosureBound<'a> for T where
    T: Fn(
            &'a mut HttpRequest,
            &'a mut Response,
        ) -> Pin<Box<dyn Future<Output = RequestFnResult> + Send + 'a>>
        + Send
        + Sync
        + 'static
{
}

/// The pinned, boxed, `Send` future produced by a middleware closure.
/// Resolves to a [`Middleware`] value that tells the router whether to
/// continue down the chain or stop and respond early.
pub type PinnedMiddlewareFuture<'a> = Pin<Box<dyn Future<Output = Middleware> + Send + 'a>>;

/// An `Arc`-wrapped, dyn-compatible middleware closure. Shaped like
/// [`ArcRequestClosure`] but resolves to a [`Middleware`] control value
/// instead of a [`RequestFnResult`].
pub type ArcMiddlewareClosure = Arc<
    dyn for<'a> Fn(&'a mut HttpRequest, &'a mut Response) -> PinnedMiddlewareFuture<'a>
        + Send
        + Sync,
>;

/// Trait bound for middleware closures. Mirrors
/// [`FutureClosureBound`] but its associated future resolves to a
/// [`Middleware`] control value. A blanket impl is provided for any closure
/// that already satisfies the required `Fn` signature.
pub trait FutureMiddlewareBound<'a>:
    Fn(
        &'a mut HttpRequest,
        &'a mut Response,
    ) -> Pin<Box<dyn Future<Output = Middleware> + Send + 'a>>
    + Send
    + Sync
    + 'static
{
}

// default implementation so that closures can be used for middleware.
impl<'a, T> FutureMiddlewareBound<'a> for T where
    T: Fn(
            &'a mut HttpRequest,
            &'a mut Response,
        ) -> Pin<Box<dyn Future<Output = Middleware> + Send + 'a>>
        + Send
        + Sync
        + 'static
{
}

#[macro_export]
macro_rules! middleware {
    () => {
        ::std::vec::Vec::new()
    };
    ($( $items:ident ),* ) => {{
        let mut collection: ::std::vec::Vec<$crate::web::ArcMiddlewareClosure> =
            ::std::vec::Vec::new();

        $( collection.push($items.clone()); )*

        collection
    }};
}

#[macro_export]
macro_rules! request {
    (|$req:ident, $res:ident| $body:block) => {
        |$req: &mut $crate::HttpRequest<'_>, $res: &mut $crate::Response| { ::std::boxed::Box::pin(async $body) }
    };

    (move |$req:ident, $res:ident| $body:block) => {
        move |$req: &mut $crate::HttpRequest<'_>, $res: &mut $crate::Response| { ::std::boxed::Box::pin(async move $body) }
    };
}

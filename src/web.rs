mod http_method;
pub mod http_request;
mod middleware;
mod resolution;

use std::{pin::Pin, sync::Arc};

pub use http_method::HttpMethod;
pub use http_request::HttpRequest;
pub use middleware::{Middleware, middleware};
pub use resolution::Resolution;

pub use crate::web::http_request::RequestError;

/// The type that is returned by all Request futures.
pub type RequestFnResult = Result<(), RequestError>;

/// A trait that represents a closure that returns the future of a request.
pub trait RequestClosure<Fut>: Fn(&'static mut HttpRequest, &'static mut Resolution) -> Fut
where
    Fut: RequestFuture,
{
}
impl<Fut, T> RequestClosure<Fut> for T
where
    Fut: RequestFuture,
    T: Fn(&'static mut HttpRequest, &'static mut Resolution) -> Fut,
{
}

pub trait MiddlewareClosure<Fut>: Fn(&'static mut HttpRequest, &'static mut Resolution) -> Fut
where
    Fut: MiddlewareFuture,
{
}
impl<Fut, T> MiddlewareClosure<Fut> for T
where
    Fut: MiddlewareFuture,
    T: Fn(&mut HttpRequest, &mut Resolution) -> Fut,
{
}

/// A trait in which the Future's output is a `RequestFnResult`.
///
/// This is implemented for all T where the `Future` output is `RequestFnResult`
pub trait RequestFuture: Future<Output = RequestFnResult> + Send {}
impl<T> RequestFuture for T where T: Future<Output = RequestFnResult> + Send {}

pub trait MiddlewareFuture: Future<Output = Middleware> + Send {}
impl<T> MiddlewareFuture for T where T: Future<Output = Middleware> + Send {}

pub type PinnedRequestFuture = Pin<Box<dyn RequestFuture + Send>>;
pub type PinnedMiddlewareFuture = Pin<Box<dyn MiddlewareFuture + Send>>;

pub type ArcRequestClosure =
    Arc<dyn Fn(&mut HttpRequest, &mut Resolution) -> PinnedRequestFuture + Send + Sync>;

pub type ArcMiddlewareClosure =
    Arc<dyn Fn(&mut HttpRequest, &mut Resolution) -> PinnedMiddlewareFuture + Send + Sync>;

mod http_method;
mod middleware;
pub mod http_request;
mod resolution;

use std::{pin::Pin, sync::Arc};

pub use http_method::HttpMethod;
pub use middleware::{Middleware, middleware};
pub use http_request::HttpRequest;
pub use resolution::Resolution;

pub use crate::web::http_request::RequestError;

/// The type that is returned by all Request futures.
pub type RequestFnResult = Result<(), RequestError>;

/// A trait in which the Future's output is a `RequestFnResult`.
///
/// This is implemented for all T where the `Future` output is `RequestFnResult`
pub trait RequestFuture: Future<Output = RequestFnResult> {}
impl<T> RequestFuture for T where T: Future<Output = RequestFnResult> {}

pub trait MiddlewareFuture: Future<Output = Middleware> {}
impl<T> MiddlewareFuture for T where T: Future<Output = Middleware> {}

/// A trait that represents a closure that returns the future of a request.
pub trait RequestClosure<Fut>: Fn(&mut HttpRequest, &mut Resolution) -> Fut
where
    Fut: RequestFuture,
{
}
impl<Fut, T> RequestClosure<Fut> for T
where
    Fut: RequestFuture,
    T: Fn(&mut HttpRequest, &mut Resolution) -> Fut,
{
}

pub trait MiddlewareClosure<Fut>: Fn(&mut HttpRequest, &mut Resolution) -> Fut
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

pub type PinnedRequestFuture = Pin<Box<dyn RequestFuture>>;
pub type PinnedMiddlewareFuture = Pin<Box<dyn MiddlewareFuture>>;

/// An Atomic Reference Counter that captures a request closure in where the request closure returns a Pinned Request Future.
pub type ArcRequestClosure = Arc<dyn Fn(&mut HttpRequest, &mut Resolution) -> PinnedRequestFuture>;

/// An Atomic Reference Counter that captures a request closure in where the request closure returns a Pinned Middleware Future.
pub type ArcMiddlewareClosure =
    Arc<dyn Fn(&mut HttpRequest, &mut Resolution) -> PinnedMiddlewareFuture>;

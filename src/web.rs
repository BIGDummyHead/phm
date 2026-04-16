mod http_method;
mod middleware;
pub mod request;
mod resolution;

use std::{pin::Pin, sync::Arc};

pub use http_method::HttpMethod;
pub use middleware::{Middleware, middleware};
pub use request::Request;
pub use resolution::Resolution;

pub use crate::web::request::RequestError;

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
pub trait RequestClosure<Fut>: Fn(&mut Request, &mut Resolution) -> Fut
where
    Fut: RequestFuture,
{
}
impl<Fut, T> RequestClosure<Fut> for T
where
    Fut: RequestFuture,
    T: Fn(&mut Request, &mut Resolution) -> Fut,
{
}

pub trait MiddlewareClosure<Fut>: Fn(&mut Request, &mut Resolution) -> Fut
where
    Fut: MiddlewareFuture,
{
}
impl<Fut, T> MiddlewareClosure<Fut> for T
where
    Fut: MiddlewareFuture,
    T: Fn(&mut Request, &mut Resolution) -> Fut,
{
}

pub type PinnedRequestFuture = Pin<Box<dyn RequestFuture>>;
pub type PinnedMiddlewareFuture = Pin<Box<dyn MiddlewareFuture>>;

/// An Atomic Reference Counter that captures a request closure in where the request closure returns a Pinned Request Future.
pub type ArcRequestClosure = Arc<dyn Fn(&mut Request, &mut Resolution) -> PinnedRequestFuture>;

/// An Atomic Reference Counter that captures a request closure in where the request closure returns a Pinned Middleware Future.
pub type ArcMiddlewareClosure =
    Arc<dyn Fn(&mut Request, &mut Resolution) -> PinnedMiddlewareFuture>;

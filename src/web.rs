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

pub trait MiddlewareClosure<Fut>: Fn(&mut HttpRequest, &mut Response) -> Fut
where
    Fut: MiddlewareFuture,
{
}
impl<Fut, T> MiddlewareClosure<Fut> for T
where
    Fut: MiddlewareFuture,
    T: Fn(&mut HttpRequest, &mut Response) -> Fut,
{
}

pub trait MiddlewareFuture: Future<Output = Middleware> {}
impl<T> MiddlewareFuture for T where T: Future<Output = Middleware> {}

pub type PinnedMiddlewareFuture = Pin<Box<dyn MiddlewareFuture + Send + Sync>>;

pub type ArcMiddlewareClosure =
    Arc<dyn Fn(&mut HttpRequest, &mut Response) -> PinnedMiddlewareFuture + Send + Sync>;

/// A trait that represents a closure that returns the future of a request.
pub trait RequestClosure<Fut>: Fn(&mut HttpRequest, &mut Response) -> Fut
where
    Fut: RequestFuture,
{
}
impl<Fut, T> RequestClosure<Fut> for T
where
    Fut: RequestFuture,
    T: Fn(&mut HttpRequest, &mut Response) -> Fut,
{
}

/// A trait in which the Future's output is a `RequestFnResult`.
///
/// This is implemented for all T where the `Future` output is `RequestFnResult`
pub trait RequestFuture: Future<Output = RequestFnResult> + Send + Sync {}
impl<T> RequestFuture for T where T: Future<Output = RequestFnResult> + Send + Sync {}

pub type PinnedRequestFuture<'a> = Pin<Box<dyn Future<Output = Result<(), RequestError>> + Send + Sync + 'a>>;
pub type ArcRequestClosure = Arc<
    dyn for<'a> Fn(&'a mut HttpRequest, &'a mut Response) -> PinnedRequestFuture<'a>
        + Send
        + Sync,
>;

pub trait RequestBound<'a>:
    Fn(
        &'a mut HttpRequest,
        &'a mut Response,
    ) -> Pin<Box<dyn Future<Output = Result<(), RequestError>> + Send + Sync + 'a>>
    + Send
    + Sync
    + 'static
{
}

impl<'a, T: Fn(&'a mut HttpRequest, &'a mut Response) -> PinnedRequestFuture<'a> + Send + Sync>
    RequestBound<'a> for T
where
    T: 'static,
{
}
use std::sync::Arc;

use crate::{
    Request, Resolution,
    web::{ArcMiddlewareClosure, MiddlewareClosure, MiddlewareFuture},
};

/// # Middleware
///
/// An enum that describes the action the router should take when encountered.
pub enum Middleware {
    /// Stop
    ///
    /// Notifies the router to STOP and return the information thus far.
    Stop,
    /// Next
    ///
    /// Notifies the router to continue onward.
    Next,
}

/// TODO: DOCUMENT
pub fn middleware<'req, Fut, Cls>(m: Cls) -> ArcMiddlewareClosure
where
    Fut: MiddlewareFuture + 'static,
    Cls: for<'a> MiddlewareClosure<'a, Fut> + 'static,
{
    Arc::new(
        move |req: &mut Request, res: &mut Resolution| 
        Box::pin(
            m(req, res)
        )
    )
}

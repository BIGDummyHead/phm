//! # Middleware
//!
//! Defines the [`Middleware`] control enum returned from middleware futures
//! and the [`middleware`] factory used to wrap an async closure into an
//! [`ArcMiddlewareClosure`] suitable for attaching to a route.

use std::sync::Arc;

use crate::{
    HttpRequest, Response,
    web::{ArcMiddlewareClosure, FutureMiddlewareBound},
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

/// # middleware
///
/// Wraps an async closure into an [`ArcMiddlewareClosure`] so it can be
/// stored in the router and executed before the request handler.
///
/// The supplied closure receives mutable references to the current
/// [`HttpRequest`] and [`Response`] and must resolve to a [`Middleware`]
/// value indicating whether the router should proceed to the next layer or
/// short-circuit.
pub fn middleware<F>(m_fn: F) -> ArcMiddlewareClosure
where
    F: for<'a> FutureMiddlewareBound<'a>,
{
    let mid: ArcMiddlewareClosure =
        Arc::new(move |req: &mut HttpRequest, res: &mut Response| m_fn(req, res));

    mid
}

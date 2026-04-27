//! # Request Error
//!
//! Error type that request handlers may return from their futures to cause
//! the framework to respond with a specific status code and optional
//! message instead of the handler's own response body.

use std::{pin::Pin};

use crate::Response;

/// # RequestError
///
/// Failure produced by a request handler. Carries a handler function that is invoked on request errors.
/// 
/// 
pub struct RequestError {
    handler: Pin<Box<dyn Fn(&mut Response) -> () + Send + 'static>>,
}

impl RequestError {
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(&mut Response) -> () + Send + 'static,
    {
        let handler = Box::pin(handler);
        Self { handler }
    }

    pub fn handle(&self, response: &mut Response) -> () {
        (*self.handler)(response);
    }
}
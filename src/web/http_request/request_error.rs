//! # Request Error
//!
//! Error type that request handlers may return from their futures to cause
//! the framework to respond with a specific status code and optional
//! message instead of the handler's own response body.

use crate::web::http_code::HttpCode;

/// # RequestError
///
/// Failure produced by a request handler. Carries an [`HttpCode`] that will
/// be applied to the outgoing [`Response`](crate::Response) and an optional
/// message that, if present, is written into the response body.
#[derive(Debug)]
pub struct RequestError {
    status: HttpCode,
    message: Option<String>,
}

/// Produces a [`RequestError`] whose status defaults to
/// [`HttpCode::InternalServerError`] with no message.
impl Default for RequestError {
    fn default() -> Self {
        Self {
            status: HttpCode::InternalServerError,
            message: None,
        }
    }
}

impl RequestError {
    /// Set the status of the request error
    pub fn set_status(&mut self, status: impl Into<HttpCode>) -> () {
        self.status = status.into();
    }

    /// Set the reason that the request failed
    pub fn set_message(&mut self, msg: impl Into<String>) -> () {
        self.message = Some(msg.into());
    }

    /// The status code that will be applied to the outgoing response.
    pub fn status(&self) -> &HttpCode {
        &self.status
    }

    /// The optional message, if one was set by [`Self::set_message`].
    pub fn message(&self) -> Option<&String> {
        self.message.as_ref()
    }
}

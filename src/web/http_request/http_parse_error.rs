//! # HTTP Parse Error
//!
//! Error type produced by any [`HttpRequestMetaParser`](crate::web::http_request::HttpRequestMetaParser)
//! implementation while parsing the request line, headers, or body.

use thiserror::Error;

use crate::router::RouterError;

/// # HttpParseError
///
/// Enumerates the failure modes encountered while parsing an incoming
/// HTTP request from a TCP stream.
#[derive(Debug, Error)]
pub enum HttpParseError {
    #[error("meta like the method, route, or host was missing")]
    MissingRequiredMeta,
    #[error("invalid header encoutered")]
    InvalidHeader,
    #[error("invalid content length value inserted into header")]
    InvalidContentLength,
    #[error("{0}")]
    IO(std::io::Error),
    #[error("invalid buffer size")]
    InvalidBufferSize,
    #[error("the type of body read {0} is not supported")]
    UnsupportedBody(String),
    #[error("when parsing the route, the router encountered an error: {0}")]
    RouterEncounteredError(RouterError),
}

/// Wraps any [`std::io::Error`] encountered while reading the stream into
/// [`HttpParseError::IO`].
impl From<std::io::Error> for HttpParseError {
    fn from(value: std::io::Error) -> Self {
        HttpParseError::IO(value)
    }
}

/// Wraps any [`RouterError`] raised during the route-resolution step of
/// parsing into [`HttpParseError::RouterEncounteredError`].
impl From<RouterError> for HttpParseError {
    fn from(value: RouterError) -> Self {
        match value {
            _ => HttpParseError::RouterEncounteredError(value),
        }
    }
}

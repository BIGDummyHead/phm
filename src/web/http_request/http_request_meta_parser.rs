//! # HTTP Request Meta Parser
//!
//! Trait contract that every protocol-specific parser (such as
//! [`HttpParserV1`](crate::web::http_request::http_parser_v1::HttpParserV1))
//! must implement. Allows additional HTTP versions to be plugged into the
//! crate via the [`Parsers`](crate::web::http_request::Parsers) enum.

use async_trait::async_trait;
use smol::net::TcpStream;

use crate::web::http_request::{HttpRequestMeta, http_parse_error::HttpParseError};

/// # HttpRequestMetaParser
///
/// Abstraction over the wire-format-specific logic for reading a request
/// off of a [`TcpStream`]. Implementations must be `Default` so that an
/// instance can be produced without configuration by the router pipeline.
#[async_trait]
pub trait HttpRequestMetaParser: Default {

    /// # Parse
    /// 
    /// Parses the given client's stream and creates an http request meta struct.
    /// 
    /// If parsing fails then a `HttpParseError` is given in return.
    async fn parse<'req>(stream: &'req mut TcpStream) -> Result<HttpRequestMeta, HttpParseError>;
}


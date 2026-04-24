//! # Parsers
//!
//! Thin dispatcher enum selecting which concrete
//! [`HttpRequestMetaParser`] implementation will be used to parse an
//! incoming request. Configured on the [`App`](crate::App) before it is
//! started.

use smol::net::TcpStream;

use crate::web::http_request::{HttpRequestMeta, HttpRequestMetaParser, http_parse_error::HttpParseError, http_parser_v1::HttpParserV1};

/// HTTP Protocol Parsers
#[derive(Default, Clone, Debug)]
pub enum Parsers {

    /// The HTTP/1.1 parser backed by
    /// [`HttpParserV1`](crate::web::http_request::http_parser_v1::HttpParserV1).
    /// This is the default parser used when no other is configured.
    #[default]
    HttpV1
}

impl Parsers {

    /// # Get Parser
    /// 
    /// Retrieves the underlying parser struct that implements the code to parse a request's metadata.
    pub async fn parse<'req>(&self, stream: &'req mut TcpStream) -> Result<HttpRequestMeta, HttpParseError> {
        match self {
            Parsers::HttpV1 => {
                HttpParserV1::parse(stream).await
            },
        }
    }
}
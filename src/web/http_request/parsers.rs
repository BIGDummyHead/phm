use tokio::net::TcpStream;

use crate::web::http_request::{HttpRequestMeta, HttpRequestMetaParser, http_parse_error::HttpParseError, http_parser_v1::HttpParserV1};

/// HTTP Protocol Parsers
#[derive(Default)]
pub enum Parsers {

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
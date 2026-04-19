use async_trait::async_trait;
use tokio::net::TcpStream;

use crate::web::http_request::{HttpRequestMeta, http_parse_error::HttpParseError};

#[async_trait]
pub trait HttpRequestMetaParser: Default {

    /// # Parse
    /// 
    /// Parses the given client's stream and creates an http request meta struct.
    /// 
    /// If parsing fails then a `HttpParseError` is given in return.
    async fn parse<'req>(stream: &'req mut TcpStream) -> Result<HttpRequestMeta, HttpParseError>;
}


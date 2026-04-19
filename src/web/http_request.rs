mod http_parse_error;
mod http_request_meta_parser;
mod http_parser_v1;
mod http_request_meta;
mod parsers;
mod request_error;
mod variables;

use std::net::SocketAddr;

pub use http_request_meta::HttpRequestMeta;
pub use request_error::RequestError;
pub use variables::Variables;
pub use parsers::Parsers;

use tokio::net::TcpStream;

pub use http_request_meta_parser::HttpRequestMetaParser;

use crate::{router::Router, web::http_request::http_parse_error::HttpParseError};

/// request placeholder, holds pertinent information about the ongoign request, things related to the request should
/// encapsulate the same lifetimes as this object.
pub struct HttpRequest<'req> {
    socket: SocketAddr,
    stream: TcpStream,
    params: Variables<'req>,
    meta: HttpRequestMeta,
}

impl<'req> HttpRequest<'req> {
    pub async fn parse<P>(router: &Router<'_>, mut stream: TcpStream, socket: SocketAddr, parser: Parsers) -> Result<Self, HttpParseError>
    where
        P: HttpRequestMetaParser,
    {
        // params are created within the parse.
        let meta = parser.parse(&mut stream).await?;

        let route = router.get_route(meta.clean_route(), meta.method().clone()).await?;

        todo!()
    }

    /// # Mut Variables
    ///
    /// Borrows the variables as Mutatable.
    pub fn mut_variables(&mut self) -> &mut Variables<'req> {
        &mut self.params
    }

    /// # Variables
    ///
    /// Immuatable variables that are stored.
    pub fn variables(&self) -> &Variables<'req> {
        &self.params
    }
}

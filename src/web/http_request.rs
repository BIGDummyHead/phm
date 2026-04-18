mod http_parser;
mod http_parser_v1;
mod http_request_meta;
mod request_error;
mod variables;
mod http_parse_error;

use std::net::SocketAddr;

pub use http_request_meta::HttpRequestMeta;
pub use request_error::RequestError;
pub use variables::Variables;

use tokio::net::TcpStream;

pub use http_parser::HttpRequestMetaParser;

/// request placeholder, holds pertinent information about the ongoign request, things related to the request should
/// encapsulate the same lifetimes as this object.
pub struct HttpRequest<'req> {
    socket: SocketAddr,
    stream: TcpStream,
    params: Variables<'req>,
    meta: HttpRequestMeta<'req>,
}

impl<'req> HttpRequest<'req> {
    pub fn parse<P>(stream: TcpStream, socket: SocketAddr) -> Self
    where
        P: HttpRequestMetaParser,
    {
        // params are created within the parse.
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

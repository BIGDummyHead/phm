mod http_parse_error;
mod http_parser_v1;
mod http_request_meta;
mod http_request_meta_parser;
mod parsers;
mod request_error;
mod variables;

use std::{collections::HashMap, net::SocketAddr, sync::Arc};

pub use http_request_meta::HttpRequestMeta;
pub use parsers::Parsers;
pub use request_error::RequestError;
use smol::{lock::RwLock, net::TcpStream};
pub use variables::Variables;

pub use http_request_meta_parser::HttpRequestMetaParser;

use crate::{
    router::{Node, Router},
    web::http_request::http_parse_error::HttpParseError,
};

/// request placeholder, holds pertinent information about the ongoign request, things related to the request should
/// encapsulate the same lifetimes as this object.
pub struct HttpRequest<'app>
where 'app : 'static {
    socket: SocketAddr,
    stream: Arc<RwLock<TcpStream>>,
    variables: Variables,
    meta: HttpRequestMeta,
    node: Arc<RwLock<Node<'app>>>,
}

unsafe impl<'app> Send for HttpRequest<'app> {}
unsafe impl<'app> Sync for HttpRequest<'app> {}

macro_rules! immut_mut_var {
    ($get_name:ident, $get_mut_name:ident, $field_name:ident, $ty:ty) => {
        pub fn $get_name(&self) -> &$ty {
            &self.$field_name
        }

        pub fn $get_mut_name(&mut self) -> &mut $ty {
            &mut self.$field_name
        }
    };
}

macro_rules! immut_var {
    ($get_name:ident, $field_name:ident, $ty:ty) => {
        pub fn $get_name(&self) -> &$ty {
            &self.$field_name
        }
    };
}

impl<'app> HttpRequest<'app>
where 'app : 'static {

    /// # Parse
    /// 
    /// Parses the TcpStream and creates an HttpRequest.
    pub async fn parse(
        parser: &Parsers,
        router: &Router<'app>,
        stream: Arc<RwLock<TcpStream>>,
        socket: SocketAddr,
    ) -> Result<Self, HttpParseError>
    {

        // params are created within the parse.
        let meta = {
            let mut stream_guard = stream.write().await;
            parser.parse(&mut stream_guard).await?
        };

        let mut route_variables = HashMap::new();
        let route = router
            .get_route(
                meta.clean_route(),
                meta.method().clone(),
                &mut route_variables,
            )
            .await?;

        let vars = Variables::new(route_variables);

        Ok(Self {
            socket,
            stream,
            variables: vars,
            meta,
            node: route,
        })
    }

    immut_var!(socket, socket, SocketAddr);
    immut_var!(stream, stream, Arc<RwLock<TcpStream>>);
    immut_var!(node, node, Arc<RwLock<Node<'app>>>);


    immut_mut_var!(variables, variables_mut, variables, Variables);
    immut_mut_var!(meta, meta_mut, meta, HttpRequestMeta);

}

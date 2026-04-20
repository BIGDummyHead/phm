use std::sync::Arc;

use crate::{HttpMethod, web::http_request::http_parser_v1::HeaderMap};

/// Immuatable data about an incoming request from a client.
pub struct HttpRequestMeta {
    route: String,
    method: HttpMethod,
    headers: HeaderMap,
    body: Arc<[u8]>
}

impl HttpRequestMeta {

    /// Creates a new instance of an httpe request.
    pub fn new(route: String, method: HttpMethod, headers: HeaderMap, body: Arc<[u8]>) -> Self {
        Self {
            route,
            method,
            headers,
            body
        }
    }

    // splits the route into two parts. The cleaned route and parameters
    fn split_route(&self) -> (&str, Option<&str>) {
        let spl = self.full_route().rsplit_once("?");

        match spl {
            Some((a, b)) => (a, Some(b)),
            None => (self.full_route(), None),
        }
    }

    /// # Clean Route
    ///
    /// Returns the route slice as cleaned (meaning missing the ending parameters)
    ///
    /// `/api/user?name=Shawn` -> `/api/user`
    pub fn clean_route(&self) -> &str {
        self.split_route().0 //always choose 0
    }

    /// The parameters of the route
    pub fn route_params(&self) -> Option<&str> {
        self.split_route().1
    }

    /// The full route.
    pub fn full_route(&self) -> &str {
        &self.route
    }

    /// The method used in the request.
    pub fn method(&self) -> &HttpMethod {
        &self.method
    }

    /// Clones the body and returns an Arc reference
    pub fn body(&self) -> Arc<[u8]> {
        self.body.clone()
    }

    /// A reference to the body
    pub fn body_ref(&self) -> &Arc<[u8]> {
        &self.body
    }

    /// The headers of the request.
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }
}

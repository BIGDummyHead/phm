use std::sync::Arc;

use crate::{HttpMethod, web::http_request::http_parser_v1::HeaderMap};

pub struct HttpRequestMeta {
    route: String,
    method: HttpMethod,
    headers: HeaderMap,
    body: Arc<[u8]>
}

impl HttpRequestMeta {

    pub fn new(route: String, method: HttpMethod, headers: HeaderMap, body: Arc<[u8]>) -> Self {
        Self {
            route,
            method,
            headers,
            body
        }
    }

    fn split_route(&self) -> (&str, Option<&str>) {
        let spl = self.dirty_route().rsplit_once("?");

        match spl {
            Some((a, b)) => (a, Some(b)),
            None => (self.dirty_route(), None),
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

    pub fn dirty_route(&self) -> &str {
        &self.route
    }

    pub fn method(&self) -> &HttpMethod {
        &self.method
    }

    pub fn body(&self) -> Arc<[u8]> {
        self.body.clone()
    }

    pub fn body_ref(&self) -> &Arc<[u8]> {
        &self.body
    }
}

//! # HTTP Method
//!
//! Defines [`HttpMethod`], the enum used throughout the crate to identify
//! the verb of an incoming request or of a registered route.

/// # Http Method
///
/// HTTP defines a set of request methods to indicate the purpose of the request and what is expected if the request is successful. Although they can also be nouns, these request methods are sometimes referred to as HTTP verbs. Each request method has its own semantics, but some characteristics are shared across multiple methods, specifically request methods can be safe, idempotent, or cacheable.
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum HttpMethod {
    /// The GET method requests a representation of the specified resource. Requests using GET should only retrieve data and should not contain a request content.
    GET,
    /// The POST method submits an entity to the specified resource, often causing a change in state or side effects on the server.
    POST,
    /// The PATCH method applies partial modifications to a resource.
    PATCH,
    /// The PUT method replaces all current representations of the target resource with the request content.
    PUT,
    /// The DELETE method deletes the specified resource.
    DELETE,
    /// The HEAD method asks for a response identical to a GET request, but without a response body.
    HEAD,
    /// The OPTIONS method describes the communication options for the target resource.
    OPTIONS,
    /// The CONNECT method establishes a tunnel to the server identified by the target resource.
    CONNECT,
    /// The TRACE method performs a message loop-back test along the path to the target resource.
    TRACE,
    /// This request header is not apart of the standard HTTP request headers.
    Unknown(String),
}

impl<T> From<T> for HttpMethod
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        let unknown_str = value.into().trim().to_uppercase();

        match unknown_str.as_ref() {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PATCH" => HttpMethod::PATCH,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            "HEAD" => HttpMethod::HEAD,
            "OPTIONS" => HttpMethod::OPTIONS,
            "CONNECT" => HttpMethod::CONNECT,
            "TRACE" => HttpMethod::TRACE,
            _ => HttpMethod::Unknown(unknown_str),
        }
    }
}

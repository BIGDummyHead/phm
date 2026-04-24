//! # HTTP Code
//!
//! Enumerates every standard HTTP status code and provides conversions to
//! and from its numeric representation and its reason phrase.

/// # Http Code
///
/// Describes a status that is served with HTTP request.
///
/// Can be transformed into the code (`u32`), friendly header name such as `"OK"`, or as a header like `HTTP/1.1 200 OK`.
///
/// ## Example
///
/// ```
/// use async_web::web::resolution::http_code::HttpCode;
/// let http_code = HttpCode::OK;
///
/// let code: u32  = http_code.as_status_code(); // 200
///
/// let status: String = http_code.as_status(); // "OK"
///
/// let header: String = http_code.as_header(); // HTTP/1.1 200 OK
///
/// // we can also make an HttpCode enum from an u32
/// let random_code: HttpCode = 200.into();
/// ```
///
/// ## Notable implementations
///
/// `Copy`, `Clone`, `Resolution`, `Into<HttpCode> for u32`
#[derive(Copy, Clone, Debug, Default)]
pub enum HttpCode {
    /// # Continue
    ///
    /// The HTTP 100 Continue informational response status code indicates that the initial part of a request has been received and has not yet been rejected by the server.
    Continue,

    /// # Switching Protocols
    ///
    /// The HTTP 101 Switching Protocols response status code indicates the server is switching protocols as requested by the client.
    SwitchingProtocols,

    /// # Processing
    ///
    /// The HTTP 102 Processing status code indicates that the server has received and is processing the request, but no response is available yet.
    Processing,

    /// # Early Hints
    ///
    /// The HTTP 103 Early Hints status code is used to return some response headers before final HTTP message.
    EarlyHints,

    /// # OK
    ///
    /// The HTTP 200 OK success status code indicates that the request has succeeded.
    #[default]
    OK,

    /// # Created
    ///
    /// The HTTP 201 Created status code indicates that the request has succeeded and a new resource has been created.
    Created,

    /// # Accepted
    ///
    /// The HTTP 202 Accepted status code indicates that the request has been accepted for processing, but processing is not complete.
    Accepted,

    /// # Non-Authoritative Information
    ///
    /// The HTTP 203 status indicates that the returned metadata is not exactly the same as available from the origin server.
    NonAuthoritativeInformation,

    /// # No Content
    ///
    /// The HTTP 204 No Content status code indicates that the request succeeded but returns no content.
    NoContent,

    /// # Reset Content
    ///
    /// The HTTP 205 Reset Content status code indicates that the client should reset the document view.
    ResetContent,

    /// # Partial Content
    ///
    /// The HTTP 206 Partial Content status code indicates that the server is delivering only part of the resource.
    PartialContent,

    /// # Multi-Status
    ///
    /// The HTTP 207 Multi-Status status code conveys information about multiple resources.
    MultiStatus,

    /// # Already Reported
    ///
    /// The HTTP 208 Already Reported status code indicates that members of a DAV binding have already been enumerated.
    AlreadyReported,

    /// # IM Used
    ///
    /// The HTTP 226 IM Used status code indicates that the server has fulfilled a request for the resource.
    ImUsed,

    /// # Multiple Choices
    ///
    /// The HTTP 300 Multiple Choices status code indicates multiple options for the resource.
    MultipleChoices,

    /// # Moved Permanently
    ///
    /// The HTTP 301 Moved Permanently status code indicates that the resource has been permanently moved.
    MovedPermanently,

    /// # Found
    ///
    /// The HTTP 302 Found status code indicates that the resource resides temporarily under a different URI.
    Found,

    /// # See Other
    ///
    /// The HTTP 303 See Other status code indicates that the response can be found under another URI.
    SeeOther,

    /// # Not Modified
    ///
    /// The HTTP 304 Not Modified status code indicates that there is no need to retransmit the requested resources.
    NotModified,

    /// # Use Proxy
    ///
    /// The HTTP 305 Use Proxy status code indicates that the resource must be accessed through a proxy.
    UseProxy,

    /// # Temporary Redirect
    ///
    /// The HTTP 307 Temporary Redirect status code indicates that the resource resides temporarily under a different URI.
    TemporaryRedirect,

    /// # Permanent Redirect
    ///
    /// The HTTP 308 Permanent Redirect status code indicates that the resource has been permanently moved.
    PermanentRedirect,

    /// # Bad Request
    ///
    /// The HTTP 400 Bad Request status code indicates that the server cannot process the request due to client error.
    BadRequest,

    /// # Unauthorized
    ///
    /// The HTTP 401 Unauthorized status code indicates that authentication is required.
    Unauthorized,

    /// # Payment Required
    ///
    /// The HTTP 402 Payment Required status code is reserved for future use.
    PaymentRequired,

    /// # Forbidden
    ///
    /// The HTTP 403 Forbidden status code indicates that the client does not have access rights.
    Forbidden,

    /// # Not Found
    ///
    /// The HTTP 404 Not Found status code indicates that the server cannot find the requested resource.
    NotFound,

    /// # Method Not Allowed
    ///
    /// The HTTP 405 Method Not Allowed status code indicates that the request method is not supported.
    MethodNotAllowed,

    /// # Not Acceptable
    ///
    /// The HTTP 406 Not Acceptable status code indicates that the server cannot produce a response matching the criteria.
    NotAcceptable,

    /// # Proxy Authentication Required
    ///
    /// The HTTP 407 Proxy Authentication Required status code indicates that proxy authentication is required.
    ProxyAuthenticationRequired,

    /// # Request Timeout
    ///
    /// The HTTP 408 Request Timeout status code indicates that the server timed out waiting for the request.
    RequestTimeout,

    /// # Conflict
    ///
    /// The HTTP 409 Conflict status code indicates a conflict with the current state of the resource.
    Conflict,

    /// # Gone
    ///
    /// The HTTP 410 Gone status code indicates that the resource is no longer available.
    Gone,

    /// # Length Required
    ///
    /// The HTTP 411 Length Required status code indicates that the request did not specify content length.
    LengthRequired,

    /// # Precondition Failed
    ///
    /// The HTTP 412 Precondition Failed status code indicates that preconditions were not met.
    PreconditionFailed,

    /// # Payload Too Large
    ///
    /// The HTTP 413 Payload Too Large status code indicates that the request entity is too large.
    PayloadTooLarge,

    /// # URI Too Long
    ///
    /// The HTTP 414 URI Too Long status code indicates that the URI is too long.
    UriTooLong,

    /// # Unsupported Media Type
    ///
    /// The HTTP 415 Unsupported Media Type status code indicates that the media format is not supported.
    UnsupportedMediaType,

    /// # Range Not Satisfiable
    ///
    /// The HTTP 416 Range Not Satisfiable status code indicates that the requested range cannot be fulfilled.
    RangeNotSatisfiable,

    /// # Expectation Failed
    ///
    /// The HTTP 417 Expectation Failed status code indicates that expectations cannot be met.
    ExpectationFailed,

    /// # I'm a teapot
    ///
    /// The HTTP 418 I'm a teapot status code indicates that the server refuses to brew coffee because it is a teapot.
    ImATeapot,

    /// # Misdirected Request
    ///
    /// The HTTP 421 Misdirected Request status code indicates that the request was directed at a server unable to produce a response.
    MisdirectedRequest,

    /// # Unprocessable Entity
    ///
    /// The HTTP 422 Unprocessable Entity status code indicates that the request was well-formed but cannot be processed.
    UnprocessableEntity,

    /// # Locked
    ///
    /// The HTTP 423 Locked status code indicates that the resource is locked.
    Locked,

    /// # Failed Dependency
    ///
    /// The HTTP 424 Failed Dependency status code indicates that the request failed due to a previous request failure.
    FailedDependency,

    /// # Too Early
    ///
    /// The HTTP 425 Too Early status code indicates that the server is unwilling to risk processing a request.
    TooEarly,

    /// # Upgrade Required
    ///
    /// The HTTP 426 Upgrade Required status code indicates that the client should switch to a different protocol.
    UpgradeRequired,

    /// # Precondition Required
    ///
    /// The HTTP 428 Precondition Required status code indicates that the origin server requires the request to be conditional.
    PreconditionRequired,

    /// # Too Many Requests
    ///
    /// The HTTP 429 Too Many Requests status code indicates that the user has sent too many requests.
    TooManyRequests,

    /// # Request Header Fields Too Large
    ///
    /// The HTTP 431 status code indicates that the request header fields are too large.
    RequestHeaderFieldsTooLarge,

    /// # Unavailable For Legal Reasons
    ///
    /// The HTTP 451 Unavailable For Legal Reasons status code indicates that the resource is unavailable due to legal reasons.
    UnavailableForLegalReasons,

    /// # Internal Server Error
    ///
    /// The HTTP 500 Internal Server Error status code indicates that the server encountered an unexpected condition.
    InternalServerError,

    /// # Not Implemented
    ///
    /// The HTTP 501 Not Implemented status code indicates that the request method is not supported.
    NotImplemented,

    /// # Bad Gateway
    ///
    /// The HTTP 502 Bad Gateway status code indicates that the server received an invalid response from upstream.
    BadGateway,

    /// # Service Unavailable
    ///
    /// The HTTP 503 Service Unavailable status code indicates that the server is not ready to handle the request.
    ServiceUnavailable,

    /// # Gateway Timeout
    ///
    /// The HTTP 504 Gateway Timeout status code indicates that the server did not receive a timely response.
    GatewayTimeout,

    /// # HTTP Version Not Supported
    ///
    /// The HTTP 505 HTTP Version Not Supported status code indicates that the HTTP version is not supported.
    HttpVersionNotSupported,

    /// # Variant Also Negotiates
    ///
    /// The HTTP 506 Variant Also Negotiates status code indicates a configuration error.
    VariantAlsoNegotiates,

    /// # Insufficient Storage
    ///
    /// The HTTP 507 Insufficient Storage status code indicates that the server cannot store the representation.
    InsufficientStorage,

    /// # Loop Detected
    ///
    /// The HTTP 508 Loop Detected status code indicates that the server detected an infinite loop.
    LoopDetected,

    /// # Not Extended
    ///
    /// The HTTP 510 Not Extended status code indicates that further extensions are required.
    NotExtended,

    /// # Network Authentication Required
    ///
    /// The HTTP 511 Network Authentication Required status code indicates that authentication is required to gain network access.
    NetworkAuthenticationRequired,

    /// # Other
    ///
    /// A non-standard or custom HTTP status code.
    Other(u32),
}

impl HttpCode {
    /// # As Status Code
    ///
    /// Returns the HTTP Code as an `u32` type.
    ///
    /// For example:
    ///
    /// ```
    /// use async_web::web::resolution::http_code::HttpCode;
    /// let status = HttpCode::OK;
    ///
    /// let status_code: u32 = status.as_status_code(); // -> 200
    /// ```
    pub fn as_status_code(&self) -> u32 {
        match self {
            HttpCode::Continue => 100,
            HttpCode::SwitchingProtocols => 101,
            HttpCode::Processing => 102,
            HttpCode::EarlyHints => 103,
            HttpCode::OK => 200,
            HttpCode::Created => 201,
            HttpCode::Accepted => 202,
            HttpCode::NonAuthoritativeInformation => 203,
            HttpCode::NoContent => 204,
            HttpCode::ResetContent => 205,
            HttpCode::PartialContent => 206,
            HttpCode::MultiStatus => 207,
            HttpCode::AlreadyReported => 208,
            HttpCode::ImUsed => 226,
            HttpCode::MultipleChoices => 300,
            HttpCode::MovedPermanently => 301,
            HttpCode::Found => 302,
            HttpCode::SeeOther => 303,
            HttpCode::NotModified => 304,
            HttpCode::UseProxy => 305,
            HttpCode::TemporaryRedirect => 307,
            HttpCode::PermanentRedirect => 308,
            HttpCode::BadRequest => 400,
            HttpCode::Unauthorized => 401,
            HttpCode::PaymentRequired => 402,
            HttpCode::Forbidden => 403,
            HttpCode::NotFound => 404,
            HttpCode::MethodNotAllowed => 405,
            HttpCode::NotAcceptable => 406,
            HttpCode::ProxyAuthenticationRequired => 407,
            HttpCode::RequestTimeout => 408,
            HttpCode::Conflict => 409,
            HttpCode::Gone => 410,
            HttpCode::LengthRequired => 411,
            HttpCode::PreconditionFailed => 412,
            HttpCode::PayloadTooLarge => 413,
            HttpCode::UriTooLong => 414,
            HttpCode::UnsupportedMediaType => 415,
            HttpCode::RangeNotSatisfiable => 416,
            HttpCode::ExpectationFailed => 417,
            HttpCode::ImATeapot => 418,
            HttpCode::MisdirectedRequest => 421,
            HttpCode::UnprocessableEntity => 422,
            HttpCode::Locked => 423,
            HttpCode::FailedDependency => 424,
            HttpCode::TooEarly => 425,
            HttpCode::UpgradeRequired => 426,
            HttpCode::PreconditionRequired => 428,
            HttpCode::TooManyRequests => 429,
            HttpCode::RequestHeaderFieldsTooLarge => 431,
            HttpCode::UnavailableForLegalReasons => 451,
            HttpCode::InternalServerError => 500,
            HttpCode::NotImplemented => 501,
            HttpCode::BadGateway => 502,
            HttpCode::ServiceUnavailable => 503,
            HttpCode::GatewayTimeout => 504,
            HttpCode::HttpVersionNotSupported => 505,
            HttpCode::VariantAlsoNegotiates => 506,
            HttpCode::InsufficientStorage => 507,
            HttpCode::LoopDetected => 508,
            HttpCode::NotExtended => 510,
            HttpCode::NetworkAuthenticationRequired => 511,
            HttpCode::Other(c) => *c,
        }
    }

    /// # As Status
    ///
    /// Returns the HTTP status as a String variant.
    ///
    /// For example:
    ///
    /// ```
    /// use async_web::web::resolution::http_code::HttpCode;
    /// let status_code = HttpCode::Continue;
    ///
    /// let status: String = status_code.as_status(); // -> "Continue"
    /// ```
    pub fn as_status(&self) -> String {
        let status = match self {
            HttpCode::Continue => "Continue",
            HttpCode::SwitchingProtocols => "Switching Protocols",
            HttpCode::Processing => "Processing",
            HttpCode::EarlyHints => "Early Hints",
            HttpCode::OK => "OK",
            HttpCode::Created => "Created",
            HttpCode::Accepted => "Accepted",
            HttpCode::NonAuthoritativeInformation => "Non-Authoritative Information",
            HttpCode::NoContent => "No Content",
            HttpCode::ResetContent => "Reset Content",
            HttpCode::PartialContent => "Partial Content",
            HttpCode::MultiStatus => "Multi-Status",
            HttpCode::AlreadyReported => "Already Reported",
            HttpCode::ImUsed => "IM Used",
            HttpCode::MultipleChoices => "Multiple Choices",
            HttpCode::MovedPermanently => "Moved Permanently",
            HttpCode::Found => "Found",
            HttpCode::SeeOther => "See Other",
            HttpCode::NotModified => "Not Modified",
            HttpCode::UseProxy => "Use Proxy",
            HttpCode::TemporaryRedirect => "Temporary Redirect",
            HttpCode::PermanentRedirect => "Permanent Redirect",
            HttpCode::BadRequest => "Bad Request",
            HttpCode::Unauthorized => "Unauthorized",
            HttpCode::PaymentRequired => "Payment Required",
            HttpCode::Forbidden => "Forbidden",
            HttpCode::NotFound => "Not Found",
            HttpCode::MethodNotAllowed => "Method Not Allowed",
            HttpCode::NotAcceptable => "Not Acceptable",
            HttpCode::ProxyAuthenticationRequired => "Proxy Authentication Required",
            HttpCode::RequestTimeout => "Request Timeout",
            HttpCode::Conflict => "Conflict",
            HttpCode::Gone => "Gone",
            HttpCode::LengthRequired => "Length Required",
            HttpCode::PreconditionFailed => "Precondition Failed",
            HttpCode::PayloadTooLarge => "Payload Too Large",
            HttpCode::UriTooLong => "URI Too Long",
            HttpCode::UnsupportedMediaType => "Unsupported Media Type",
            HttpCode::RangeNotSatisfiable => "Range Not Satisfiable",
            HttpCode::ExpectationFailed => "Expectation Failed",
            HttpCode::ImATeapot => "I'm a teapot",
            HttpCode::MisdirectedRequest => "Misdirected Request",
            HttpCode::UnprocessableEntity => "Unprocessable Entity",
            HttpCode::Locked => "Locked",
            HttpCode::FailedDependency => "Failed Dependency",
            HttpCode::TooEarly => "Too Early",
            HttpCode::UpgradeRequired => "Upgrade Required",
            HttpCode::PreconditionRequired => "Precondition Required",
            HttpCode::TooManyRequests => "Too Many Requests",
            HttpCode::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            HttpCode::UnavailableForLegalReasons => "Unavailable For Legal Reasons",
            HttpCode::InternalServerError => "Internal Server Error",
            HttpCode::NotImplemented => "Not Implemented",
            HttpCode::BadGateway => "Bad Gateway",
            HttpCode::ServiceUnavailable => "Service Unavailable",
            HttpCode::GatewayTimeout => "Gateway Timeout",
            HttpCode::HttpVersionNotSupported => "HTTP Version Not Supported",
            HttpCode::VariantAlsoNegotiates => "Variant Also Negotiates",
            HttpCode::InsufficientStorage => "Insufficient Storage",
            HttpCode::LoopDetected => "Loop Detected",
            HttpCode::NotExtended => "Not Extended",
            HttpCode::NetworkAuthenticationRequired => "Network Authentication Required",
            HttpCode::Other(c) => &format!("Unknown {c}"),
        };

        String::from(status)
    }
}

/// Maps a numeric status code (e.g. `200`) to the corresponding
/// [`HttpCode`] variant. Unknown codes fall through to
/// [`HttpCode::Other`].
impl Into<HttpCode> for u32 {
    fn into(self) -> HttpCode {
        match self {
            100 => HttpCode::Continue,
            101 => HttpCode::SwitchingProtocols,
            102 => HttpCode::Processing,
            103 => HttpCode::EarlyHints,
            200 => HttpCode::OK,
            201 => HttpCode::Created,
            202 => HttpCode::Accepted,
            203 => HttpCode::NonAuthoritativeInformation,
            204 => HttpCode::NoContent,
            205 => HttpCode::ResetContent,
            206 => HttpCode::PartialContent,
            207 => HttpCode::MultiStatus,
            208 => HttpCode::AlreadyReported,
            226 => HttpCode::ImUsed,
            300 => HttpCode::MultipleChoices,
            301 => HttpCode::MovedPermanently,
            302 => HttpCode::Found,
            303 => HttpCode::SeeOther,
            304 => HttpCode::NotModified,
            305 => HttpCode::UseProxy,
            307 => HttpCode::TemporaryRedirect,
            308 => HttpCode::PermanentRedirect,
            400 => HttpCode::BadRequest,
            401 => HttpCode::Unauthorized,
            402 => HttpCode::PaymentRequired,
            403 => HttpCode::Forbidden,
            404 => HttpCode::NotFound,
            405 => HttpCode::MethodNotAllowed,
            406 => HttpCode::NotAcceptable,
            407 => HttpCode::ProxyAuthenticationRequired,
            408 => HttpCode::RequestTimeout,
            409 => HttpCode::Conflict,
            410 => HttpCode::Gone,
            411 => HttpCode::LengthRequired,
            412 => HttpCode::PreconditionFailed,
            413 => HttpCode::PayloadTooLarge,
            414 => HttpCode::UriTooLong,
            415 => HttpCode::UnsupportedMediaType,
            416 => HttpCode::RangeNotSatisfiable,
            417 => HttpCode::ExpectationFailed,
            418 => HttpCode::ImATeapot,
            421 => HttpCode::MisdirectedRequest,
            422 => HttpCode::UnprocessableEntity,
            423 => HttpCode::Locked,
            424 => HttpCode::FailedDependency,
            425 => HttpCode::TooEarly,
            426 => HttpCode::UpgradeRequired,
            428 => HttpCode::PreconditionRequired,
            429 => HttpCode::TooManyRequests,
            431 => HttpCode::RequestHeaderFieldsTooLarge,
            451 => HttpCode::UnavailableForLegalReasons,
            500 => HttpCode::InternalServerError,
            501 => HttpCode::NotImplemented,
            502 => HttpCode::BadGateway,
            503 => HttpCode::ServiceUnavailable,
            504 => HttpCode::GatewayTimeout,
            505 => HttpCode::HttpVersionNotSupported,
            506 => HttpCode::VariantAlsoNegotiates,
            507 => HttpCode::InsufficientStorage,
            508 => HttpCode::LoopDetected,
            510 => HttpCode::NotExtended,
            511 => HttpCode::NetworkAuthenticationRequired,
            _ => HttpCode::Other(self),
        }
    }
}

/// Converts an [`HttpCode`] back into its numeric status code by delegating
/// to [`HttpCode::as_status_code`].
impl Into<u32> for HttpCode {
    fn into(self) -> u32 {
        self.as_status_code()
    }
}

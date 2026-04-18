use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpParseError {
    #[error("meta like the method, route, or host was missing")]
    MissingRequiredMeta,
    #[error("invalid header encoutered")]
    InvalidHeader,
    #[error("invalid content length value inserted into header")]
    InvalidContentLength,
    #[error("{0}")]
    IO(std::io::Error),

    #[error("the type of body read {0} is not supported")]
    UnsupportedBody(String)
}
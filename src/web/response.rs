//! # Response
//!
//! The [`Response`] builder and its header/body types. A [`Response`] is
//! constructed per-request, configured by the registered handler and any
//! middleware, and finally serialised into a byte buffer that is written
//! back out over the TCP stream.

pub mod http_code;

use linked_hash_map::LinkedHashMap;
#[cfg(feature = "json")]
use serde::Serialize;
use smol::fs;
use std::{path::Path, sync::Arc};

/// String type used for HTTP header names.
pub type HeaderKey = String;
/// String type used for HTTP header values.
pub type HeaderValue = String;

use crate::{
    RequestError,
    web::{http_code::HttpCode, http_request::Parsers},
};

/// # Response
///
/// Builder for the outgoing HTTP response. Holds the status code, an
/// ordered header map, an optional body, and the parser that will be used
/// to serialise the final byte payload.
#[derive(Debug)]
pub struct Response {
    headers: LinkedHashMap<HeaderKey, HeaderValue>,
    status: HttpCode,
    body: Option<Arc<[u8]>>,
    parser: Parsers,
}

impl Response {
    /// Create a new response object with the selected parser (determines how the data is encoded)
    pub fn new(parser: &Parsers) -> Self {
        Self {
            headers: LinkedHashMap::new(),
            status: HttpCode::OK,
            body: None,
            parser: parser.clone(),
        }
    }

    /// Set the status of the response
    pub fn status(&mut self, code: impl Into<HttpCode>) -> &mut Response {
        self.status = code.into();
        self
    }

    /// Set a header's value
    pub fn set_header(
        &mut self,
        key: impl Into<HeaderKey>,
        val: impl Into<HeaderValue>,
    ) -> &mut Response {
        self.headers.insert(key.into(), val.into());
        self
    }

    /// Retrieve a header via the key
    pub fn get_header(&self, key: &HeaderKey) -> Option<&HeaderValue> {
        self.headers.get(key)
    }

    /// A reference to a linked hashmap
    pub fn headers(&self) -> &LinkedHashMap<HeaderKey, HeaderValue> {
        &self.headers
    }

    /// set the body of the response.
    pub fn set_body(&mut self, body: impl Into<Arc<[u8]>>) -> &mut Response {
        let data = body.into();

        let content_length = data.len();
        self.set_header("Content-Length", content_length.to_string());

        self.body = Some(data);
        self
    }

    /// sets the body to none
    pub fn no_body(&mut self) -> &mut Response {
        self.body = None;
        self
    }

    /// Changes the header `Content-Type` to `text/plain` and sets the body to represent the `UTF-8`.
    pub fn text(&mut self, text: impl Into<String>) -> &mut Response {
        self.set_header("Content-Type", "text/plain");
        self.set_body(text.into().as_bytes());
        self
    }

    /// From a file path attempts to read and return the file data to the user.
    ///
    /// ## Returns
    ///
    /// If the path does not exist, an error is returned.
    ///
    /// If the file cannot be read, an error is returned.
    pub async fn file(&mut self, file_path: &str) -> Result<&mut Response, std::io::Error> {
        let path = Path::new(file_path);

        // check if the path exist
        if !Path::exists(path) {
            Err(std::io::Error::from(std::io::ErrorKind::NotFound))
        } else {
            // read the file, set the header, set the body, and return the response builder
            let file_content = fs::read(path).await?;

            self.set_header("Content-Type", get_file_type_header(&file_path));
            self.set_body(file_content);
            Ok(self)
        }
    }
}

/// Serialises a [`Response`] into the wire-format byte buffer dictated by
/// the configured [`Parsers`] variant (for example, HTTP/1.1 framing).
impl Into<Arc<[u8]>> for Response {
    fn into(self) -> Arc<[u8]> {
        match self.parser {
            Parsers::HttpV1 => {
                let header_first = format!(
                    "HTTP/1.1 {} {}\r\n",
                    self.status.as_status_code(),
                    self.status.as_status()
                );

                let mut buffer: Vec<u8> = vec![];
                buffer.extend(header_first.into_bytes());

                for (k, v) in self.headers {
                    buffer.extend(format!("{k}: {v}\r\n").into_bytes());
                }
                buffer.extend(b"\r\n");

                if let Some(body) = self.body {
                    buffer.extend(body.iter());
                }

                buffer.into()
            }
        }
    }
}

#[cfg(feature = "json")]
impl Response {
    /// Serialises `val` as JSON, sets the `Content-Type` header to
    /// `application/json`, and stores the resulting bytes as the response
    /// body.
    ///
    /// Returns the [`serde_json::Error`] produced by `serde_json::to_string`
    /// if serialisation fails.
    pub fn json<T>(&mut self, val: &T) -> Result<&mut Response, serde_json::Error>
    where
        T: Serialize,
    {
        let json_content = serde_json::to_string(&val)?;

        self.set_header("Content-Type", "application/json");
        self.set_body(json_content.into_bytes());

        Ok(self)
    }
}

// ? Convert serde_json::Error into RequestError
#[cfg(feature = "json")]
impl From<serde_json::Error> for RequestError {
    fn from(value: serde_json::Error) -> Self {
        RequestError::new(move |res| {
            res.text(value.to_string()).status(HttpCode::InternalServerError);
        })
    }
}

// ? Convert STD IO Errors into request errors
impl From<std::io::Error> for RequestError {
    fn from(value: std::io::Error) -> Self {
        let code = match value.kind() {
            std::io::ErrorKind::NotFound => HttpCode::NotFound,
            std::io::ErrorKind::PermissionDenied => HttpCode::Unauthorized,
            std::io::ErrorKind::QuotaExceeded => HttpCode::TooManyRequests,
            _ => HttpCode::InternalServerError,
        };

        RequestError::new(move |res| {
            res.status(code).text(value.to_string());
        })
    }
}

fn get_file_type_header(file_path: &str) -> String {
    // extract extension (lowercased)
    let ext = match std::path::Path::new(file_path)
        .extension()
        .and_then(|e| e.to_str())
    {
        Some(e) => e.to_lowercase(),
        None => return "application/octet-stream".to_string(),
    };

    match ext.as_str() {
        // text types
        "html" | "htm" => "text/html",
        "css" => "text/css",
        "js" => "application/javascript",
        "json" => "application/json",
        "txt" => "text/plain",
        "csv" => "text/csv",
        "xml" => "application/xml",

        // images
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "bmp" => "image/bmp",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",

        // audio / video
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "ogg" => "audio/ogg",
        "mp4" => "video/mp4",
        "webm" => "video/webm",

        // fonts
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "ttf" => "font/ttf",
        "otf" => "font/otf",

        // documents / archives
        "pdf" => "application/pdf",
        "zip" => "application/zip",
        "tar" => "application/x-tar",
        "gz" => "application/gzip",

        // fallback
        _ => "application/octet-stream",
    }
    .to_string()
}

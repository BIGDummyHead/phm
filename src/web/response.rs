pub mod http_code;

use linked_hash_map::LinkedHashMap;
use serde::Serialize;
use smol::fs;
use std::{path::Path, sync::Arc};

pub type HeaderKey = String;
pub type HeaderValue = String;

use crate::{
    RequestError,
    web::{http_code::HttpCode, http_request::Parsers},
};
/// Resolution placeholder things will implment on top of this, should last as long as the request itself.
pub struct Response {
    headers: LinkedHashMap<HeaderKey, HeaderValue>,
    status: HttpCode,
    body: Option<Arc<[u8]>>,
    version: String,
}

impl Response {
    pub fn new(parser: &Parsers) -> Self {
        let version = match parser {
            Parsers::HttpV1 => "HTTP/1.1",
        }
        .to_string();

        Self {
            headers: LinkedHashMap::new(),
            status: HttpCode::OK,
            body: None,
            version,
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

    pub fn body(&mut self, body: impl Into<Arc<[u8]>>) -> &mut Response {
        let data = body.into();

        let content_length = data.len();
        self.set_header("Content-Length", content_length.to_string());

        self.body = Some(data);
        self
    }

    pub fn no_body(&mut self) -> &mut Response {
        self.body = None;
        self
    }

    pub fn text(&mut self, text: impl Into<String>) -> &mut Response {
        self.set_header("Content-Type", "text/plain");
        self.body(text.into().as_bytes());
        self
    }

    pub async fn file(
        &mut self,
        file_path: impl Into<String>,
    ) -> Result<&mut Response, std::io::Error> {
        let file_path = file_path.into();
        let path = Path::new(&file_path);
        if !Path::exists(path) {
            return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
        }

        let file_content = fs::read(path).await?;

        self.set_header("Content-Type", get_file_type_header(&file_path));
        self.body(file_content);
        Ok(self)
    }
}

impl Into<Arc<[u8]>> for Response {
    fn into(self) -> Arc<[u8]> {
        let header_first = format!(
            "{} {} {}\r\n",
            self.version,
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

impl Response {
    pub fn json<T>(&mut self, val: &T) -> Result<&mut Response, serde_json::Error>
    where
        T: Serialize,
    {
        let json_content = serde_json::to_string(&val)?;

        self.set_header("Content-Type", "application/json");
        self.body(json_content.into_bytes());

        Ok(self)
    }
}

impl From<serde_json::Error> for RequestError {
    fn from(value: serde_json::Error) -> Self {
        let mut req_e = RequestError::default();
        req_e.set_message(value.to_string());
        req_e.set_status(HttpCode::InternalServerError);
        req_e
    }
}

impl From<std::io::Error> for RequestError {
    fn from(value: std::io::Error) -> Self {
        let code = match value.kind() {
            std::io::ErrorKind::NotFound => HttpCode::NotFound,
            std::io::ErrorKind::PermissionDenied => HttpCode::Unauthorized,
            std::io::ErrorKind::QuotaExceeded => HttpCode::TooManyRequests,
            _ => HttpCode::InternalServerError,
        };

        let mut req_e = RequestError::default();
        req_e.set_status(code);
        req_e
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

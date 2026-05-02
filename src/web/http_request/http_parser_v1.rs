//! # HTTP/1.1 Parser
//!
//! [`HttpRequestMetaParser`] implementation for HTTP/1.1. Reads the request
//! line, header block, and body (either length-prefixed or chunked) from a
//! [`TcpStream`] and produces an [`HttpRequestMeta`].

use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use smol::{io::{AsyncBufReadExt, AsyncReadExt, BufReader}, net::TcpStream};

use crate::{
    HttpMethod,
    web::http_request::{HttpRequestMeta, HttpRequestMetaParser, http_parse_error::HttpParseError},
};

/// Case-insensitive map from header name to header value produced while
/// parsing the request. Keys and values are stored lower-cased.
pub type HeaderMap = HashMap<String, String>;

/// # HttpParserV1
///
/// Zero-sized HTTP/1.1 parser. Implements [`HttpRequestMetaParser`] to
/// drive reading the request line, headers, and body from an incoming TCP
/// stream.
#[derive(Default)]
pub struct HttpParserV1;

async fn parse_route_meta<'req>(
    buf_stream: &mut BufReader<&mut TcpStream>,
) -> Result<(String, HttpMethod), HttpParseError> {
    let mut data = String::new();
    buf_stream.read_line(&mut data).await?;

    let mut spl = data.split(' ');

    let method = spl
        .next()
        .map(|req_method| {
            let req_method = req_method.to_uppercase();
            match req_method.as_str() {
                "POST" => HttpMethod::POST,
                "PUT" => HttpMethod::PUT,
                "PATCH" => HttpMethod::PATCH,
                "GET" => HttpMethod::GET,
                "OPTIONS" => HttpMethod::OPTIONS,
                "CONNECT" => HttpMethod::CONNECT,
                "DELETE" => HttpMethod::DELETE,
                "HEAD" => HttpMethod::HEAD,
                "TRACE" => HttpMethod::TRACE,
                other => HttpMethod::Unknown(other.to_string()),
            }
        })
        .ok_or(HttpParseError::MissingRequiredMeta)?;

    let route = spl.next().ok_or(HttpParseError::MissingRequiredMeta)?;

    Ok((route.to_string(), method))
}

async fn create_header_map(
    buf_stream: &mut BufReader<&mut TcpStream>,
) -> Result<HeaderMap, HttpParseError> {
    let mut map = HashMap::new();

    // read until an empty line is encoutered, notating that there is a body to be read
    loop {
        let mut buf = String::new();
        buf_stream.read_line(&mut buf).await?;

        let trim_buf = buf.trim();

        // ready to read that body 😘😘😘 <- this shit is funny
        if trim_buf.is_empty() {
            break;
        }

        let (header_name, header_value) =
            trim_buf.split_once(":").ok_or(HttpParseError::InvalidHeader)?;

        // ! insert case insensitive, very important
        map.insert(
            header_name.trim().to_lowercase(),
            header_value.to_string(),
        );
    }

    Ok(map)
}

async fn read_body(
    buf_stream: &mut BufReader<&mut TcpStream>,
    headers: &HeaderMap,
) -> Result<Arc<[u8]>, HttpParseError> {
    let content_length = headers
        .get("content-length")
        .map(|s| str::parse::<usize>(&*s).map_err(|_| HttpParseError::InvalidContentLength))
        .transpose()?;

    //there is a fixed body to read 🤤🍴
    if let Some(content_length) = content_length {
        let mut buf = vec![0u8; content_length];

        buf_stream.read_exact(&mut buf).await?;

        let arc_buf: Arc<[u8]> = buf.into();

        return Ok(arc_buf);
    }

    let transfer_encode = headers.get("transfer-encoding");

    if let Some(transfer_type) = transfer_encode {
        if *transfer_type != "chunked" {
            return Err(HttpParseError::UnsupportedBody(transfer_type.to_string()));
        }

        // chunked, assume only supported currently
        let mut data: Vec<u8> = Vec::new();
        loop {
            let mut buf_size = String::new();
            buf_stream.read_line(&mut buf_size).await?;

            if buf_size.is_empty() {
                return Err(HttpParseError::InvalidBufferSize);
            }

            let buf_size = buf_size
                .parse::<usize>()
                .map_err(|_| HttpParseError::InvalidBufferSize)?;

            // read the line no matter if the buffer size is 0
            let mut content = String::new();
            buf_stream.read_line(&mut content).await?;

            if buf_size == 0 {
                break;
            }

            data.extend(content.as_bytes());
        }

        return Ok(data.into());
    }

    // we can configure other things here like Transfer chunk encoding...

    Ok(Arc::new([0u8; 0]))
}

#[async_trait]
impl HttpRequestMetaParser for HttpParserV1 {
    async fn parse<'req>(stream: &'req mut TcpStream) -> Result<HttpRequestMeta, HttpParseError> {
        let mut buffer = BufReader::new(stream);

        let (route, method) = parse_route_meta(&mut buffer).await?;

        let headers = create_header_map(&mut buffer).await?;

        let body = if !matches!(method, HttpMethod::GET) {
            read_body(&mut buffer, &headers).await?
        } else {
            Arc::new([0u8; 0])
        };

        Ok(HttpRequestMeta::new(route, method, headers, body))
    }
}

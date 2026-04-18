use std::{collections::HashMap, string::ParseError, sync::Arc};

use async_trait::async_trait;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, BufReader},
    net::TcpStream,
};

use crate::{
    HttpMethod,
    web::http_request::{HttpRequestMeta, HttpRequestMetaParser, http_parse_error::HttpParseError},
};

// responsible for parsing http1 request.
#[derive(Default)]
pub struct HttpParserV1;

fn parse_route_meta<'req>(data: &'req String) -> Result<(&'req str, HttpMethod), HttpParseError> {
    let mut spl = data.split(' ');

    let method = spl
        .next()
        .map(|req_method| {
            let req_method = req_method.to_lowercase();
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

    Ok((route, method))
}

async fn create_header_map<'req>(
    buf_stream: &'req mut BufReader<&'req mut TcpStream>,
) -> Result<HashMap<String, String>, HttpParseError> {
    let mut map = HashMap::new();

    // read until an empty line is encoutered, notating that there is a body to be read
    loop {
        let mut buf = String::new();
        buf_stream
            .read_line(&mut buf)
            .await
            .map_err(HttpParseError::IO)?;

        // ready to read that body 😘😘😘 <- this shit is funny
        if buf.is_empty() {
            break;
        }

        let (header_name, header_value) =
            buf.split_once(":").ok_or(HttpParseError::InvalidHeader)?;

        // ! insert case insensitive, very important
        map.insert(
            header_name.trim().to_lowercase(),
            header_value.trim().to_lowercase(),
        );
    }

    Ok(map)
}

async fn read_body<'req>(
    buf_stream: &'req mut BufReader<&'req mut TcpStream>,
    headers: &HashMap<&str, &str>,
) -> Result<Arc<[u8]>, HttpParseError> {
    let content_length = headers
        .get("content-length")
        .map(|s| str::parse::<usize>(*s).map_err(|_| HttpParseError::InvalidContentLength))
        .transpose()?;

    //there is a fixed body to read 🤤🍴
    if let Some(content_length) = content_length {
        let mut buf = vec![0u8; content_length];

        buf_stream
            .read_exact(&mut buf)
            .await
            .map_err(HttpParseError::IO)?;

        let arc_buf: Arc<[u8]> = buf.into();

        return Ok(arc_buf);
    }

    let transfer_encode = headers.get("transfer-encoding");

    if let Some(transfer_type) = transfer_encode {
        if *transfer_type != "chunked" {
            return Err(HttpParseError::UnsupportedBody(transfer_type.to_string()));
        }

        // chunked, assume only supported currently
        loop {
            let mut buf_size = String::new();
            buf_stream.read_line(&mut buf_size);
        }
    }

    // we can configure other things here like Transfer chunk encoding...

    Ok(Arc::new([0u8; 0]))
}

#[async_trait]
impl HttpRequestMetaParser for HttpParserV1 {
    async fn parse<'req>(stream: &'req mut TcpStream) -> Result<HttpRequestMeta, HttpParseError> {
        let mut buffer = BufReader::new(stream);

        let mut req_info_meta = String::new();
        buffer
            .read_line(&mut req_info_meta)
            .await
            .map_err(HttpParseError::IO)?;

        let (route, method) = parse_route_meta(&req_info_meta)?;

        loop {}

        todo!()
    }
}

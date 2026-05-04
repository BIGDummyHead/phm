use std::{fmt::Debug, sync::Arc};

use smol::io::{AsyncBufReadExt, AsyncReadExt, BufReader, Cursor};
use thiserror::Error;

use crate::web::http_request::HttpRequestMeta;

pub struct FileBody {
    file_name: String,
    content_type: String,
    data: Arc<[u8]>,
}

impl Debug for FileBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FileBody")
            .field("file_name", &self.file_name)
            .field("content_type", &self.content_type)
            .field("data", &self.data.len())
            .finish()
    }
}

impl FileBody {
    /// Name of the file
    pub fn name(&self) -> &String {
        &self.file_name
    }

    /// File's content type
    pub fn content_type(&self) -> &String {
        &self.content_type
    }

    /// File data
    pub fn file_data(&self) -> &Arc<[u8]> {
        &self.data
    }
}

#[derive(Debug, Error)]
pub enum FileReadError {
    #[error("Boundary does not match indicated boundary key")]
    BoundaryNoMatch,
    #[error("file name was missing!")]
    FileNameMissing,
    #[error("std::io read issue")]
    ReadError(std::io::Error),
    #[error("content type is missing from read")]
    ContentTypeMissing,
}

impl From<std::io::Error> for FileReadError {
    fn from(value: std::io::Error) -> Self {
        FileReadError::ReadError(value)
    }
}

#[async_trait::async_trait]
pub trait FileMetaExtension {
    /// As Files
    ///
    /// Transforms the MetaData Body reference into a collection of FileBody's reading each file.
    async fn as_files(&self) -> Vec<FileBody>;
}

/// Reads the buffer until the next boundary is encurred.
async fn read_until_next_boundary(
    reader: &mut BufReader<Cursor<&Arc<[u8]>>>,
    boundary: &[u8],
) -> std::io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    let mut byte = [0u8; 1];

    let mut pattern = Vec::with_capacity(2 + boundary.len());
    pattern.extend_from_slice(b"\r\n--");
    pattern.extend_from_slice(boundary);

    while reader.read(&mut byte).await? == 1 {
        buf.push(byte[0]);

        if buf.len() >= pattern.len() && &buf[buf.len() - pattern.len()..] == pattern.as_slice() {
            buf.truncate(buf.len() - pattern.len());
            break;
        }
    }

    Ok(buf)
}

/// Reads the next file in the form data sequence
async fn read_file(
    boundary: &str,
    reader: &mut BufReader<Cursor<&Arc<[u8]>>>,
) -> Result<Option<FileBody>, FileReadError> {
    let mut reader_boundary = String::new();
    reader.read_line(&mut reader_boundary).await?;

    // boundary does not match
    if reader_boundary != format!("--{boundary}\r\n") {
        return Err(FileReadError::BoundaryNoMatch);
    }

    let mut disposition = String::new();
    reader.read_line(&mut disposition).await?;

    let file_name = disposition
        .split_once(':')
        .map(|(_, data)| {
            let filename = data
                .split(';')
                .map(str::trim)
                .find(|md| md.starts_with("filename"))?;

            filename
                .split_once('=')
                .map(|(_, name)| name.trim().replace('"', ""))
        })
        .flatten()
        .ok_or(FileReadError::FileNameMissing)?;

    let mut content_type = String::new();
    reader.read_line(&mut content_type).await?;

    let content_type = content_type
        .split_once(':')
        .map(|(_, t)| t.trim().to_string())
        .ok_or(FileReadError::ContentTypeMissing)?;

    let mut empty_line = String::new();
    reader.read_line(&mut empty_line).await?;

    let file_data = read_until_next_boundary(reader, boundary.as_bytes()).await?;

    Ok(Some(FileBody {
        file_name,
        content_type,
        data: file_data.into(),
    }))
}

#[async_trait::async_trait]
impl FileMetaExtension for HttpRequestMeta {
    async fn as_files(&self) -> Vec<FileBody> {
        let mut files = vec![];

        let Some(boundary_key) = self
            .headers()
            .get("content-type")
            .map(|s| {
                let Some((data_type, boundary)) = s.split_once(';') else {
                    return None;
                };

                if data_type != "multipart/form-data" {
                    return None;
                }

                let Some((_, key)) = boundary.split_once('=') else {
                    return None;
                };

                Some(key)
            })
            .flatten()
        else {
            return files;
        };

        let cursor = Cursor::new(self.body_ref());

        let mut buf_reader = BufReader::new(cursor);

        loop {
            match read_file(boundary_key, &mut buf_reader).await {
                Ok(Some(file)) => files.push(file),
                Ok(None) => break,
                Err(_) => {
                    //dbg!(e);
                    break;
                }
            }
        }
        files
    }
}

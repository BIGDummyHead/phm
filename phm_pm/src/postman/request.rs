use serde::Serialize;

use crate::{Url, base_url};

#[derive(Serialize, Debug, Clone)]
pub struct Request {
    pub method: String,
    pub description: String,
    pub url: Url,
}

impl Request {
    /// # New
    ///
    /// Creates a new request object.
    ///
    /// ## Parameters
    ///
    /// `method`: The method that the router uses i.e. GET, POST, PUT, etc...
    /// `description`: The description of what this endpoint does.
    pub fn new(name: String, path: String, method: String, description: Option<String>) -> Self {
        Self {
            method,
            description: description.unwrap_or("".to_string()),
            url: Url::new(base_url(), path),
        }
    }
}

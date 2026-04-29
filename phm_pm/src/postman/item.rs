use serde::Serialize;

use crate::{Request, Url, base_url};

#[derive(Debug, Clone, Serialize)]
pub struct Item {
    pub name: String,
    pub request: Request,
}

impl Item {
    /// # New
    ///
    /// Creates a new item object.
    ///
    /// ## Parameters
    ///
    /// `name`: The name of the item in collection of items
    /// `method`: The method that is used to access the resource
    /// `description`: An optional description that tells you what the request does.
    pub fn new(name: String, path: String, method: String, description: Option<String>) -> Self {
        Self {
            name: name.clone(),
            request: Request::new(name, path, method, description),
        }
    }
}

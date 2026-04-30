use crate::Request;
use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, Clone, Serialize)]
pub struct Item {
    pub name: String,
    #[serde(skip_serializing_if = "Request::is_default")]
    pub request: Request,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub item: Vec<Box<Item>>,
}

impl Item {
    /// # Create
    ///
    /// Creates a new item object.
    ///
    /// ## Parameters
    ///
    /// `name`: The name of the item in collection of items
    /// `method`: The method that is used to access the resource
    /// `description`: An optional description that tells you what the request does.
    pub fn create(name: String, request: Request) -> Self {
        Self {
            name: name,
            request,
            item: Vec::new(),
        }
    }

    /// # Is Folder
    ///
    /// Determines if the item is a folder.
    pub fn is_folder(&self) -> bool {
        self.request.is_default()
    }

    /// # Folder
    ///
    /// Creates a new folder.
    pub fn folder(name: String) -> Self {
        Self {
            name,
            item: Vec::new(),
            request: Request::default(),
        }
    }
}

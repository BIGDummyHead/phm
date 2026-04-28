use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct Request {
    pub method: String,
    pub description: String
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
    pub fn new(method: String, description: Option<String>) -> Self {
        Self { method, description: description.unwrap_or("".to_string()) }
    }
}
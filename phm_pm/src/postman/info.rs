use serde::Serialize;

/// Metadata information for the api collection.
#[derive(Debug, Clone, Serialize)]
pub struct Info {
    /// Name of the collection
    pub name: String,
    /// Schema being used for the collection, automatically set.
    schema: String
}

impl Info {
    /// # New
    /// 
    /// Creates a new info metadata schema with a name.
    pub fn new(name: String) -> Self {
        Self {
            name,
            schema: "https://schema.getpostman.com/json/collection/v2.1.0/collection.json".to_string()
        }
    }
}


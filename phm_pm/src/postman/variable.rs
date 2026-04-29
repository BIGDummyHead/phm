use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Variable {
    pub key: String,
    pub value: String
}

impl Variable {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self { key: key.into(), value: value.into() }
    }
}
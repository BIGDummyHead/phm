use serde::Serialize;
use syn::{LitStr, Token, parse::Parse};

/// Metadata information for the api collection.
#[derive(Debug, Clone, Serialize)]
pub struct Info {
    /// Name of the collection
    pub name: String,
    /// Schema being used for the collection, automatically set.
    schema: String,
}

impl Info {
    /// # New
    ///
    /// Creates a new info metadata schema with a name.
    pub fn new(name: String) -> Self {
        Self {
            name,
            schema: "https://schema.getpostman.com/json/collection/v2.1.0/collection.json"
                .to_string(),
        }
    }

    /// # With Schema
    ///
    /// Creates a new info metadata schema with a name and defined postman schema
    pub fn with_schema(name: String, schema: String) -> Self {
        Self { name, schema }
    }
}

impl Parse for Info {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<LitStr>()?;

        let info = if !input.is_empty() {
            input.parse::<Token![,]>()?;
            let schema = input.parse::<LitStr>()?;

            Info::with_schema(name.value(), schema.value())
        } else {
            Info::new(name.value())
        };

        Ok(info)
    }
}

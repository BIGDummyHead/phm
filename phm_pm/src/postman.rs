mod api_schema;
mod info;
mod item;
mod request;

use std::{ops::DerefMut, sync::{LazyLock, RwLock}};

use api_schema::ApiSchema;
use info::Info;
pub use item::Item;
pub use request::Request;

static SCHEMA: LazyLock<RwLock< ApiSchema >> = LazyLock::new(|| {
    RwLock::new(ApiSchema::new("collection_name".to_string()))
});

pub fn add_to_schema(item: Item) -> Result<(), &'static str> {
    let mut s = SCHEMA.write().expect("write guard failed:");
    
    s.try_add_item(item)
}

pub fn json_schema() -> String {

    let schem = &*SCHEMA.read().expect("failed to obtain read lock");

    serde_json::to_string(schem).expect("failed to parse schema: ")
}

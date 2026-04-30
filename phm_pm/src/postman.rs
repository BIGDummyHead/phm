mod api_schema;
mod info;
mod item;
mod request;
mod url;
mod variable;

use std::{
    io::Write,
    path::Path,
    sync::{LazyLock, RwLock},
};

use api_schema::ApiSchema;
pub use info::Info;
pub use item::Item;
pub use request::Request;
pub use url::Url;
pub use variable::Variable;

pub static SCHEMA: LazyLock<RwLock<ApiSchema>> = LazyLock::new(|| {
    let mut schema = ApiSchema::new("My API".to_string());
    schema.add_variable("base_url", "http://localhost");
    RwLock::new(schema)
});

pub fn add_to_schema(item: Item) -> () {
    SCHEMA.write().expect("no write lock: ").add_item(item);

    let path_dir = Path::new("./postman");
    if !path_dir.exists() {
        match std::fs::create_dir(path_dir) {
            Ok(_) => {}
            Err(e) => eprintln!("{e}"),
        };
    }

    let mut f = std::fs::File::create(format!("{}/postman_api.json", path_dir.display()))
        .expect("failed to write file");

    f.write_all(json_schema().as_bytes())
        .expect("failed to write to postman schema");
}

pub fn base_url() -> String {
    "{{base_url}}".to_string()
}

pub fn json_schema() -> String {
    let schem = &*SCHEMA.read().expect("failed to obtain read lock");

    serde_json::to_string_pretty(schem).expect("failed to parse schema: ")
}

pub fn set_global_info(info: Info) -> () {
    SCHEMA
        .write()
        .expect("failed to obtain write lock")
        .set_info(info);
}

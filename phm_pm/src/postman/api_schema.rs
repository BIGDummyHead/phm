use crate::postman::Info;
use crate::{Item, Variable};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ApiSchema {
    info: Info,
    variable: Vec<Variable>,
    item: Vec<Item>,
}

impl ApiSchema {
    pub fn new(collection_name: String) -> Self {
        Self {
            item: Vec::new(),
            variable: Vec::new(),
            info: Info::new(collection_name),
        }
    }

    pub fn add_item(&mut self, item: Item) -> () {
        if let Some(existing_index) = self.item.iter().position(|i| {
            i.name.to_lowercase() == item.name.to_lowercase()
                && i.request.method.to_lowercase() == item.request.method.to_lowercase()
        }) {
            self.item.remove(existing_index);
        }

        self.item.push(item);
    }

    pub fn add_variable(&mut self, key: impl Into<String>, val: impl Into<String>) -> () {
        self.variable.push(Variable::new(key, val));
    }

    pub fn set_info(&mut self, info: Info) -> () {
        self.info = info;
    }
}

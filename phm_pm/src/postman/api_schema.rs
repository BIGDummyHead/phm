use crate::Item;
use crate::postman::Info;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ApiSchema {
    info: Info,
    item: Vec<Item>,
}

impl ApiSchema {
    pub fn new(collection_name: String) -> Self {
        Self {
            item: Vec::new(),
            info: Info::new(collection_name),
        }
    }

    pub fn try_add_item(&mut self, item: Item) -> Result<(), &'static str> {
        if let Some(_) = self.item.iter().find(|i| {
            i.name.to_lowercase() == item.name.to_lowercase()
                && i.request.method.to_lowercase() == item.request.method.to_lowercase()
        }) {
            return Err("item already exists");
        }

        self.item.push(item);

        Ok(())
    }

    pub fn info(&self) -> &Info {
        &self.info
    }

    pub fn items(&self) -> &Vec<Item> {
        &self.item
    }
}

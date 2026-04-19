use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::router::node::Node;

pub struct RouteTrace<'app> {
    variables: HashMap<String, String>,
    node: Arc<RwLock<Node<'app>>>,
}

impl<'app> RouteTrace<'app> {
    pub fn new(node: Arc<RwLock<Node<'app>>>, variables: HashMap<String, String>) -> Self {
        Self {
            node,
            variables
        }
    }
}
use std::{
    collections::HashMap,
    sync::{Arc, LazyLock},
};

use tokio::sync::RwLock;

use crate::{
    HttpMethod,
    router::RouterError,
    web::{ArcMiddlewareClosure, ArcRequestClosure},
};

pub static VARIABLE_ROUTE_SIGN: LazyLock<char> = LazyLock::new(|| ':');

type MethodToNode<'app> = HashMap<HttpMethod, Arc<RwLock<Node<'app>>>>;
pub struct Node<'app> {
    children: HashMap<&'app str, MethodToNode<'app>>,
    /// The portion of the route that is stored here (/api/**test**)
    route_part: &'app str,
    /// The middleware that is stored at this route that should be invoked.
    middleware: Vec<ArcMiddlewareClosure>,
    request_closure: Option<ArcRequestClosure>,
}

impl<'app> Node<'app> {
    /// # Is Variable
    ///
    /// Returns true if the node starts with the variable marker.
    pub fn is_variable(&self) -> bool {
        self.route_part.starts_with(*VARIABLE_ROUTE_SIGN)
    }

    /// # New head
    ///
    /// Creates a new head node to be used.
    pub fn new_head() -> Self {
        Self {
            children: HashMap::new(),
            route_part: "/",
            middleware: vec![],
            request_closure: None,
        }
    }

    /// # Add Child
    ///
    /// Adds a new child node to this node with a given method and route part.
    pub async fn add_child(
        &mut self,
        method: HttpMethod,
        route_part: &'app str,
        middleware: Vec<ArcMiddlewareClosure>,
        request_closure: Option<ArcRequestClosure>,
    ) -> Result<(), RouterError> {
        if route_part.is_empty() {
            return Err(RouterError::BadName);
        }

        let node = Node {
            children: HashMap::new(),
            route_part,
            middleware,
            request_closure,
        };

        let child_node = self.children.get_mut(route_part);

        match child_node {
            Some(child_map) => {
                if let Some(v) = child_map.get(&method) {
                    let mut node_lock = v.write().await;

                    // do not overwrite if there is already a request fn on there!
                    if let Some(_) = node_lock.request_fn() {
                        return Err(RouterError::AlreadyExist);
                    }
                }

                child_map.insert(method, Arc::new(RwLock::new(node)));
            }
            None => {
                let mut child_map = HashMap::new();
                child_map.insert(method, Arc::new(RwLock::new(node)));

                self.children.insert(route_part, child_map);
            }
        };

        Ok(())
    }

    /// Adds an empty child node.
    pub async fn add_empty_child(
        &mut self,
        method: HttpMethod,
        route_part: &'app str,
    ) -> Result<(), RouterError> {
        self.add_child(method, route_part, vec![], None).await
    }

    /// # Get Child
    ///
    /// Attempts to get the child by the route part.
    ///
    /// If the child does not exist HOWEVER a node that is a Variable exist than that item is returned.
    pub async fn get_child(
        &self,
        route_part: &str,
        method: &HttpMethod,
    ) -> Option<&Arc<RwLock<Node<'app>>>> {
        let exact_child_node = self.children.get(route_part).and_then(|m| m.get(method));

        if exact_child_node.is_none() {
            for (_, method_map) in &self.children {
                // check if the method is available and it is a variable
                if let Some(n) = method_map.get(method)
                    && n.read().await.is_variable()
                {
                    return Some(n);
                }
            }
        }

        exact_child_node
    }

    /// # Middleware
    ///
    /// The middleware that is associated with this node.
    pub fn middleware(&self) -> &Vec<ArcMiddlewareClosure> {
        &self.middleware
    }

    pub fn request_fn(&self) -> Option<&ArcRequestClosure> {
        match &self.request_closure {
            Some(c) => Some(c),
            None => None,
        }
    }

    /// # Route
    ///
    /// The route that is associated with this node.
    pub fn route(&self) -> &str {
        self.route_part
    }
}

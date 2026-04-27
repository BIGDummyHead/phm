//! # Node
//!
//! A single node in the router's trie. Each node owns a map from child
//! route segment → [`HttpMethod`] → child node, along with the middleware
//! and (optional) request closure associated with this segment.

use std::{
    collections::HashMap,
    sync::{Arc, LazyLock},
};

use smol::lock::RwLock;

use crate::{
    HttpMethod,
    router::RouterError,
    web::{ArcMiddlewareClosure, ArcRequestClosure},
};

/// The character used to mark a route segment as a variable (e.g.
/// `/users/:id`). Segments starting with this character match any single
/// path component and capture the value into the request's variables.
pub static VARIABLE_ROUTE_SIGN: LazyLock<char> = LazyLock::new(|| ':');

pub type MethodToNode<'app> = HashMap<HttpMethod, Arc<RwLock<Node<'app>>>>;

/// # Node
///
/// Trie node used by [`Router`](crate::router::Router) to represent a
/// single segment of a registered route. A node may hold multiple children
/// per segment string, differentiated by [`HttpMethod`].
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
                    let node_read = v.read().await;

                    // do not overwrite if there is already a request fn on there!
                    if let Some(_) = node_read.request_fn() {
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

    /// # Request Function
    ///
    /// The closure that should be invoked to produce a response when a
    /// request matches this node. Returns `None` if the node is purely an
    /// intermediate node in the trie (no handler registered).
    pub fn request_fn(&self) -> Option<&ArcRequestClosure> {
        match &self.request_closure {
            Some(c) => Some(c),
            None => None,
        }
    }

    /// # Middleware Mut
    ///
    /// A mutatable reference to the middleware that is on the node.
    pub fn middleware_mut(&mut self) -> &mut Vec<ArcMiddlewareClosure> {
        &mut self.middleware
    }

    /// # Set Request Fn
    ///
    /// Sets the request closure that controls how the request responds.
    pub fn set_request_fn(&mut self, request: Option<ArcRequestClosure>) -> () {
        self.request_closure = request;
    }

    /// # Route
    ///
    /// The route that is associated with this node.
    pub fn route(&self) -> &str {
        self.route_part
    }
}

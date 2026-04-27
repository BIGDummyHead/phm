//! # Router
//!
//! Trie-based HTTP route store. Each segment of a registered route becomes a
//! [`Node`] in the tree, keyed by the segment string and branched further by
//! [`HttpMethod`]. Variable segments (segments beginning with `:`) match any
//! value at that position and capture the raw string into the request's
//! variables map.

mod node;
mod router_error;

use std::{collections::HashMap, sync::Arc};

pub use node::Node;
pub use router_error::RouterError;
use smol::lock::RwLock;

use crate::{
    HttpMethod,
    router::node::MethodToNode,
    web::{ArcMiddlewareClosure, ArcRequestClosure},
};

/// # Router
///
/// Thread-safe wrapper around the root [`Node`] of a route trie. Used by
/// [`App`](crate::App) to register routes up-front and to look up the
/// handler for an incoming request.
pub struct Router<'app>
where
    'app: 'static,
{
    root: Arc<RwLock<Node<'app>>>,
    root_method: RwLock<MethodToNode<'app>>,
    missing_404: Arc<RwLock<Node<'app>>>,
}

unsafe impl<'app> Send for Router<'app> {}

unsafe impl<'app> Sync for Router<'app> {}
impl<'app> Router<'app> {
    /// Creates a new router whose head node holds no routes.
    pub fn new() -> Self {
        Self {
            root: Arc::new(RwLock::new(Node::new_head())),
            missing_404: Arc::new(RwLock::new(Node::new_head())),
            root_method: RwLock::new(MethodToNode::new()),
        }
    }

    /// # Set 404
    /// 
    /// Sets the request closure when a 404 request is received.
    pub async fn set_404(&self, request: ArcRequestClosure) -> () {
        let mut write_guard = self.missing_404.write().await;
        write_guard.set_request_fn(Some(request));
    }

    /// # Add Route
    ///
    /// Walks the trie, inserting empty intermediate nodes as needed, and
    /// attaches `middleware` and `request_closure` at the leaf node that
    /// corresponds to `full_route` under `method`.
    ///
    /// Returns a [`RouterError`] if the route already exists, contains an
    /// invalid segment, or could not be added.
    pub async fn add_route(
        &self,
        full_route: &'app str,
        method: HttpMethod,
        middleware: Vec<ArcMiddlewareClosure>,
        request_closure: ArcRequestClosure,
    ) -> Result<(), RouterError> {
        let mut current_node = self.root.clone();

        if full_route == "/" {
            let mut root_write = self.root_method.write().await;

            if let Some(_) = root_write.get(&method) {
                return Err(RouterError::AlreadyExist);
            }

            // set the data for this resolution
            let mut node = Node::new_head();
            node.set_request_fn(Some(request_closure));

            //add the middleware
            let head_middleware = node.middleware_mut();
            head_middleware.extend(middleware);

            //finally insert this node with the method
            root_write.insert(method, Arc::new(RwLock::new(node)));

            return Ok(());
        }

        let mut peek_route = full_route.split('/').peekable();
        while let Some(route_part) = peek_route.next() {
            if route_part.is_empty() {
                continue;
            }

            let is_last = match peek_route.peek() {
                None => true,
                Some(p) => p.is_empty(),
            };

            if !is_last {
                let mut current_node_lock = current_node.write().await;
                match current_node_lock
                    .add_empty_child(method.clone(), route_part)
                    .await
                {
                    Ok(_) => {}
                    Err(re) => match re {
                        RouterError::AlreadyExist => {}
                        _ => return Err(re),
                    },
                }

                let child = current_node_lock
                    .get_child(route_part, &method)
                    .await
                    .ok_or(RouterError::CouldNotAdd)?
                    .clone();

                drop(current_node_lock);

                current_node = child;
                continue;
            }

            let mut current_node_lock = current_node.write().await;
            return current_node_lock
                .add_child(method, route_part, middleware, Some(request_closure))
                .await;
        }

        Ok(())
    }

    /// # Get Route
    ///
    /// Walks the trie using `full_route` and `method` and returns the node
    /// that was matched. Any variable segments encountered along the way are
    /// captured into `variables` keyed by the segment name (with the leading
    /// `:` stripped).
    ///
    /// Returns [`RouterError::NotFound`] if no node matches the route.
    pub async fn get_route(
        &self,
        full_route: &str,
        method: HttpMethod,
        variables: &mut HashMap<String, String>,
    ) -> Result<Arc<RwLock<Node<'app>>>, RouterError> {
        if full_route == "/" {
            //try to find the root method, clone the node, and transform the option to result
            let read_guard = self.root_method.read().await;
            let head_method = read_guard
                .get(&method)
                .map(|n| n.clone())
                .ok_or(RouterError::NotFound);

            //check for a missing 404 route
            if let Err(e) = head_method {
                return match &self.missing_404.read().await.request_fn() {
                    Some(_) => Ok(self.missing_404.clone()),
                    None => Err(e),
                };
            }

            return head_method;
        }

        let mut current_node = self.root.clone();

        let mut route_parts = full_route.split('/');
        while let Some(route_part) = route_parts.next() {
            if route_part.is_empty() {
                continue;
            }

            let node = current_node.read().await;
            if node.is_variable() {
                variables.insert(node.route().to_string(), route_part.to_string());
            }

            let child_node = match node.get_child(route_part, &method).await {
                None => match &self.missing_404.read().await.request_fn() {
                    None => Err(RouterError::NotFound),
                    Some(_) => Ok(self.missing_404.clone()),
                },
                Some(c) => Ok(c.clone()),
            }?;

            {
                let c_node = child_node.read().await;
                if c_node.is_variable() {
                    variables.insert(
                        c_node.route().chars().skip(1).collect(),
                        route_part.to_string(),
                    );
                }
            }

            drop(node);
            current_node = child_node;
        }

        Ok(current_node)
    }
}

mod node;
mod router_error;
mod route_trace;

use std::{collections::HashMap, sync::Arc};

use node::Node;
use tokio::sync::RwLock;

pub use router_error::RouterError;
pub use route_trace::RouteTrace;

use crate::{
    HttpMethod,
    web::{ArcMiddlewareClosure, ArcRequestClosure},
};

pub struct Router<'app> {
    head: Arc<RwLock<Node<'app>>>,
}

impl<'app> Router<'app> {
    pub fn new() -> Self {
        Self {
            head: Arc::new(RwLock::new(Node::new_head())),
        }
    }

    pub async fn add_route(
        &self,
        full_route: &'app str,
        method: HttpMethod,
        middleware: Vec<ArcMiddlewareClosure>,
        request_closure: ArcRequestClosure,
    ) -> Result<(), RouterError> {
        let mut current_node = self.head.clone();

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
                    .expect("failed to add empty")
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

    pub async fn get_route(
        &self,
        full_route: &str,
        method: HttpMethod
    ) -> Result<RouteTrace<'app>, RouterError> {
        let mut current_node = self.head.clone();

        let mut variables = HashMap::new();

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
                None => Err(RouterError::NotFound),
                Some(c) => Ok(c.clone())
            }?;

            drop(node);
            current_node = child_node;
        }

        Ok(RouteTrace::new(current_node, variables))
    }
}
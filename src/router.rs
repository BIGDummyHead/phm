mod node;
mod router_error;

use std::{collections::HashMap, sync::Arc};

pub use node::Node;
pub use router_error::RouterError;
use smol::lock::RwLock;

use crate::{
    HttpMethod,
    web::{ArcMiddlewareClosure, ArcRequestClosure},
};

pub struct Router<'app>
where
    'app: 'static,
{
    head: Arc<RwLock<Node<'app>>>,
}

unsafe impl<'app> Send for Router<'app> {}

unsafe impl<'app> Sync for Router<'app> {}
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
        method: HttpMethod,
        variables: &mut HashMap<String, String>,
    ) -> Result<Arc<RwLock<Node<'app>>>, RouterError> {
        let mut current_node = self.head.clone();

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
                None => {
                    Err(RouterError::NotFound)
                },
                Some(c) => Ok(c.clone()),
            }?;

            {
                let c_node = child_node.read().await;
                if c_node.is_variable() {
                    variables.insert(c_node.route().chars().skip(1).collect(), route_part.to_string());
                }
            }

            drop(node);
            current_node = child_node;
        }

        Ok(current_node)
    }
}

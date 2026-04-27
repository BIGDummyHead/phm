use serde::Serialize;

/// # Route Document
/// 
/// Can be created from a function that utilizies the `request` macro.
/// 
/// This meta data is then collected and generated into a doc file that can be used in PostMan to import request pathing.
#[derive(Debug, Serialize, Clone)]
#[allow(dead_code)]
pub struct RouteDocument {
    route: String, // the actual route to get 
    method: String, // method to invoke
    docs: String // comments on how to use the route from the user.
}
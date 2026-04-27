use darling::{FromMeta, util::PathList};

/// # Request Args
/// 
/// Nested meta data that can be included in the `request` proc macro.
#[derive(FromMeta)]
pub struct RequestArgs {
    pub route: String, // the route at which the user can find the resource
    pub method: String, // the method (HttpMethod) at which the user can invoke on the route
    #[darling(default)]
    pub middleware: PathList // optional middleware fn() collection
}


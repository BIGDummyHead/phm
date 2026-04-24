//! # Router Error
//!
//! Error type surfaced by [`Router`](crate::router::Router) and
//! [`Node`](crate::router::Node) during route registration and lookup.

use thiserror::Error;

/// # Router Error
///
/// Enumerates the failure modes encountered when registering or resolving a
/// route within the router.
#[derive(Debug, Error)]
pub enum RouterError {
    /// A route with the same method and path already exists.
    #[error("this route already exist!")]
    AlreadyExist,
    /// A segment of the supplied route is invalid (for example, empty).
    #[error("route name is invalid")]
    BadName,
    /// No route in the trie matches the requested path and method.
    #[error("route did not exist")]
    NotFound,
    /// A node could not be added; typically returned when a child lookup
    /// failed unexpectedly after attempting to create an intermediate node.
    #[error("could not add the node")]
    CouldNotAdd
}
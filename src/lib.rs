//! # Project Hail Mary
//!
//! An asynchronous web framework built on top of [`smol`] that provides a
//! lifecycle-driven HTTP server, a trie-based router, middleware support, and
//! a pluggable HTTP protocol parser.
//!
//! The public surface is intentionally small. The core entry point is
//! [`App`], which transitions through a typestate lifecycle:
//! [`App<Closed>`](app::App) for registering routes and configuring the
//! parser, and [`App<Running>`](app::Running) once the listener has been
//! started. Requests are dispatched through handlers that conform to the
//! [`FutureClosureBound`](web::FutureClosureBound) trait and may be wrapped
//! by [`Middleware`] layers.

pub mod app;

/// Internal module hosting the HTTP abstractions (methods, requests,
/// responses, middleware, and status codes). Selected items are re-exported
/// from the crate root.
pub mod web;

mod router;

mod manager;

pub use app::App;

pub use web::{
    HttpMethod, HttpMethod::DELETE, HttpMethod::GET, HttpMethod::PATCH, HttpMethod::POST,
    HttpRequest, Middleware, RequestError, Response, middleware,
};

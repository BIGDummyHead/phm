mod app;

/// WEB
mod web;

mod router;

pub use web::{HttpMethod::GET, HttpMethod::POST, HttpMethod::PATCH, HttpMethod::DELETE, HttpMethod, Request, Resolution};
pub use app::App;
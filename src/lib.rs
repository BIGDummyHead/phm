pub mod app;

/// WEB
mod web;

mod router;

pub use app::App;

pub use web::{
    HttpMethod, HttpMethod::DELETE, HttpMethod::GET, HttpMethod::PATCH, HttpMethod::POST, HttpRequest,
    RequestError, Resolution,
};

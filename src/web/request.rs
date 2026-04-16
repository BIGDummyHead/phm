mod request_error;
use std::marker::PhantomData;

pub use request_error::RequestError;

/// request placeholder, holds pertinent information about the ongoign request, things related to the request should 
/// encapsulate the same lifetimes as this object.
pub struct Request {
}


impl Request {
    pub fn parse() -> Self {
        Request { }
    }
}

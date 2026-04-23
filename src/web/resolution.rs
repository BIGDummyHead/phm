use std::marker::PhantomData;

/// Resolution placeholder things will implment on top of this, should last as long as the request itself.
pub struct Resolution;

unsafe impl Send for Resolution {}
unsafe impl Sync for Resolution {}

impl Resolution {
    pub fn status(&mut self, code: i32) {}

    pub fn new() -> Self {
        Self {
        }
    }
}
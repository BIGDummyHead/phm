/// Failure
#[derive(Debug)]
pub struct RequestError {
    status: i32,
    message: Option<String>
}

impl RequestError {
    pub fn test_status(status: i32) -> Self {
        Self {
            status,
            message: None
        }
    }
}
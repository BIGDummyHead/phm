use crate::web::http_code::HttpCode;

/// Failure
#[derive(Debug)]
pub struct RequestError {
    status: HttpCode,
    message: Option<String>,
}

impl Default for RequestError {
    fn default() -> Self {
        Self {
            status: HttpCode::InternalServerError,
            message: None,
        }
    }
}

impl RequestError {
    /// Set the status of the request error
    pub fn set_status(&mut self, status: impl Into<HttpCode>) -> () {
        self.status = status.into();
    }

    /// Set the reason that the request failed
    pub fn set_message(&mut self, msg: impl Into<String>) -> () {
        self.message = Some(msg.into());
    }

    pub fn status(&self) -> &HttpCode {
        &self.status
    }

    pub fn message(&self) -> Option<&String> {
        self.message.as_ref()
    }
}

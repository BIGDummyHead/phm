use thiserror::Error;

#[derive(Debug, Error)]
pub enum RouterError {
    #[error("this route already exist!")]
    AlreadyExist,
    #[error("route name is invalid")]
    BadName,
    #[error("route did not exist")]
    NotFound
}
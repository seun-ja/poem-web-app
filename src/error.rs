use poem::{error::ResponseError, http::StatusCode};

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("User already exists")]
    AlreadyExist,
    #[error("User does not exist")]
    NonExistence,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Lock poisoned: {0}")]
    LockPoison(String),
}

impl ResponseError for ApiError {
    fn status(&self) -> poem::http::StatusCode {
        StatusCode::BAD_REQUEST
    }
}

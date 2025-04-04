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
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Expired JWTs")]
    ExpiredJWTs,
    #[error("Invalid password hash")]
    InvalidPasswordHash,
    #[error("Invalid JWT credentials: {0}")]
    InvalidJWTCredentials(#[source] anyhow::Error),
    #[error("Wrong timestamp")]
    WrongTimeStamp,
}

impl ResponseError for ApiError {
    fn status(&self) -> poem::http::StatusCode {
        match self {
            ApiError::AlreadyExist => StatusCode::BAD_REQUEST,
            ApiError::NonExistence => StatusCode::NOT_FOUND,
            ApiError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            ApiError::LockPoison(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::ExpiredJWTs => StatusCode::UNAUTHORIZED,
            ApiError::InvalidJWTCredentials(_) => StatusCode::UNAUTHORIZED,
            ApiError::InvalidPasswordHash => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::WrongTimeStamp => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

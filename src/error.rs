use poem::{error::ResponseError, http::StatusCode, IntoResponse};

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
    #[error("Token blacklisted")]
    TokenBlacklisted,
    #[error("Error parsing salt string: {0}")]
    ErrorParsingSaltString(argon2::password_hash::Error),
    #[error("Failed hashing password: {0}")]
    FailedHashingPassword(argon2::password_hash::Error),
    #[error("No token provided")]
    NoTokenProvided,
    #[error("Unable to decode claims: {0}")]
    UnableToDecodeClaims(#[source] anyhow::Error),
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
            ApiError::TokenBlacklisted => StatusCode::UNAUTHORIZED,
            ApiError::ErrorParsingSaltString(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::FailedHashingPassword(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NoTokenProvided => StatusCode::UNAUTHORIZED,
            ApiError::UnableToDecodeClaims(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn as_response(&self) -> poem::Response
    where
        Self: std::error::Error + Send + Sync + 'static,
    {
        tracing::error!("Status: {}, Message: {}", self.status(), &self.to_string());
        let mut resp = self.to_string().into_response();
        resp.set_status(self.status());
        if resp.status().as_u16() == 500 {
            resp.set_body("Internal Server Error");
        }
        resp
    }
}

use poem::{error::ResponseError, http::StatusCode, IntoResponse};

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Lock poisoned: {0}")]
    LockPoison(String),
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
    #[error("Failed hashing password: {0}")]
    FailedHashingPassword(argon2::password_hash::Error),
    #[error("Unable to decode claims: {0}")]
    UnableToDecodeClaims(#[source] anyhow::Error),
}

impl ResponseError for ApiError {
    fn status(&self) -> poem::http::StatusCode {
        match self {
            ApiError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            ApiError::LockPoison(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ExpiredJWTs => StatusCode::UNAUTHORIZED,
            ApiError::InvalidJWTCredentials(_) => StatusCode::UNAUTHORIZED,
            ApiError::InvalidPasswordHash => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::WrongTimeStamp => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::TokenBlacklisted => StatusCode::UNAUTHORIZED,
            ApiError::FailedHashingPassword(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::UnableToDecodeClaims(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn as_response(&self) -> poem::Response
    where
        Self: std::error::Error + Send + Sync + 'static,
    {
        tracing::error!(
            "Status code: {}, Message: {}",
            self.status().as_u16(),
            &self.to_string()
        );
        let mut resp = self.to_string().into_response();
        resp.set_status(self.status());
        if resp.status().as_u16() == 500 {
            resp.set_body("Internal Server Error");
        }
        resp
    }
}

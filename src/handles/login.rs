use poem_openapi::Object;
use serde::Deserialize;

use crate::{error::ApiError, schemas::LoggedUser, state::AppState};

#[derive(Debug, Deserialize, Object)]
pub struct LoginBody {
    pub email: String,
    pub password: String,
}

/// Handles user login
pub async fn login(body: LoginBody, data: &AppState) -> Result<LoggedUser, ApiError> {
    data.db
        .lock()
        .map_err(|err| ApiError::LockPoison(err.to_string()))?
        .get_user_by_email(&body.email)?
        .verify_password(&body.password, &data.hmac_secret)
}

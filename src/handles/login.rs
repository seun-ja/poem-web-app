use serde::Deserialize;

use crate::{error::ApiError, schemas::LoggedUser, state::AppState};

#[derive(Debug, Deserialize)]
pub struct LoginParameters {
    pub email: String,
    pub password: String,
}

/// Handles user login
pub async fn login(params: LoginParameters, data: &AppState) -> Result<LoggedUser, ApiError> {
    data.db
        .lock()
        .map_err(|err| ApiError::LockPoison(err.to_string()))?
        .get_user(&params.email)?
        .verify_password(&params.password, &data.hmac_secret)
}

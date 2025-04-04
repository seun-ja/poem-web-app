use poem::web::Data;
use serde::Deserialize;

use crate::{error::ApiError, schemas::LoggedUser, state::AppState};

#[derive(Debug, Deserialize)]
pub struct LoginParameters {
    pub email: String,
    pub password: String,
}

pub async fn login(params: LoginParameters, data: Data<&AppState>) -> Result<LoggedUser, ApiError> {
    data.db
        .lock()
        .map_err(|err| ApiError::LockPoison(err.to_string()))?
        .get(&params.email)?
        .verify_password(&params.password) //TODO: return JWT token
}

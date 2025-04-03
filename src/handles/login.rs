use poem::web::Data;
use serde::Deserialize;

use crate::{PasswordEncryptor, error::ApiError, schemas::User, state::AppState};

#[derive(Debug, Deserialize)]
pub struct LoginParameters {
    pub email: String,
    pub password: String,
}

pub async fn login(params: LoginParameters, data: Data<&AppState>) -> Result<User, ApiError> {
    data.db
        .lock()
        .map_err(|err| ApiError::LockPoison(err.to_string()))?
        .get(&params.email)?
        .decrypt_password(data.paraphrase.clone())
        .password_match(&params.password)
}

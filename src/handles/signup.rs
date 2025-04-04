use poem::web::Data;

use crate::{error::ApiError, schemas::NewUser, state::AppState};

pub async fn signup(params: NewUser, data: Data<&AppState>) -> Result<(), ApiError> {
    if data
        .db
        .lock()
        .map_err(|err| ApiError::LockPoison(err.to_string()))?
        .insert(params, data.passphrase.clone())
        .is_ok()
    {
        Ok(())
    } else {
        Err(ApiError::InvalidCredentials)
    }
}

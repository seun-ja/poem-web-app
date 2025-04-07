use crate::{error::ApiError, state::AppState};


/// Handles user logout
/// Blacklists JWT token
pub fn black_list_user_jwt(token: &str, data: &AppState) -> Result<(), ApiError> {
    data.db
        .lock()
        .map_err(|err| ApiError::LockPoison(err.to_string()))?
        .insert_black_list(token.to_owned());

    Ok(())
}

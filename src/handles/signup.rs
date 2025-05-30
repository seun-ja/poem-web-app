use crate::{error::ApiError, schemas::NewUser, state::AppState};

/// Handles user signup
pub async fn signup(new_user: NewUser, data: &AppState) -> Result<(), ApiError> {
    data.db
        .lock()
        .map_err(|err| ApiError::LockPoison(err.to_string()))?
        .insert_user(new_user)
}

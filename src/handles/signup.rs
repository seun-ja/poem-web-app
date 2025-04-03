use poem::web::Data;

use crate::{
    error::ApiError,
    schemas::{NewUser, User},
    state::AppState,
};

pub async fn signup(params: NewUser, data: Data<&AppState>) -> Result<User, ApiError> {
    if let Ok(user) = data
        .db
        .lock()
        .map_err(|err| ApiError::LockPoison(err.to_string()))?
        .insert(params, data.paraphrase.clone())
    {
        Ok(user.user_pub_data())
    } else {
        Err(ApiError::InvalidCredentials)
    }
}

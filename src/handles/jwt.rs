use chrono::{DateTime, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, encode};
use poem::http::HeaderValue;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;

#[derive(Deserialize, Serialize)]
struct Claims {
    sub: String,
    name: String,
    exp: usize,
}

pub fn handle_jwt_token(token: &HeaderValue, secret: &str) -> Result<String, ApiError> {
    if let Some(jwt) = extract_header_value(token) {
        if let Ok(claim) = jsonwebtoken::decode::<Claims>(
            jwt,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        ) {
            // Check if the token is expired
            if let Some(expiry) = DateTime::from_timestamp(claim.claims.exp as i64, 0) {
                let now = chrono::Utc::now();
                if now > expiry {
                    return Err(ApiError::ExpiredJWTs);
                }
            }
            Ok(claim.claims.name)
        } else {
            Err(ApiError::InvalidCredentials)
        }
    } else {
        Err(ApiError::InvalidCredentials)
    }
}

pub fn extract_header_value(token: &HeaderValue) -> Option<&str> {
    token.to_str().ok()
}

pub fn create_jwt(uid: &str, secret: &str) -> Result<String, ApiError> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(2))
        .map(|dt| dt.timestamp())
        .ok_or(ApiError::WrongTimeStamp)?;

    let claims = Claims {
        sub: uid.to_owned(),
        exp: expiration as usize,
        name: "Poem".to_string(),
    };

    let header = Header::new(Algorithm::HS256);
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| ApiError::InvalidJWTCredentials(e.into()))
}

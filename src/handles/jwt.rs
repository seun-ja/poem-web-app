use chrono::{DateTime, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use poem::http::HeaderValue;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;

/// JWT claims
#[derive(Deserialize, Serialize)]
struct Claims {
    sub: String,
    name: String,
    exp: usize,
}

/// JWT token decoder
pub fn handle_jwt_token(jwt: &str, hmac_secret: &str) -> Result<String, ApiError> {
    jsonwebtoken::decode::<Claims>(
        jwt,
        &DecodingKey::from_secret(hmac_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|err| ApiError::UnableToDecodeClaims(err.into()))
    .map(|claims| {
        let _expiry = DateTime::from_timestamp(claims.claims.exp as i64, 0)
            .ok_or(ApiError::ExpiredJWTs)
            .map(|e| {
                if e < Utc::now() {
                    Err(ApiError::ExpiredJWTs)
                } else {
                    Ok(())
                }
            });
        claims.claims.name
    })
}

/// JWT header extractor
pub fn extract_header_value(token: &HeaderValue) -> Option<&str> {
    token
        .to_str()
        .ok()
        .map(|t| t.split(' ').collect::<Vec<&str>>()[1])
}

/// JWT token creator
/// Creates a JWT token with a 1-day expiration time
/// and the provided user ID and name.
pub fn create_jwt(uid: &str, name: &str, hmac_secret: &str) -> Result<String, ApiError> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(1))
        .map(|dt| dt.timestamp())
        .ok_or(ApiError::WrongTimeStamp)?;

    let claims = Claims {
        sub: uid.to_owned(),
        exp: expiration as usize,
        name: name.to_owned(),
    };

    let header = Header::new(Algorithm::HS256);
    jsonwebtoken::encode(
        &header,
        &claims,
        &EncodingKey::from_secret(hmac_secret.as_bytes()),
    )
    .map_err(|e| ApiError::InvalidJWTCredentials(e.into()))
}

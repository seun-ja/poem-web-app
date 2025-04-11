pub mod jwt;
pub mod login;
pub mod logout;
pub mod signup;

use std::sync::Arc;

use jwt::{extract_header_value, handle_jwt_token};
use logout::black_list_user_jwt;
use poem::web::{Data, Json as JsonBody};
use poem_openapi::{
    payload::{Json, PlainText},
    OpenApi,
};

use crate::{
    error::ApiError,
    handles::{
        login::{login, LoginBody},
        signup::signup,
    },
    schemas::{LoggedUser, NewUser},
    state::AppState,
};

/// OpenAPI documentation
#[derive(Debug)]
pub struct OpenApiDoc;

#[OpenApi]
impl OpenApiDoc {
    #[oai(path = "/ping", method = "get")]
    async fn health(&self) -> PlainText<String> {
        PlainText("pong".to_string())
    }

    #[oai(path = "/login", method = "post")]
    async fn login(
        &self,
        JsonBody(body): JsonBody<LoginBody>,
        req: &poem::Request,
        Data(data): Data<&Arc<AppState>>,
    ) -> poem::Result<Json<LoggedUser>> {
        if let Some(header_value) = req.headers().get("Authorization") {
            data.db
                .lock()
                .map_err(|err| ApiError::LockPoison(err.to_string()))?
                .check_token_black_listed(extract_header_value(header_value)?)?
        }

        login(body, data).await.map_err(|err| err.into()).map(Json)
    }

    #[oai(path = "/signup", method = "post")]
    async fn signup(
        &self,
        JsonBody(body): JsonBody<NewUser>,
        Data(data): Data<&Arc<AppState>>,
    ) -> poem::Result<()> {
        signup(body, data).await.map_err(|err| err.into())
    }

    #[oai(path = "/protected", method = "get")]
    async fn protected(
        &self,
        req: &poem::Request,
        Data(data): Data<&Arc<AppState>>,
    ) -> poem::Result<PlainText<String>> {
        if let Some(header_value) = req.headers().get("Authorization") {
            let token = extract_header_value(header_value)?;
            let db = data
                .db
                .lock()
                .map_err(|err| ApiError::LockPoison(err.to_string()))?;

            db.check_token_black_listed(token)?;

            handle_jwt_token(token, &data.hmac_secret).map(|claims| {
                db.assert_user_exists(&claims.sub)?;
                Ok(PlainText(format!("Access granted, user: {}", claims.name)))
            })?
        } else {
            Err(ApiError::Unauthorized.into())
        }
    }

    #[oai(path = "/logout", method = "post")]
    async fn logout(
        &self,
        req: &poem::Request,
        Data(data): Data<&Arc<AppState>>,
    ) -> poem::Result<()> {
        if let Some(header_value) = req.headers().get("Authorization") {
            black_list_user_jwt(extract_header_value(header_value)?, data)
                .map_err(|err| err.into())
                .map(|_| ())
        } else {
            Err(ApiError::Unauthorized.into())
        }
    }
}

pub mod jwt;
pub mod login;
pub mod logout;
pub mod signup;

use std::sync::Arc;

use jwt::handle_jwt_token;
use logout::black_list_user_jwt;
use poem::web::Data;
use poem_openapi::{
    auth::Bearer,
    payload::{Json, PlainText},
    OpenApi, SecurityScheme,
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

#[derive(SecurityScheme)]
#[oai(ty = "bearer", key_name = "Authorization")]
struct BearerAuth(Bearer);

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
        Json(body): Json<LoginBody>,
        BearerAuth(bearer): BearerAuth,
        Data(data): Data<&Arc<AppState>>,
    ) -> poem::Result<Json<LoggedUser>> {
        data.db
            .lock()
            .map_err(|err| ApiError::LockPoison(err.to_string()))?
            .check_token_black_listed(&bearer.token)?;

        login(body, data).await.map_err(|err| err.into()).map(Json)
    }

    #[oai(path = "/signup", method = "post")]
    async fn signup(
        &self,
        Json(body): Json<NewUser>,
        Data(data): Data<&Arc<AppState>>,
    ) -> poem::Result<()> {
        signup(body, data).await.map_err(|err| err.into())
    }

    #[oai(path = "/protected", method = "get")]
    async fn protected(
        &self,
        BearerAuth(bearer): BearerAuth,
        Data(data): Data<&Arc<AppState>>,
    ) -> poem::Result<PlainText<String>> {
        let db = data
            .db
            .lock()
            .map_err(|err| ApiError::LockPoison(err.to_string()))?;

        db.check_token_black_listed(&bearer.token)?;

        handle_jwt_token(&bearer.token, &data.hmac_secret).map(|claims| {
            db.assert_user_exists(&claims.sub)?;
            Ok(PlainText(format!("Access granted, user: {}", claims.name)))
        })?
    }

    #[oai(path = "/logout", method = "post")]
    async fn logout(
        &self,
        BearerAuth(bearer): BearerAuth,
        Data(data): Data<&Arc<AppState>>,
    ) -> poem::Result<()> {
        black_list_user_jwt(&bearer.token, data)
            .map_err(|err| err.into())
            .map(|_| ())
    }
}

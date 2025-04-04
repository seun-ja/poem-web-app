pub mod jwt;
pub mod login;
pub mod logout;
pub mod signup;

use jwt::handle_jwt_token;
use poem::web::{Data, Query};
use poem_openapi::{
    OpenApi,
    payload::{Json, PlainText},
};

use crate::{
    error::ApiError,
    handles::{
        login::{LoginParameters, login},
        signup::signup,
    },
    schemas::{LoggedUser, NewUser},
    state::AppState,
};

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
        params: poem::Result<Query<LoginParameters>>,
        data: Data<&AppState>,
    ) -> poem::Result<Json<LoggedUser>> {
        if let Ok(params) = params {
            login(params.0, data)
                .await
                .map_err(|err| err.into())
                .map(Json)
        } else {
            Err(ApiError::NonExistence.into())
        }
    }

    #[oai(path = "/signup", method = "post")]
    async fn signup(
        &self,
        params: poem::Result<Query<NewUser>>,
        data: Data<&AppState>,
    ) -> poem::Result<()> {
        if let Ok(params) = params {
            if signup(params.0, data).await.is_ok() {
                Ok(())
            } else {
                Err(ApiError::NonExistence.into())
            }
        } else {
            Err(ApiError::NonExistence.into())
        }
    }

    #[oai(path = "/protected", method = "get")]
    async fn protected(
        &self,
        req: &poem::Request,
        data: Data<&AppState>,
    ) -> poem::Result<PlainText<String>> {
        if let Some(token) = req.headers().get("Authorization") {
            if let Ok(name) = handle_jwt_token(token, &data.secret) {
                Ok(PlainText(format!("Access granted, user: {}", name)))
            } else {
                Err(ApiError::Unauthorized.into())
            }
        } else {
            Err(ApiError::Unauthorized.into())
        }
    }

    #[oai(path = "/logout", method = "post")]
    async fn logout(&self) {}
}

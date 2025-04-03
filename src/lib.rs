pub mod db;
pub mod error;
pub mod handles;
pub mod state;

pub use docs::OpenApiDoc;

mod schemas {
    use poem_openapi::Object;
    use serde::Deserialize;
    use uuid::Uuid;

    use crate::{PasswordEncryptor, error::ApiError};

    #[derive(Debug, Clone, Deserialize, Object)]
    pub struct UserDbSchema {
        pub id: Uuid,
        pub encrypted_password: String,
        pub email: String,
        pub first_name: String,
        pub last_name: String,
    }

    impl UserDbSchema {
        pub fn user_pub_data(&self) -> User {
            User {
                id: self.id,
                email: self.email.clone(),
                first_name: self.first_name.clone(),
                last_name: self.last_name.clone(),
            }
        }

        pub fn password_match(&self, password: &str) -> Result<User, ApiError> {
            if self.encrypted_password == password {
                Ok(self.user_pub_data())
            } else {
                Err(ApiError::InvalidCredentials)
            }
        }
    }

    #[derive(Debug, Clone, Deserialize, Object)]
    pub struct User {
        id: Uuid,
        email: String,
        first_name: String,
        last_name: String,
    }

    impl UserDbSchema {
        pub fn get_id(&self) -> Uuid {
            self.id
        }

        pub fn get_email(&self) -> &str {
            &self.email
        }
    }

    impl PasswordEncryptor for UserDbSchema {
        fn encrypt_password(&self, _pass_phrase: String) -> Self {
            todo!()
        }

        fn decrypt_password(&self, _pass_phrase: String) -> Self {
            todo!()
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct NewUser {
        pub email: String,
        pub password: String,
        pub first_name: String,
        pub last_name: String,
    }

    impl From<NewUser> for UserDbSchema {
        fn from(val: NewUser) -> Self {
            UserDbSchema {
                id: uuid::Uuid::new_v4(),
                encrypted_password: val.password,
                email: val.email,
                first_name: val.first_name,
                last_name: val.last_name,
            }
        }
    }
}

pub trait PasswordEncryptor {
    fn encrypt_password(&self, pass_phrase: String) -> Self;
    fn decrypt_password(&self, pass_phrase: String) -> Self;
}

mod docs {
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
        schemas::{NewUser, User},
        state::AppState,
    };

    pub struct OpenApiDoc;

    #[OpenApi]
    impl OpenApiDoc {
        #[oai(path = "/ping", method = "get")]
        async fn testing(&self) -> PlainText<String> {
            PlainText("pong".to_string())
        }

        #[oai(path = "/login", method = "post")]
        async fn login(
            &self,
            params: poem::Result<Query<LoginParameters>>,
            data: Data<&AppState>,
        ) -> poem::Result<Json<User>> {
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
        ) -> poem::Result<Json<User>> {
            if let Ok(params) = params {
                signup(params.0, data)
                    .await
                    .map_err(|err| err.into())
                    .map(Json)
            } else {
                Err(ApiError::NonExistence.into())
            }
        }
    }
}

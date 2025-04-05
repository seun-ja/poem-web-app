pub mod db;
pub mod error;
pub mod handles;
pub mod state;

mod schemas {
    use argon2::{
        Argon2, PasswordHash, PasswordVerifier as _,
        password_hash::{PasswordHasher, SaltString},
    };
    use base64::{Engine, engine::general_purpose};
    use poem_openapi::Object;
    use serde::Deserialize;
    use uuid::Uuid;

    use crate::{error::ApiError, handles::jwt::create_jwt};

    #[derive(Debug, Clone, Deserialize, Object)]
    pub struct UserDbSchema {
        pub id: Uuid,
        pub encrypted_password: String,
        pub email: String,
        pub first_name: String,
        pub last_name: String,
    }

    impl UserDbSchema {
        pub fn verify_password(
            &self,
            password: &str,
            hmac_secret: &str,
        ) -> Result<LoggedUser, ApiError> {
            let expected_password_hash = PasswordHash::new(&self.encrypted_password)
                .map_err(|_| ApiError::InvalidPasswordHash)?;

            Argon2::default()
                .verify_password(password.as_bytes(), &expected_password_hash)
                .map_err(|_| ApiError::InvalidCredentials)?;

            Ok(LoggedUser {
                token: create_jwt(&self.id.to_string(), &self.first_name, hmac_secret)?,
            })
        }
    }

    impl UserDbSchema {
        pub fn get_email(&self) -> &str {
            &self.email
        }

        pub fn encrypt_password(&mut self, pass_phrase: String) -> Result<Self, ApiError> {
            let salt = general_purpose::STANDARD_NO_PAD.encode(pass_phrase);
            self.encrypted_password = Argon2::default()
                .hash_password(
                    self.encrypted_password.as_bytes(),
                    &SaltString::from_b64(&salt).map_err(ApiError::ErrorParsingSaltString)?,
                )
                .map_err(ApiError::FailedHashingPassword)?
                .to_string();
            Ok(self.clone())
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

    #[derive(Debug, Clone, Deserialize, Object)]
    pub struct LoggedUser {
        token: String,
    }
}

pub mod tracing {
    use tracing_subscriber::{EnvFilter, layer::SubscriberExt as _};

    pub fn init(env_filter: &str) {
        let env_filter = EnvFilter::from(env_filter);
        let subscriber = tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer());
        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set global default subscriber");
    }
}

// pub trait PasswordEncryptor {
//     fn encrypt_password(&mut self, pass_phrase: String) -> Result<Self, ApiError>;
// }

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
        pub fn logged_user(&self) -> LoggedUser {
            LoggedUser { id: self.id }
        }

        pub fn verify_password(&self, password: &str) -> Result<LoggedUser, ApiError> {
            let expected_password_hash = PasswordHash::new(&self.encrypted_password)
                .map_err(|_| ApiError::InvalidPasswordHash)?;

            Argon2::default()
                .verify_password(password.as_bytes(), &expected_password_hash)
                .map_err(|_| ApiError::InvalidCredentials)
                .map(|_| self.logged_user())
        }
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
        fn encrypt_password(&mut self, pass_phrase: String) -> Self {
            let password_hash = Argon2::default()
                .hash_password(
                    self.encrypted_password.as_bytes(),
                    &SaltString::from_b64(&general_purpose::STANDARD.encode(pass_phrase)).unwrap(),
                )
                .unwrap()
                .to_string();
            self.encrypted_password = password_hash;
            self.clone()
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
        id: Uuid,
    }
}

pub trait PasswordEncryptor {
    fn encrypt_password(&mut self, pass_phrase: String) -> Self;
}

pub mod db;
pub mod error;
pub mod handles;
pub mod state;

mod schemas {
    use argon2::{
        password_hash::{PasswordHasher, SaltString},
        Argon2, PasswordHash, PasswordVerifier as _,
    };
    use poem_openapi::Object;
    use serde::Deserialize;
    use uuid::Uuid;

    use crate::{error::ApiError, handles::jwt::create_jwt};

    /// Represents a user in the database
    #[derive(Debug, Clone, Deserialize, Object)]
    pub struct UserDbSchema {
        pub id: Uuid,
        pub encrypted_password: String,
        pub email: String,
        pub first_name: String,
        pub last_name: String,
    }

    impl UserDbSchema {
        /// Verifies users credentials
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
        /// Gets user's email
        pub fn get_email(&self) -> &str {
            &self.email
        }

        /// Encrypts user's password
        pub fn encrypt_password(&mut self) -> Result<Self, ApiError> {
            let salt = SaltString::generate(&mut rand::thread_rng());
            self.encrypted_password = Argon2::default()
                .hash_password(self.encrypted_password.as_bytes(), &salt)
                .map_err(ApiError::FailedHashingPassword)?
                .to_string();
            Ok(self.clone())
        }
    }

    /// Represents a new user
    #[derive(Debug, Clone, Deserialize, Object)]
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

    /// User's successful login information
    /// Contains JWT token
    #[derive(Debug, Clone, Deserialize, Object)]
    pub struct LoggedUser {
        token: String,
    }
}

pub mod tracing {
    use tracing_subscriber::{layer::SubscriberExt as _, EnvFilter};

    /// Initializes the tracing subscriber with the given environment filter.
    pub fn init(env_filter: &str) {
        let env_filter = EnvFilter::from(env_filter);
        let subscriber = tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer());
        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set global default subscriber");
    }
}

use std::collections::{btree_map, BTreeMap};

use crate::{
    error::ApiError,
    schemas::{NewUser, UserDbSchema},
};

/// In-memory database for user management
/// This is a simple implementation of a database that stores user information
pub struct InMemDatabase {
    pub(crate) user_db: BTreeMap<String, UserDbSchema>,
    pub(crate) black_listed_db: Vec<String>,
}

impl InMemDatabase {
    /// Stores a `NewUser` to database
    pub fn insert_user(&mut self, new_user: NewUser) -> Result<(), ApiError> {
        let user: UserDbSchema = new_user.into();
        if let btree_map::Entry::Vacant(e) = self.user_db.entry(user.get_email().to_string()) {
            e.insert(user.clone().encrypt_password()?);
            Ok(())
        } else {
            Err(ApiError::AlreadyExist)
        }
    }

    /// Get a `UserDbSchema` from database
    pub fn get_user_by_email(&self, email: &str) -> Result<UserDbSchema, ApiError> {
        if let Some(user) = self.user_db.get(email) {
            let value = user.clone();
            Ok(value)
        } else {
            Err(ApiError::NonExistence)
        }
    }

    pub fn assert_user_exists(&self, id: &str) -> Result<(), ApiError> {
        if self.user_db.values().any(|user| user.id.to_string() == id) {
            Ok(())
        } else {
            Err(ApiError::NonExistence)
        }
    }

    /// Stores a blacklisted token to database
    pub fn insert_black_list(&mut self, new_token: String) {
        self.black_listed_db.push(new_token)
    }

    /// Checks blacklisted token existence in database
    pub fn check_token_black_listed(&self, token: &str) -> Result<(), ApiError> {
        for blacklisted_token in &self.black_listed_db {
            if blacklisted_token == token {
                return Err(ApiError::TokenBlacklisted);
            }
        }
        Ok(())
    }
}

use std::collections::{btree_map, BTreeMap};

use crate::{
    error::ApiError,
    schemas::{NewUser, UserDbSchema},
};

pub struct InMemDatabase {
    pub(crate) user_db: BTreeMap<String, UserDbSchema>,
    pub(crate) black_listed_db: Vec<String>,
}

impl InMemDatabase {
    pub fn insert_user(&mut self, new_user: NewUser, pass_phrase: String) -> Result<(), ApiError> {
        let user: UserDbSchema = new_user.into();
        if let btree_map::Entry::Vacant(e) = self.user_db.entry(user.get_email().to_string()) {
            e.insert(user.clone().encrypt_password(pass_phrase)?);
            Ok(())
        } else {
            Err(ApiError::AlreadyExist)
        }
        // if self.mem.contains_key(&user.id) {
        //     Err(ApiError::AlreadyExist)
        // } else {
        //     self.mem.insert(user.id, user);
        //     Ok(())
        // }
    }

    pub fn get_user(&self, email: &str) -> Result<UserDbSchema, ApiError> {
        if let Some(user) = self.user_db.get(email) {
            let value = user.clone();
            Ok(value)
        } else {
            Err(ApiError::NonExistence)
        }
    }

    pub fn insert_black_list(&mut self, new_token: String) {
        self.black_listed_db.push(new_token)
    }

    pub fn check_token_black_listed(&self, token: &str) -> bool {
        for blacklisted_token in &self.black_listed_db {
            if blacklisted_token == token {
                return true;
            }
        }
        false
    }
}

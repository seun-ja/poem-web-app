use std::collections::{BTreeMap, btree_map};

use crate::{
    PasswordEncryptor,
    error::ApiError,
    schemas::{NewUser, UserDbSchema},
};

pub struct InMemDatabase {
    pub(crate) mem: BTreeMap<String, UserDbSchema>,
}

impl InMemDatabase {
    pub fn insert(
        &mut self,
        new_user: NewUser,
        pass_phrase: String,
    ) -> Result<UserDbSchema, ApiError> {
        let user: UserDbSchema = new_user.into();
        if let btree_map::Entry::Vacant(e) = self.mem.entry(user.get_email().to_string()) {
            e.insert(user.clone().encrypt_password(pass_phrase));
            Ok(user)
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

    pub fn get(&self, email: &str) -> Result<UserDbSchema, ApiError> {
        if let Some(user) = self.mem.get(email) {
            let value = user.clone();
            Ok(value)
        } else {
            Err(ApiError::NonExistence)
        }
    }
}

use std::{collections::BTreeMap, sync::Mutex};

use serde::Deserialize;

use crate::db::InMemDatabase;

pub struct AppState {
    pub db: Mutex<InMemDatabase>,
    pub passphrase: String,
    pub hmac_secret: String,
}

impl AppState {
    pub fn build(config: Config) -> poem::Result<Self> {
        let db = Mutex::new(InMemDatabase {
            user_db: BTreeMap::new(),
            black_listed_db: Vec::new(),
        });

        Ok(Self {
            db,
            passphrase: config.passphrase.clone(),
            hmac_secret: config.hmac_secret.clone(),
        })
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub passphrase: String,
    pub hmac_secret: String,
    pub log_level: String,
}

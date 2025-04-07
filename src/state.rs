use std::{collections::BTreeMap, sync::Mutex};

use serde::Deserialize;

use crate::db::InMemDatabase;

/// Application state
pub struct AppState {
    pub db: Mutex<InMemDatabase>,
    pub hmac_secret: String,
}

impl AppState {
    /// Creates a new instance of `AppState`
    pub fn build(config: Config) -> poem::Result<Self> {
        let db = Mutex::new(InMemDatabase {
            user_db: BTreeMap::new(),
            black_listed_db: Vec::new(),
        });

        Ok(Self {
            db,
            hmac_secret: config.hmac_secret.clone(),
        })
    }
}

/// Configuration for the application
#[derive(Deserialize)]
pub struct Config {
    pub hmac_secret: String,
    pub log_level: String,
}

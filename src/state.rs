use std::{collections::BTreeMap, sync::Mutex};

use serde::Deserialize;

use crate::db::InMemDatabase;

pub struct AppState {
    pub db: Mutex<InMemDatabase>,
    pub passphrase: String,
    pub secret: String,
    pub black_listed_jwt: Vec<String>,
}

impl AppState {
    pub fn build(config: Config) -> poem::Result<Self> {
        let db = Mutex::new(InMemDatabase {
            mem: BTreeMap::new(),
        });

        Ok(Self {
            db,
            passphrase: config.passphrase.clone(),
            secret: config.secret.clone(),
            black_listed_jwt: config.get_black_listed_jwt(),
        })
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub passphrase: String,
    pub secret: String,
    black_listed_jwt: String,
}

impl Config {
    fn get_black_listed_jwt(&self) -> Vec<String> {
        self.black_listed_jwt.split(',').map(String::from).collect()
    }
}

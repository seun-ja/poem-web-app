use std::{collections::BTreeMap, sync::Mutex};

use crate::db::InMemDatabase;

pub struct AppState {
    pub db: Mutex<InMemDatabase>,
    pub paraphrase: String,
}

impl AppState {
    pub fn build() -> poem::Result<Self> {
        let db = Mutex::new(InMemDatabase {
            mem: BTreeMap::new(),
        });

        Ok(Self {
            db,
            paraphrase: "Hello, World!".to_string(), // TODO: move to env
        })
    }
}

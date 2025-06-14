use std::env;

pub struct Config {
    pub db_url: String,
}

impl Config {
    pub fn new() -> Self {
        let db_url = env::var("DB_URL").unwrap_or(String::from("sqlite:dev.db"));

        Config { db_url }
    }
}

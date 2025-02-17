use std::env;

pub struct Config {
    pub db_url:         String,
    pub max_conn:       u32,
    pub allowed_origin: String,
    pub secret:         String,
}

impl Config {
    pub fn new() -> Config {
        Config {
            db_url:         env::var("DATABASE_URL").unwrap(),
            max_conn:       env::var("DATABASE_MAX_CONN").unwrap().parse::<u32>().unwrap(),
            allowed_origin: env::var("ALLOWED_ORIGIN").unwrap(),
            secret:         env::var("SECRET").unwrap(),
        }
    }
}

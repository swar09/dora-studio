use std::env;

pub struct Config {
    pub port: u16,
    pub db_path: String,
}

impl Config {
    pub fn load() -> Self {
        let home = env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        Self {
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            db_path: env::var("DB_PATH").unwrap_or_else(|_| format!("{}/.dora/store", home)),
        }
    }
}

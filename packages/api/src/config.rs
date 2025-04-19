use std::env;
use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_port: u16,
    pub jwt_secret: String,
}

impl Config {
    fn new() -> Self {
        Config {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            server_port: env::var("PORT")
                .ok()
                .and_then(|port| port.parse().ok())
                .unwrap_or(3000),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your_default_secret_key".to_string()),
        }
    }

    pub fn get() -> &'static Config {
        static CONFIG: OnceLock<Config> = OnceLock::new();
        CONFIG.get_or_init(|| Config::new())
    }
}

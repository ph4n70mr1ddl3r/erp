use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub jwt_secret: String,
    pub jwt_expiration: i64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: "sqlite:erp.db?mode=rwc".to_string(),
            server_host: "127.0.0.1".to_string(),
            server_port: 3000,
            jwt_secret: "your-super-secret-jwt-key-change-in-production".to_string(),
            jwt_expiration: 24,
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite:erp.db?mode=rwc".to_string()),
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-super-secret-jwt-key-change-in-production".to_string()),
            jwt_expiration: env::var("JWT_EXPIRATION")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(24),
        }
    }
}

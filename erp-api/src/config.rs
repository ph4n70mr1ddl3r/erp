use std::env;

const MIN_JWT_SECRET_LENGTH: usize = 32;

fn generate_dev_secret() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let secret: String = (0..64)
        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
        .collect();
    format!("dev-{}", secret)
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub jwt_secret: String,
    pub jwt_expiration: i64,
}

#[derive(Debug)]
pub struct ConfigError(String);

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ConfigError {}

impl Default for Config {
    fn default() -> Self {
        let secret = generate_dev_secret();
        eprintln!("WARNING: Using generated dev secret. Set JWT_SECRET for production.");
        Self {
            database_url: "sqlite:erp.db?mode=rwc".to_string(),
            server_host: "127.0.0.1".to_string(),
            server_port: 3000,
            jwt_secret: secret,
            jwt_expiration: 24,
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        let production_mode = env::var("ENVIRONMENT").unwrap_or_default() == "production";

        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
            if production_mode {
                eprintln!("ERROR: JWT_SECRET environment variable must be set in production mode");
                std::process::exit(1);
            }
            generate_dev_secret()
        });

        if jwt_secret.len() < MIN_JWT_SECRET_LENGTH {
            if production_mode {
                eprintln!(
                    "ERROR: JWT_SECRET must be at least {} characters long in production mode",
                    MIN_JWT_SECRET_LENGTH
                );
                std::process::exit(1);
            }
            eprintln!(
                "WARNING: JWT_SECRET is shorter than {} characters. This is insecure for production.",
                MIN_JWT_SECRET_LENGTH
            );
        }

        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite:erp.db?mode=rwc".to_string()),
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var("SERVER_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            jwt_secret,
            jwt_expiration: env::var("JWT_EXPIRATION")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(24),
        }
    }
}

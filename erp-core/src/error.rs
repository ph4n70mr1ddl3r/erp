use thiserror::Error;
use tracing::warn;
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Entity not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Business rule violation: {0}")]
    BusinessRule(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl Error {
    pub fn not_found(entity: &str, id: &str) -> Self {
        Error::NotFound(format!("{} with id '{}'", entity, id))
    }

    pub fn validation(msg: impl Into<String>) -> Self {
        Error::Validation(msg.into())
    }

    pub fn business_rule(msg: impl Into<String>) -> Self {
        Error::BusinessRule(msg.into())
    }

    pub fn unauthorized(msg: &str) -> Self {
        warn!("Unauthorized access attempt: {}", msg);
        Error::Unauthorized
    }

    pub fn internal(msg: impl Into<String> + std::fmt::Display) -> Self {
        Error::Internal(anyhow::anyhow!("{}", msg))
    }
}

pub fn parse_uuid(s: &str, field_name: &str) -> Result<Uuid> {
    Uuid::parse_str(s)
        .map_err(|e| Error::internal(format!("Invalid UUID for {}: {} - {}", field_name, s, e)))
}

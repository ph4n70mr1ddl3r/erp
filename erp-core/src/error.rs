use thiserror::Error;

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
        Error::Validation(msg.to_string())
    }

    pub fn internal(msg: impl Into<String> + std::fmt::Display) -> Self {
        Error::Internal(anyhow::anyhow!("{}", msg))
    }
}

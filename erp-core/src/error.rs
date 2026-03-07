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
        .map_err(|e| Error::validation(format!("Invalid UUID for {}: {} - {}", field_name, s, e)))
}

pub fn parse_uuid_opt(s: Option<&str>, field_name: &str) -> Result<Option<Uuid>> {
    match s {
        Some(s) => Ok(Some(parse_uuid(s, field_name)?)),
        None => Ok(None),
    }
}

pub fn parse_datetime(s: &str, field_name: &str) -> Result<chrono::DateTime<chrono::Utc>> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .map_err(|e| Error::validation(format!("Invalid DateTime for {}: {} - {}", field_name, s, e)))
}

pub fn parse_datetime_opt(s: Option<&str>, field_name: &str) -> Result<Option<chrono::DateTime<chrono::Utc>>> {
    match s {
        Some(s) => Ok(Some(parse_datetime(s, field_name)?)),
        None => Ok(None),
    }
}

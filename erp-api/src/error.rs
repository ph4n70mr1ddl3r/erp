use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use erp_core::Error;
use serde_json::json;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug)]
pub struct ApiError(pub Error);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self.0 {
            Error::NotFound(_) => (StatusCode::NOT_FOUND, self.0.to_string()),
            Error::Validation(_) => (StatusCode::BAD_REQUEST, self.0.to_string()),
            Error::BusinessRule(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.0.to_string()),
            Error::Conflict(_) => (StatusCode::CONFLICT, self.0.to_string()),
            Error::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            Error::Database(e) => {
                tracing::error!("Database error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            Error::Internal(e) => {
                tracing::error!("Internal error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

impl From<Error> for ApiError {
    fn from(err: Error) -> Self {
        ApiError(err)
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        ApiError(Error::Database(err))
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        ApiError(Error::Internal(err.to_string()))
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError(Error::Validation(err.to_string()))
    }
}

impl From<std::io::Error> for ApiError {
    fn from(err: std::io::Error) -> Self {
        ApiError(Error::Internal(err.to_string()))
    }
}

impl From<uuid::Error> for ApiError {
    fn from(err: uuid::Error) -> Self {
        ApiError(Error::Validation(err.to_string()))
    }
}

impl From<validator::ValidationErrors> for ApiError {
    fn from(err: validator::ValidationErrors) -> Self {
        ApiError(Error::Validation(err.to_string()))
    }
}

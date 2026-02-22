pub mod auth;
pub mod finance;
pub mod inventory;
pub mod sales;
pub mod purchasing;
pub mod manufacturing;
pub mod hr;

use axum::{extract::State, Json};
use serde_json::json;
use crate::db::AppState;

pub async fn health(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "status": "healthy", "service": "erp-api" }))
}

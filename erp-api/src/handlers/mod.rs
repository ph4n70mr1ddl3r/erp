pub mod pcard;
pub mod loyalty;
pub mod partner;
pub mod territory;
pub mod lease;
pub mod bank;
pub mod predictive;
pub mod mrp;
pub mod eam;

use axum::{extract::State, Json};
use serde_json::json;
use crate::db::AppState;

pub async fn health(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "status": "healthy", "service": "erp-api" }))
}

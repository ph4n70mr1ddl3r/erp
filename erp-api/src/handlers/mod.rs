pub mod pcard;
pub mod loyalty;
pub mod partner;
pub mod territory;
pub mod lease;
pub mod bank;
pub mod predictive;
pub mod mrp;
pub mod eam;
pub mod bi;
pub mod i18n;
pub mod push;
pub mod bpm;
pub mod graphql;
pub mod assistant;
pub mod ocr;
pub mod fraud;
pub mod processmining;
pub mod promotions;
pub mod approval_workflow;
pub mod credit;

use axum::{extract::State, Json};
use serde_json::json;
use crate::db::AppState;

pub async fn health(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "status": "healthy", "service": "erp-api" }))
}

pub mod audit;
pub mod attachment;
pub mod auth;
pub mod compliance;
pub mod config;
pub mod documents;
pub mod extended;
pub mod finance;
pub mod import_export;
pub mod inventory;
pub mod pricing;
pub mod purchasing;
pub mod manufacturing;
pub mod hr;
pub mod workflow;
pub mod sales;
pub mod service;
pub mod assets;
pub mod projects;
pub mod returns;
pub mod pos;
pub mod ecommerce;
pub mod tax;
pub mod reports;
pub mod barcode;
pub mod ai;
pub mod portals;
pub mod iot;
pub mod automation;
pub mod notifications;
pub mod webhooks;
pub mod jobs;
pub mod integration;
pub mod templates;
pub mod rules;
pub mod sourcing;
pub mod company;
pub mod subscription;
pub mod shipping;
pub mod payments;
pub mod risk;

use axum::{extract::State, Json};
use serde_json::json;
use crate::db::AppState;

pub async fn health(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "status": "healthy", "service": "erp-api" }))
}

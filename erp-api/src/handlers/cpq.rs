use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use crate::db::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTemplateRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub product_id: Option<Uuid>,
    pub base_price: i64,
    pub min_margin_percent: f64,
    pub max_discount_percent: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateConfigurationRequest {
    pub template_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub base_price: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuoteRequest {
    pub configuration_id: Uuid,
    pub customer_id: Uuid,
    pub opportunity_id: Option<Uuid>,
    pub valid_until: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/templates", get(list_templates).post(create_template))
        .route("/templates/:id", get(get_template))
        .route("/configurations", get(list_configurations).post(create_configuration))
        .route("/configurations/:id", get(get_configuration))
        .route("/configurations/:id/price", post(calculate_price))
        .route("/quotes", get(list_quotes).post(create_quote))
        .route("/quotes/:id", get(get_quote))
        .route("/quotes/:id/approve", post(approve_quote))
}

async fn list_templates(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "templates": [], "total": 0 }))
}

async fn create_template(
    State(_state): State<AppState>,
    Json(_req): Json<CreateTemplateRequest>,
) -> Json<serde_json::Value> {
    Json(json!({ "id": Uuid::new_v4().to_string(), "message": "Template created" }))
}

async fn get_template(Path(_id): Path<Uuid>, State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "template": null }))
}

async fn list_configurations(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "configurations": [], "total": 0 }))
}

async fn create_configuration(
    State(_state): State<AppState>,
    Json(_req): Json<CreateConfigurationRequest>,
) -> Json<serde_json::Value> {
    Json(json!({ "id": Uuid::new_v4().to_string(), "message": "Configuration created" }))
}

async fn get_configuration(Path(_id): Path<Uuid>, State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "configuration": null }))
}

async fn calculate_price(Path(_id): Path<Uuid>, State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "base_price": 0, "configured_price": 0, "breakdown": [] }))
}

async fn list_quotes(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "quotes": [], "total": 0 }))
}

async fn create_quote(
    State(_state): State<AppState>,
    Json(_req): Json<CreateQuoteRequest>,
) -> Json<serde_json::Value> {
    Json(json!({ "id": Uuid::new_v4().to_string(), "message": "Quote created" }))
}

async fn get_quote(Path(_id): Path<Uuid>, State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "quote": null }))
}

async fn approve_quote(Path(_id): Path<Uuid>, State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "message": "Quote approved" }))
}

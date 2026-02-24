use axum::{
    extract::{Path, State},
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use crate::db::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePlanRequest {
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub commission_type: String,
    pub basis: String,
    pub frequency: String,
    pub default_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalculateCommissionRequest {
    pub sales_rep_id: Uuid,
    pub plan_id: Uuid,
    pub period_start: String,
    pub period_end: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuotaRequest {
    pub sales_rep_id: Uuid,
    pub quota_type: String,
    pub period: String,
    pub year: i32,
    pub target_amount: i64,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/plans", get(list_plans).post(create_plan))
        .route("/plans/:id", get(get_plan))
        .route("/calculations", get(list_calculations).post(calculate_commission))
        .route("/calculations/:id", get(get_calculation))
        .route("/calculations/:id/approve", post(approve_calculation))
        .route("/quotas", get(list_quotas).post(create_quota))
        .route("/quotas/:id/progress", get(get_quota_progress))
        .route("/teams", get(list_teams))
        .route("/reports", get(get_reports))
        .route("/forecasts", get(get_forecasts))
}

async fn list_plans(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "plans": [], "total": 0 }))
}

async fn create_plan(
    State(_state): State<AppState>,
    Json(_req): Json<CreatePlanRequest>,
) -> Json<serde_json::Value> {
    Json(json!({ "id": Uuid::new_v4().to_string(), "message": "Plan created" }))
}

async fn get_plan(Path(_id): Path<Uuid>, State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "plan": null }))
}

async fn list_calculations(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "calculations": [], "total": 0 }))
}

async fn calculate_commission(
    State(_state): State<AppState>,
    Json(_req): Json<CalculateCommissionRequest>,
) -> Json<serde_json::Value> {
    Json(json!({ "id": Uuid::new_v4().to_string(), "total_commission": 0, "message": "Commission calculated" }))
}

async fn get_calculation(Path(_id): Path<Uuid>, State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "calculation": null }))
}

async fn approve_calculation(Path(_id): Path<Uuid>, State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "message": "Calculation approved" }))
}

async fn list_quotas(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "quotas": [], "total": 0 }))
}

async fn create_quota(
    State(_state): State<AppState>,
    Json(_req): Json<CreateQuotaRequest>,
) -> Json<serde_json::Value> {
    Json(json!({ "id": Uuid::new_v4().to_string(), "message": "Quota created" }))
}

async fn get_quota_progress(Path(_id): Path<Uuid>, State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "progress": null }))
}

async fn list_teams(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "teams": [] }))
}

async fn get_reports(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "reports": [] }))
}

async fn get_forecasts(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "forecasts": [] }))
}

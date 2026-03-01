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
pub struct CreateContractRequest {
    pub title: String,
    pub description: Option<String>,
    pub category: String,
    pub vendor_id: Option<Uuid>,
    pub customer_id: Option<Uuid>,
    pub contract_type: String,
    pub value: i64,
    pub currency: String,
    pub start_date: String,
    pub end_date: String,
    pub owner_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitApprovalRequest {
    pub approver_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApproveRequest {
    pub comments: Option<String>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_contracts).post(create_contract))
        .route("/:id", get(get_contract))
        .route("/:id/submit", post(submit_for_approval))
        .route("/:id/approve", post(approve_contract))
        .route("/:id/activate", post(activate_contract))
        .route("/:id/terminate", post(terminate_contract))
        .route("/expiring", get(list_expiring))
        .route("/types", get(list_contract_types))
        .route("/:id/risk", get(get_risk_assessment))
}

async fn list_contracts(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "contracts": [], "total": 0 }))
}

async fn create_contract(
    State(_state): State<AppState>,
    Json(_req): Json<CreateContractRequest>,
) -> Json<serde_json::Value> {
    Json(json!({ "id": Uuid::new_v4().to_string(), "message": "Contract created" }))
}

async fn get_contract(Path(_id): Path<Uuid>, State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "contract": null }))
}

async fn submit_for_approval(
    Path(_id): Path<Uuid>,
    State(_state): State<AppState>,
    Json(_req): Json<SubmitApprovalRequest>,
) -> Json<serde_json::Value> {
    Json(json!({ "message": "Contract submitted for approval" }))
}

async fn approve_contract(
    Path(_id): Path<Uuid>,
    State(_state): State<AppState>,
    Json(_req): Json<ApproveRequest>,
) -> Json<serde_json::Value> {
    Json(json!({ "message": "Contract approved" }))
}

async fn activate_contract(Path(_id): Path<Uuid>, State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "message": "Contract activated" }))
}

async fn terminate_contract(Path(_id): Path<Uuid>, State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "message": "Contract terminated" }))
}

async fn list_expiring(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "contracts": [], "total": 0 }))
}

async fn list_contract_types(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "types": [] }))
}

async fn get_risk_assessment(Path(_id): Path<Uuid>, State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "risk_assessment": null }))
}

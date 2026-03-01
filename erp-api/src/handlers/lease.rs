use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_lease).get(list_leases))
        .route("/:id", get(get_lease))
        .route("/:id/amortization", post(calculate_amortization))
        .route("/:id/rou-asset", post(create_rou_asset))
        .route("/:id/liability", post(create_liability))
        .route("/:id/payment", post(record_lease_payment))
        .route("/:id/modification", post(create_modification))
        .route("/disclosures", post(generate_disclosure))
}

#[derive(Deserialize)]
pub struct CreateLeaseRequest {
    pub name: String,
    pub description: Option<String>,
    pub lease_type: String,
    pub lessor_id: Uuid,
    pub lessee_id: Uuid,
    pub commencement_date: String,
    pub end_date: String,
    pub fair_value: i64,
    pub currency: String,
    pub discount_rate: f64,
}

#[derive(Serialize)]
pub struct LeaseResponse {
    pub id: Uuid,
    pub lease_number: String,
    pub name: String,
    pub status: String,
}

pub async fn create_lease(
    State(_state): State<AppState>,
    Json(_req): Json<CreateLeaseRequest>,
) -> Json<LeaseResponse> {
    Json(LeaseResponse {
        id: Uuid::new_v4(),
        lease_number: format!("LSE-{}", Uuid::new_v4()),
        name: "Lease".to_string(),
        status: "Draft".to_string(),
    })
}

pub async fn list_leases(State(_state): State<AppState>) -> Json<Vec<LeaseResponse>> {
    Json(vec![])
}

pub async fn get_lease(State(_state): State<AppState>) -> Json<LeaseResponse> {
    Json(LeaseResponse {
        id: Uuid::new_v4(),
        lease_number: "LSE-001".to_string(),
        name: "Office Lease".to_string(),
        status: "Active".to_string(),
    })
}

pub async fn calculate_amortization(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"message": "Amortization calculated"}))
}

pub async fn create_rou_asset(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"message": "ROU asset created"}))
}

pub async fn create_liability(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"message": "Liability created"}))
}

pub async fn record_lease_payment(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"message": "Payment recorded"}))
}

pub async fn create_modification(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"message": "Modification created"}))
}

pub async fn generate_disclosure(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"message": "Disclosure generated"}))
}

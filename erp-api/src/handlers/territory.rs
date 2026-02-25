use axum::{extract::State, routing::{get, post}, Json, Router};
use serde::Serialize;
use uuid::Uuid;
use crate::db::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/territories", post(create_territory).get(list_territories))
        .route("/territories/:id", get(get_territory))
        .route("/territories/:id/assign", post(assign_rep))
        .route("/quotas", post(create_quota).get(list_quotas))
        .route("/quotas/:id", get(get_quota))
        .route("/quotas/:id/attainment", post(record_attainment).get(get_attainment))
        .route("/performance/:territitory_id", get(get_territory_performance))
}

#[derive(Serialize)]
pub struct TerritoryResponse { pub id: Uuid, pub name: String, pub territory_type: String }
pub async fn create_territory(State(_state): State<AppState>) -> Json<TerritoryResponse> {
    Json(TerritoryResponse { id: Uuid::new_v4(), name: "Territory".to_string(), territory_type: "Geographic".to_string() })
}
pub async fn list_territories(State(_state): State<AppState>) -> Json<Vec<TerritoryResponse>> { Json(vec![]) }
pub async fn get_territory(State(_state): State<AppState>) -> Json<TerritoryResponse> {
    Json(TerritoryResponse { id: Uuid::new_v4(), name: "Territory".to_string(), territory_type: "Geographic".to_string() })
}
pub async fn assign_rep(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"message": "Rep assigned"}))
}

#[derive(Serialize)]
pub struct QuotaResponse { pub id: Uuid, pub name: String, pub annual_target: i64 }
pub async fn create_quota(State(_state): State<AppState>) -> Json<QuotaResponse> {
    Json(QuotaResponse { id: Uuid::new_v4(), name: "Quota".to_string(), annual_target: 1000000 })
}
pub async fn list_quotas(State(_state): State<AppState>) -> Json<Vec<QuotaResponse>> { Json(vec![]) }
pub async fn get_quota(State(_state): State<AppState>) -> Json<QuotaResponse> {
    Json(QuotaResponse { id: Uuid::new_v4(), name: "Quota".to_string(), annual_target: 1000000 })
}

#[derive(Serialize)]
pub struct AttainmentResponse { pub quota_id: Uuid, pub attainment_percent: f64 }
pub async fn record_attainment(State(_state): State<AppState>) -> Json<AttainmentResponse> {
    Json(AttainmentResponse { quota_id: Uuid::new_v4(), attainment_percent: 85.0 })
}
pub async fn get_attainment(State(_state): State<AppState>) -> Json<AttainmentResponse> {
    Json(AttainmentResponse { quota_id: Uuid::new_v4(), attainment_percent: 85.0 })
}

#[derive(Serialize)]
pub struct PerformanceResponse { pub revenue: i64, pub attainment_percent: f64 }
pub async fn get_territory_performance(State(_state): State<AppState>) -> Json<PerformanceResponse> {
    Json(PerformanceResponse { revenue: 500000, attainment_percent: 75.0 })
}

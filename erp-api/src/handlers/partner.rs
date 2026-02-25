use axum::{extract::State, routing::{get, post}, Json, Router};
use serde::Serialize;
use uuid::Uuid;
use crate::db::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_partner).get(list_partners))
        .route("/:id", get(get_partner))
        .route("/:id/contacts", post(create_contact).get(list_contacts))
        .route("/:id/agreements", post(create_agreement).get(list_agreements))
        .route("/:id/deals", post(create_deal).get(list_deals))
        .route("/deals/:id/register", post(register_deal))
        .route("/:id/commissions", post(create_commission).get(list_commissions))
        .route("/:id/performance", get(get_performance))
}

#[derive(Serialize)]
pub struct PartnerResponse { pub id: Uuid, pub name: String, pub partner_type: String, pub status: String }
pub async fn create_partner(State(_state): State<AppState>) -> Json<PartnerResponse> {
    Json(PartnerResponse { id: Uuid::new_v4(), name: "Partner".to_string(), partner_type: "Reseller".to_string(), status: "Active".to_string() })
}
pub async fn list_partners(State(_state): State<AppState>) -> Json<Vec<PartnerResponse>> { Json(vec![]) }
pub async fn get_partner(State(_state): State<AppState>) -> Json<PartnerResponse> {
    Json(PartnerResponse { id: Uuid::new_v4(), name: "Partner".to_string(), partner_type: "Reseller".to_string(), status: "Active".to_string() })
}

#[derive(Serialize)]
pub struct ContactResponse { pub id: Uuid, pub name: String }
pub async fn create_contact(State(_state): State<AppState>) -> Json<ContactResponse> {
    Json(ContactResponse { id: Uuid::new_v4(), name: "Contact".to_string() })
}
pub async fn list_contacts(State(_state): State<AppState>) -> Json<Vec<ContactResponse>> { Json(vec![]) }

#[derive(Serialize)]
pub struct AgreementResponse { pub id: Uuid, pub name: String }
pub async fn create_agreement(State(_state): State<AppState>) -> Json<AgreementResponse> {
    Json(AgreementResponse { id: Uuid::new_v4(), name: "Agreement".to_string() })
}
pub async fn list_agreements(State(_state): State<AppState>) -> Json<Vec<AgreementResponse>> { Json(vec![]) }

#[derive(Serialize)]
pub struct DealResponse { pub id: Uuid, pub deal_name: String, pub amount: i64 }
pub async fn create_deal(State(_state): State<AppState>) -> Json<DealResponse> {
    Json(DealResponse { id: Uuid::new_v4(), deal_name: "Deal".to_string(), amount: 10000 })
}
pub async fn list_deals(State(_state): State<AppState>) -> Json<Vec<DealResponse>> { Json(vec![]) }
pub async fn register_deal(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"message": "Deal registered"}))
}

#[derive(Serialize)]
pub struct CommissionResponse { pub id: Uuid, pub amount: i64 }
pub async fn create_commission(State(_state): State<AppState>) -> Json<CommissionResponse> {
    Json(CommissionResponse { id: Uuid::new_v4(), amount: 1000 })
}
pub async fn list_commissions(State(_state): State<AppState>) -> Json<Vec<CommissionResponse>> { Json(vec![]) }

#[derive(Serialize)]
pub struct PerformanceResponse { pub revenue: i64, pub deals_won: i32 }
pub async fn get_performance(State(_state): State<AppState>) -> Json<PerformanceResponse> {
    Json(PerformanceResponse { revenue: 100000, deals_won: 5 })
}

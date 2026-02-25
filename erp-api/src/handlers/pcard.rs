use axum::{extract::State, routing::{get, post}, Json, Router};
use serde::Serialize;
use uuid::Uuid;
use crate::db::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/cards", post(issue_card).get(list_cards))
        .route("/cards/:id", get(get_card))
        .route("/cards/:id/transactions", post(record_transaction).get(list_transactions))
        .route("/cards/:id/statements", post(create_statement).get(list_statements))
        .route("/virtual-cards", post(create_virtual_card))
        .route("/policies", post(create_policy).get(list_policies))
        .route("/disputes", post(file_dispute).get(list_disputes))
}

#[derive(Serialize)]
pub struct CardResponse { pub id: Uuid, pub masked_number: String, pub card_type: String, pub status: String }
pub async fn issue_card(State(_state): State<AppState>) -> Json<CardResponse> {
    Json(CardResponse { id: Uuid::new_v4(), masked_number: "****1234".to_string(), card_type: "Corporate".to_string(), status: "Active".to_string() })
}
pub async fn list_cards(State(_state): State<AppState>) -> Json<Vec<CardResponse>> { Json(vec![]) }
pub async fn get_card(State(_state): State<AppState>) -> Json<CardResponse> {
    Json(CardResponse { id: Uuid::new_v4(), masked_number: "****1234".to_string(), card_type: "Corporate".to_string(), status: "Active".to_string() })
}

#[derive(Serialize)]
pub struct TransactionResponse { pub id: Uuid, pub merchant: String, pub amount: i64 }
pub async fn record_transaction(State(_state): State<AppState>) -> Json<TransactionResponse> {
    Json(TransactionResponse { id: Uuid::new_v4(), merchant: "Merchant".to_string(), amount: 1000 })
}
pub async fn list_transactions(State(_state): State<AppState>) -> Json<Vec<TransactionResponse>> { Json(vec![]) }

#[derive(Serialize)]
pub struct StatementResponse { pub id: Uuid, pub balance: i64 }
pub async fn create_statement(State(_state): State<AppState>) -> Json<StatementResponse> {
    Json(StatementResponse { id: Uuid::new_v4(), balance: 5000 })
}
pub async fn list_statements(State(_state): State<AppState>) -> Json<Vec<StatementResponse>> { Json(vec![]) }

pub async fn create_virtual_card(State(_state): State<AppState>) -> Json<CardResponse> {
    Json(CardResponse { id: Uuid::new_v4(), masked_number: "****5678".to_string(), card_type: "Virtual".to_string(), status: "Active".to_string() })
}

#[derive(Serialize)]
pub struct PolicyResponse { pub id: Uuid, pub name: String }
pub async fn create_policy(State(_state): State<AppState>) -> Json<PolicyResponse> {
    Json(PolicyResponse { id: Uuid::new_v4(), name: "Policy".to_string() })
}
pub async fn list_policies(State(_state): State<AppState>) -> Json<Vec<PolicyResponse>> { Json(vec![]) }

#[derive(Serialize)]
pub struct DisputeResponse { pub id: Uuid, pub status: String }
pub async fn file_dispute(State(_state): State<AppState>) -> Json<DisputeResponse> {
    Json(DisputeResponse { id: Uuid::new_v4(), status: "Filed".to_string() })
}
pub async fn list_disputes(State(_state): State<AppState>) -> Json<Vec<DisputeResponse>> { Json(vec![]) }

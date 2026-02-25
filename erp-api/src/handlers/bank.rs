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
        .route("/connections", post(create_connection).get(list_connections))
        .route("/connections/:id", get(get_connection))
        .route("/accounts", post(create_bank_account).get(list_bank_accounts))
        .route("/accounts/:id", get(get_bank_account))
        .route("/statements", post(import_statement).get(list_statements))
        .route("/transactions", post(create_transaction).get(list_transactions))
        .route("/reconcile", post(start_reconciliation))
        .route("/matches", post(create_match))
        .route("/payment-files", post(generate_payment_file))
}

#[derive(Serialize)]
pub struct BankConnectionResponse {
    pub id: Uuid,
    pub bank_name: String,
    pub status: String,
}

pub async fn create_connection(State(_state): State<AppState>) -> Json<BankConnectionResponse> {
    Json(BankConnectionResponse { id: Uuid::new_v4(), bank_name: "Bank".to_string(), status: "Active".to_string() })
}

pub async fn list_connections(State(_state): State<AppState>) -> Json<Vec<BankConnectionResponse>> { Json(vec![]) }
pub async fn get_connection(State(_state): State<AppState>) -> Json<BankConnectionResponse> {
    Json(BankConnectionResponse { id: Uuid::new_v4(), bank_name: "Bank".to_string(), status: "Active".to_string() })
}

#[derive(Serialize)]
pub struct BankAccountResponse { pub id: Uuid, pub account_name: String }
pub async fn create_bank_account(State(_state): State<AppState>) -> Json<BankAccountResponse> {
    Json(BankAccountResponse { id: Uuid::new_v4(), account_name: "Account".to_string() })
}
pub async fn list_bank_accounts(State(_state): State<AppState>) -> Json<Vec<BankAccountResponse>> { Json(vec![]) }
pub async fn get_bank_account(State(_state): State<AppState>) -> Json<BankAccountResponse> {
    Json(BankAccountResponse { id: Uuid::new_v4(), account_name: "Account".to_string() })
}

pub async fn import_statement(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"message": "Statement imported"}))
}
pub async fn list_statements(State(_state): State<AppState>) -> Json<serde_json::Value> { Json(serde_json::json!([])) }
pub async fn create_transaction(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"message": "Transaction created"}))
}
pub async fn list_transactions(State(_state): State<AppState>) -> Json<serde_json::Value> { Json(serde_json::json!([])) }
pub async fn start_reconciliation(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"message": "Reconciliation started"}))
}
pub async fn create_match(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"message": "Match created"}))
}
pub async fn generate_payment_file(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"message": "Payment file generated"}))
}

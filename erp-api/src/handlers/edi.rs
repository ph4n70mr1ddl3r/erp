use axum::{
    extract::{State, Query},
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::db::AppState;
use crate::error::ApiResult;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    partner_type: Option<String>,
}

pub async fn list_partners(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> ApiResult<Json<Vec<erp_edi::EdiPartner>>> {
    let partner_type = query.partner_type.and_then(|s| match s.as_str() {
        "Customer" => Some(erp_edi::PartnerType::Customer),
        "Vendor" => Some(erp_edi::PartnerType::Vendor),
        "Carrier" => Some(erp_edi::PartnerType::Carrier),
        "Bank" => Some(erp_edi::PartnerType::Bank),
        "Warehouse" => Some(erp_edi::PartnerType::Warehouse),
        _ => None,
    });
    let partners = erp_edi::EdiService::new(erp_edi::SqliteEdiRepository::new(state.pool.clone()))
        .list_partners(partner_type)
        .await?;
    Ok(Json(partners))
}

pub async fn create_partner(
    State(state): State<AppState>,
    Json(req): Json<erp_edi::CreatePartnerRequest>,
) -> ApiResult<Json<erp_edi::EdiPartner>> {
    let partner = erp_edi::EdiService::new(erp_edi::SqliteEdiRepository::new(state.pool.clone()))
        .create_partner(req)
        .await?;
    Ok(Json(partner))
}

pub async fn process_inbound(
    State(state): State<AppState>,
    Json(req): Json<erp_edi::ProcessEdiRequest>,
) -> ApiResult<Json<erp_edi::EdiTransaction>> {
    let txn = erp_edi::EdiService::new(erp_edi::SqliteEdiRepository::new(state.pool.clone()))
        .process_inbound(req)
        .await?;
    Ok(Json(txn))
}

pub async fn generate_outbound(
    State(state): State<AppState>,
    Json(req): Json<erp_edi::GenerateEdiRequest>,
) -> ApiResult<Json<erp_edi::EdiTransmissionResult>> {
    let result = erp_edi::EdiService::new(erp_edi::SqliteEdiRepository::new(state.pool.clone()))
        .generate_outbound(req)
        .await?;
    Ok(Json(result))
}

#[derive(Debug, Deserialize)]
pub struct ListTransactionsQuery {
    partner_id: Option<Uuid>,
    transaction_type: Option<String>,
}

pub async fn list_transactions(
    State(state): State<AppState>,
    Query(query): Query<ListTransactionsQuery>,
) -> ApiResult<Json<Vec<erp_edi::EdiTransaction>>> {
    let txn_type = query.transaction_type.and_then(|s| match s.as_str() {
        "X12_850" => Some(erp_edi::EdiTransactionType::X12_850),
        "X12_810" => Some(erp_edi::EdiTransactionType::X12_810),
        "X12_856" => Some(erp_edi::EdiTransactionType::X12_856),
        _ => None,
    });
    let txns = erp_edi::EdiService::new(erp_edi::SqliteEdiRepository::new(state.pool.clone()))
        .list_transactions(query.partner_id, txn_type)
        .await?;
    Ok(Json(txns))
}

pub fn routes() -> axum::Router<crate::db::AppState> {
    axum::Router::new()
        .route("/partners", axum::routing::get(list_partners).post(create_partner))
        .route("/inbound", axum::routing::post(process_inbound))
        .route("/outbound", axum::routing::post(generate_outbound))
        .route("/transactions", axum::routing::get(list_transactions))
}

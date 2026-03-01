use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::ApiResult;
use crate::db::AppState;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    page: Option<i32>,
    page_size: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    items: Vec<T>,
    total: i64,
    page: i32,
    page_size: i32,
}

pub async fn list_entities(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<erp_intercompany::IntercompanyEntity>>> {
    let _service = erp_intercompany::IntercompanyService::new(erp_intercompany::SqliteIntercompanyRepository::new(state.pool.clone()));
    Ok(Json(vec![]))
}

#[derive(Debug, Deserialize)]
pub struct CreateEntityRequest {
    pub code: String,
    pub name: String,
    pub currency: String,
}

pub async fn create_entity(
    State(state): State<AppState>,
    Json(req): Json<CreateEntityRequest>,
) -> ApiResult<Json<erp_intercompany::IntercompanyEntity>> {
    let entity = erp_intercompany::IntercompanyService::new(erp_intercompany::SqliteIntercompanyRepository::new(state.pool.clone()))
        .create_entity(erp_intercompany::CreateEntityRequest {
            code: req.code,
            name: req.name,
            legal_entity_id: None,
            currency: req.currency,
        })
        .await?;
    Ok(Json(entity))
}

pub async fn create_transaction(
    State(state): State<AppState>,
    Json(req): Json<erp_intercompany::CreateICTransactionRequest>,
) -> ApiResult<Json<erp_intercompany::IntercompanyTransaction>> {
    let txn = erp_intercompany::IntercompanyService::new(erp_intercompany::SqliteIntercompanyRepository::new(state.pool.clone()))
        .create_transaction(req)
        .await?;
    Ok(Json(txn))
}

pub async fn settle_transaction(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<erp_intercompany::IntercompanyTransaction>> {
    let txn = erp_intercompany::IntercompanyService::new(erp_intercompany::SqliteIntercompanyRepository::new(state.pool.clone()))
        .settle_transaction(id)
        .await?;
    Ok(Json(txn))
}

pub async fn create_transfer_price(
    State(state): State<AppState>,
    Json(req): Json<erp_intercompany::CreateTransferPriceRequest>,
) -> ApiResult<Json<erp_intercompany::TransferPrice>> {
    let price = erp_intercompany::IntercompanyService::new(erp_intercompany::SqliteIntercompanyRepository::new(state.pool.clone()))
        .create_transfer_price(req)
        .await?;
    Ok(Json(price))
}

pub async fn run_consolidation(
    State(state): State<AppState>,
    Json(req): Json<erp_intercompany::RunConsolidationRequest>,
) -> ApiResult<Json<erp_intercompany::ConsolidationResult>> {
    let result = erp_intercompany::IntercompanyService::new(erp_intercompany::SqliteIntercompanyRepository::new(state.pool.clone()))
        .run_consolidation(req)
        .await?;
    Ok(Json(result))
}

#[derive(Debug, Deserialize)]
pub struct CreateAgreementRequest {
    pub source_entity_id: Uuid,
    pub target_entity_id: Uuid,
    pub name: String,
    pub agreement_type: erp_intercompany::AgreementType,
}

pub async fn create_agreement(
    State(state): State<AppState>,
    Json(req): Json<CreateAgreementRequest>,
) -> ApiResult<Json<erp_intercompany::IntercompanyAgreement>> {
    let agreement = erp_intercompany::IntercompanyService::new(erp_intercompany::SqliteIntercompanyRepository::new(state.pool.clone()))
        .create_agreement(req.source_entity_id, req.target_entity_id, req.name, req.agreement_type)
        .await?;
    Ok(Json(agreement))
}

pub fn routes() -> axum::Router<crate::db::AppState> {
    axum::Router::new()
        .route("/entities", axum::routing::get(list_entities).post(create_entity))
        .route("/transactions", axum::routing::post(create_transaction))
        .route("/transactions/:id/settle", axum::routing::post(settle_transaction))
        .route("/transfer-prices", axum::routing::post(create_transfer_price))
        .route("/consolidations", axum::routing::post(run_consolidation))
        .route("/agreements", axum::routing::post(create_agreement))
}

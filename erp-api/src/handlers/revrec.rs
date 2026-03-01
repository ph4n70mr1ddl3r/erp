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
    customer_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    items: Vec<T>,
    total: i64,
    page: i32,
    page_size: i32,
}

pub async fn create_contract(
    State(state): State<AppState>,
    Json(req): Json<erp_revrec::CreateContractRequest>,
) -> ApiResult<Json<erp_revrec::RevenueContract>> {
    let contract = erp_revrec::RevRecService::new(erp_revrec::SqliteRevRecRepository::new(state.pool.clone()))
        .create_contract(req)
        .await?;
    Ok(Json(contract))
}

pub async fn get_contract(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<erp_revrec::RevenueContract>> {
    let contract = erp_revrec::RevRecService::new(erp_revrec::SqliteRevRecRepository::new(state.pool.clone()))
        .get_contract(id)
        .await?
        .ok_or_else(|| erp_core::Error::NotFound("Contract not found".into()))?;
    Ok(Json(contract))
}

pub async fn get_waterfall(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<erp_revrec::RevenueWaterfall>> {
    let waterfall = erp_revrec::RevRecService::new(erp_revrec::SqliteRevRecRepository::new(state.pool.clone()))
        .get_waterfall(id)
        .await?;
    Ok(Json(waterfall))
}

pub async fn recognize_revenue(
    State(state): State<AppState>,
    Json(req): Json<erp_revrec::RecognizeRevenueRequest>,
) -> ApiResult<Json<erp_revrec::RevenueEvent>> {
    let event = erp_revrec::RevRecService::new(erp_revrec::SqliteRevRecRepository::new(state.pool.clone()))
        .recognize_revenue(req)
        .await?;
    Ok(Json(event))
}

#[derive(Debug, Deserialize)]
pub struct ModifyContractRequest {
    pub contract_id: Uuid,
    pub modification_type: erp_revrec::ModificationType,
    pub description: String,
    pub price_change: i64,
}

pub async fn modify_contract(
    State(state): State<AppState>,
    Json(req): Json<ModifyContractRequest>,
) -> ApiResult<Json<erp_revrec::ContractModification>> {
    let _service = erp_revrec::RevRecService::new(erp_revrec::SqliteRevRecRepository::new(state.pool.clone()));
    let _ = req;
    Err(erp_core::Error::validation("Not implemented"))?
}

#[derive(Debug, Deserialize)]
pub struct CreateAllocationRuleRequest {
    pub name: String,
    pub method: erp_revrec::AllocationMethod,
    pub basis: erp_revrec::AllocationBasis,
}

pub async fn create_allocation_rule(
    State(state): State<AppState>,
    Json(req): Json<CreateAllocationRuleRequest>,
) -> ApiResult<Json<erp_revrec::AllocationRule>> {
    let rule = erp_revrec::RevRecService::new(erp_revrec::SqliteRevRecRepository::new(state.pool.clone()))
        .create_allocation_rule(req.name, req.method, req.basis)
        .await?;
    Ok(Json(rule))
}

pub fn routes() -> axum::Router<crate::db::AppState> {
    axum::Router::new()
        .route("/contracts", axum::routing::post(create_contract))
        .route("/contracts/:id", axum::routing::get(get_contract))
        .route("/contracts/:id/waterfall", axum::routing::get(get_waterfall))
        .route("/recognize", axum::routing::post(recognize_revenue))
        .route("/modify", axum::routing::post(modify_contract))
        .route("/allocation-rules", axum::routing::post(create_allocation_rule))
}

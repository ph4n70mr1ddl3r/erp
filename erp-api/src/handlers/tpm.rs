use axum::{
    extract::{Path, Query, State},
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
    status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    items: Vec<T>,
    total: i64,
    page: i32,
    page_size: i32,
}

pub async fn list_promotions(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> ApiResult<Json<PaginatedResponse<erp_tpm::TradePromotion>>> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);
    let status = query.status.and_then(|s| match s.as_str() {
        "Draft" => Some(erp_tpm::PromotionStatus::Draft),
        "Planned" => Some(erp_tpm::PromotionStatus::Planned),
        "Active" => Some(erp_tpm::PromotionStatus::Active),
        "Completed" => Some(erp_tpm::PromotionStatus::Completed),
        "Cancelled" => Some(erp_tpm::PromotionStatus::Cancelled),
        _ => None,
    });
    let items = erp_tpm::TPMService::new(erp_tpm::SqliteTPMRepository::new(state.pool.clone()))
        .list_promotions(status, page, page_size)
        .await?;
    Ok(Json(PaginatedResponse {
        total: items.len() as i64,
        items,
        page,
        page_size,
    }))
}

pub async fn create_promotion(
    State(state): State<AppState>,
    Json(req): Json<erp_tpm::CreatePromotionRequest>,
) -> ApiResult<Json<erp_tpm::TradePromotion>> {
    let promo = erp_tpm::TPMService::new(erp_tpm::SqliteTPMRepository::new(state.pool.clone()))
        .create_promotion(req)
        .await?;
    Ok(Json(promo))
}

pub async fn get_promotion(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<erp_tpm::TradePromotion>> {
    let promo = erp_tpm::TPMService::new(erp_tpm::SqliteTPMRepository::new(state.pool.clone()))
        .get_promotion(id)
        .await?
        .ok_or_else(|| erp_core::Error::NotFound("Promotion not found".into(.into())))?;
    Ok(Json(promo))
}

pub async fn activate_promotion(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<erp_tpm::TradePromotion>> {
    let promo = erp_tpm::TPMService::new(erp_tpm::SqliteTPMRepository::new(state.pool.clone()))
        .activate_promotion(id)
        .await?;
    Ok(Json(promo))
}

pub async fn calculate_promotion_performance(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<erp_tpm::PromotionPerformance>> {
    let perf = erp_tpm::TPMService::new(erp_tpm::SqliteTPMRepository::new(state.pool.clone()))
        .calculate_promotion_performance(id)
        .await?;
    Ok(Json(perf))
}

pub async fn create_rebate_agreement(
    State(state): State<AppState>,
    Json(req): Json<erp_tpm::CreateRebateAgreementRequest>,
) -> ApiResult<Json<erp_tpm::RebateAgreement>> {
    let agreement = erp_tpm::TPMService::new(erp_tpm::SqliteTPMRepository::new(state.pool.clone()))
        .create_rebate_agreement(req)
        .await?;
    Ok(Json(agreement))
}

pub async fn get_rebate_agreement(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<erp_tpm::RebateAgreement>> {
    let agreement = erp_tpm::TPMService::new(erp_tpm::SqliteTPMRepository::new(state.pool.clone()))
        .get_rebate_agreement(id)
        .await?
        .ok_or_else(|| erp_core::Error::NotFound("Rebate agreement not found".into(.into())))?;
    Ok(Json(agreement))
}

#[derive(Debug, Deserialize)]
pub struct CalculateRebateRequest {
    sales_amount: i64,
    product_id: Uuid,
}

pub async fn calculate_rebate(
    State(state): State<AppState>,
    Path(agreement_id): Path<Uuid>,
    Json(req): Json<CalculateRebateRequest>,
) -> ApiResult<Json<erp_tpm::RebateAccrual>> {
    let accrual = erp_tpm::TPMService::new(erp_tpm::SqliteTPMRepository::new(state.pool.clone()))
        .calculate_rebate(agreement_id, req.sales_amount, req.product_id)
        .await?;
    Ok(Json(accrual))
}

#[derive(Debug, Deserialize)]
pub struct ProcessPaymentRequest {
    customer_id: Uuid,
    period_start: chrono::DateTime<chrono::Utc>,
    period_end: chrono::DateTime<chrono::Utc>,
}

pub async fn process_rebate_payment(
    State(state): State<AppState>,
    Path(agreement_id): Path<Uuid>,
    Json(req): Json<ProcessPaymentRequest>,
) -> ApiResult<Json<erp_tpm::RebatePayment>> {
    let payment = erp_tpm::TPMService::new(erp_tpm::SqliteTPMRepository::new(state.pool.clone()))
        .process_rebate_payment(agreement_id, req.customer_id, req.period_start, req.period_end)
        .await?;
    Ok(Json(payment))
}

pub async fn submit_chargeback(
    State(state): State<AppState>,
    Json(req): Json<erp_tpm::SubmitChargebackRequest>,
) -> ApiResult<Json<erp_tpm::Chargeback>> {
    let cb = erp_tpm::TPMService::new(erp_tpm::SqliteTPMRepository::new(state.pool.clone()))
        .submit_chargeback(req)
        .await?;
    Ok(Json(cb))
}

#[derive(Debug, Deserialize)]
pub struct ReviewChargebackRequest {
    approved_amount: i64,
    reviewed_by: Uuid,
    rejection_reason: Option<String>,
}

pub async fn review_chargeback(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<ReviewChargebackRequest>,
) -> ApiResult<Json<erp_tpm::Chargeback>> {
    let cb = erp_tpm::TPMService::new(erp_tpm::SqliteTPMRepository::new(state.pool.clone()))
        .review_chargeback(id, req.approved_amount, req.reviewed_by, req.rejection_reason)
        .await?;
    Ok(Json(cb))
}

#[derive(Debug, Deserialize)]
pub struct CreateFundRequest {
    customer_id: Option<Uuid>,
    name: String,
    fund_type: String,
    fiscal_year: i32,
    total_budget: i64,
}

pub async fn create_trade_fund(
    State(state): State<AppState>,
    Json(req): Json<CreateFundRequest>,
) -> ApiResult<Json<erp_tpm::TradeFund>> {
    let fund_type = match req.fund_type.as_str() {
        "MarketingDevelopment" => erp_tpm::FundType::MarketingDevelopment,
        "CooperativeAdvertising" => erp_tpm::FundType::CooperativeAdvertising,
        "Display" => erp_tpm::FundType::Display,
        "Sampling" => erp_tpm::FundType::Sampling,
        "TradeShow" => erp_tpm::FundType::TradeShow,
        _ => erp_tpm::FundType::Other,
    };
    let fund = erp_tpm::TPMService::new(erp_tpm::SqliteTPMRepository::new(state.pool.clone()))
        .create_trade_fund(req.customer_id, req.name, fund_type, req.fiscal_year, req.total_budget)
        .await?;
    Ok(Json(fund))
}

#[derive(Debug, Deserialize)]
pub struct CommitFundRequest {
    promotion_id: Uuid,
    amount: i64,
}

pub async fn commit_fund(
    State(state): State<AppState>,
    Path(fund_id): Path<Uuid>,
    Json(req): Json<CommitFundRequest>,
) -> ApiResult<Json<erp_tpm::TradeFund>> {
    let fund = erp_tpm::TPMService::new(erp_tpm::SqliteTPMRepository::new(state.pool.clone()))
        .commit_fund(fund_id, req.promotion_id, req.amount)
        .await?;
    Ok(Json(fund))
}

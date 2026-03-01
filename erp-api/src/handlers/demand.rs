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
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    items: Vec<T>,
    total: i64,
    page: i32,
    page_size: i32,
}

pub async fn run_forecast(
    State(state): State<AppState>,
    Json(req): Json<erp_demand::RunForecastRequest>,
) -> ApiResult<Json<erp_demand::ForecastResult>> {
    let result = erp_demand::DemandService::new(erp_demand::SqliteDemandRepository::new(state.pool.clone()))
        .run_forecast(req)
        .await?;
    Ok(Json(result))
}

pub async fn create_plan(
    State(state): State<AppState>,
    Json(req): Json<erp_demand::CreateDemandPlanRequest>,
) -> ApiResult<Json<erp_demand::DemandPlan>> {
    let plan = erp_demand::DemandService::new(erp_demand::SqliteDemandRepository::new(state.pool.clone()))
        .create_demand_plan(req)
        .await?;
    Ok(Json(plan))
}

pub async fn calculate_safety_stock(
    State(state): State<AppState>,
    Json(req): Json<erp_demand::CalculateSafetyStockRequest>,
) -> ApiResult<Json<erp_demand::SafetyStock>> {
    let stock = erp_demand::DemandService::new(erp_demand::SqliteDemandRepository::new(state.pool.clone()))
        .calculate_safety_stock(req)
        .await?;
    Ok(Json(stock))
}

#[derive(Debug, Deserialize)]
pub struct AccuracyRequest {
    pub product_id: Uuid,
    pub period: String,
}

pub async fn get_accuracy(
    State(state): State<AppState>,
    Json(req): Json<AccuracyRequest>,
) -> ApiResult<Json<erp_demand::ForecastAccuracy>> {
    let accuracy = erp_demand::DemandService::new(erp_demand::SqliteDemandRepository::new(state.pool.clone()))
        .get_forecast_accuracy(req.product_id, req.period)
        .await?;
    Ok(Json(accuracy))
}

#[derive(Debug, Deserialize)]
pub struct AddSignalRequest {
    pub signal_type: erp_demand::SignalType,
    pub source: String,
    pub value: f64,
    pub product_ids: Vec<Uuid>,
}

pub async fn add_signal(
    State(state): State<AppState>,
    Json(req): Json<AddSignalRequest>,
) -> ApiResult<Json<erp_demand::DemandSensingSignal>> {
    let signal = erp_demand::DemandService::new(erp_demand::SqliteDemandRepository::new(state.pool.clone()))
        .add_demand_signal(req.signal_type, req.source, req.value, req.product_ids)
        .await?;
    Ok(Json(signal))
}

pub fn routes() -> axum::Router<crate::db::AppState> {
    axum::Router::new()
        .route("/run", axum::routing::post(run_forecast))
        .route("/plans", axum::routing::post(create_plan))
        .route("/safety-stock", axum::routing::post(calculate_safety_stock))
        .route("/accuracy", axum::routing::post(get_accuracy))
        .route("/signals", axum::routing::post(add_signal))
}

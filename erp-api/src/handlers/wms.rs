use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::ApiResult;
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    page: Option<i32>,
    page_size: Option<i32>,
    warehouse_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    items: Vec<T>,
    total: i64,
    page: i32,
    page_size: i32,
}

pub async fn list_locations(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> ApiResult<Json<PaginatedResponse<erp_wms::StorageLocation>>> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);
    let warehouse_id = query.warehouse_id.ok_or_else(|| crate::error::ApiError::BadRequest("warehouse_id required".into()))?;
    let items = erp_wms::WMSService::new(erp_wms::SqliteWMSRepository::new(state.pool.clone()))
        .list_locations(warehouse_id, page, page_size)
        .await?;
    Ok(Json(PaginatedResponse {
        total: items.len() as i64,
        items,
        page,
        page_size,
    }))
}

pub async fn create_location(
    State(state): State<AppState>,
    Json(req): Json<erp_wms::CreateLocationRequest>,
) -> ApiResult<Json<erp_wms::StorageLocation>> {
    let location = erp_wms::WMSService::new(erp_wms::SqliteWMSRepository::new(state.pool.clone()))
        .create_location(req)
        .await?;
    Ok(Json(location))
}

pub async fn get_location(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<erp_wms::StorageLocation>> {
    let location = erp_wms::WMSService::new(erp_wms::SqliteWMSRepository::new(state.pool.clone()))
        .get_location(id)
        .await?
        .ok_or_else(|| crate::error::ApiError::NotFound("Location not found".into()))?;
    Ok(Json(location))
}

pub async fn create_wave(
    State(state): State<AppState>,
    Json(req): Json<erp_wms::CreateWaveRequest>,
) -> ApiResult<Json<erp_wms::Wave>> {
    let wave = erp_wms::WMSService::new(erp_wms::SqliteWMSRepository::new(state.pool.clone()))
        .create_wave(req)
        .await?;
    Ok(Json(wave))
}

pub async fn release_wave(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<erp_wms::Wave>> {
    let wave = erp_wms::WMSService::new(erp_wms::SqliteWMSRepository::new(state.pool.clone()))
        .release_wave(id)
        .await?;
    Ok(Json(wave))
}

pub async fn create_pick_task(
    State(state): State<AppState>,
    Json(req): Json<erp_wms::CreatePickTaskRequest>,
) -> ApiResult<Json<erp_wms::PickTask>> {
    let task = erp_wms::WMSService::new(erp_wms::SqliteWMSRepository::new(state.pool.clone()))
        .create_pick_task(req)
        .await?;
    Ok(Json(task))
}

#[derive(Debug, Deserialize)]
pub struct CompletePickRequest {
    pub quantity: i64,
}

pub async fn complete_pick(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<CompletePickRequest>,
) -> ApiResult<Json<erp_wms::PickTask>> {
    let task = erp_wms::WMSService::new(erp_wms::SqliteWMSRepository::new(state.pool.clone()))
        .complete_pick(id, req.quantity)
        .await?;
    Ok(Json(task))
}

pub async fn create_cycle_count(
    State(state): State<AppState>,
    Json(req): Json<erp_wms::CreateCycleCountRequest>,
) -> ApiResult<Json<erp_wms::CycleCount>> {
    let count = erp_wms::WMSService::new(erp_wms::SqliteWMSRepository::new(state.pool.clone()))
        .create_cycle_count(req)
        .await?;
    Ok(Json(count))
}

pub async fn create_receipt(
    State(state): State<AppState>,
    Json(req): Json<CreateReceiptRequest>,
) -> ApiResult<Json<erp_wms::ReceivingReceipt>> {
    let receipt = erp_wms::WMSService::new(erp_wms::SqliteWMSRepository::new(state.pool.clone()))
        .create_receipt(req.warehouse_id, req.po_id)
        .await?;
    Ok(Json(receipt))
}

#[derive(Debug, Deserialize)]
pub struct CreateReceiptRequest {
    pub warehouse_id: Uuid,
    pub po_id: Option<Uuid>,
}

pub async fn optimize_wave(
    State(state): State<AppState>,
    Json(req): Json<erp_wms::OptimizeWaveRequest>,
) -> ApiResult<Json<erp_wms::WaveOptimizationResult>> {
    let result = erp_wms::WMSService::new(erp_wms::SqliteWMSRepository::new(state.pool.clone()))
        .optimize_wave(req)
        .await?;
    Ok(Json(result))
}

pub fn routes() -> axum::Router<crate::state::AppState> {
    axum::Router::new()
        .route("/locations", axum::routing::get(list_locations).post(create_location))
        .route("/locations/:id", axum::routing::get(get_location))
        .route("/waves", axum::routing::post(create_wave))
        .route("/waves/:id/release", axum::routing::post(release_wave))
        .route("/pick-tasks", axum::routing::post(create_pick_task))
        .route("/pick-tasks/:id/complete", axum::routing::post(complete_pick))
        .route("/cycle-counts", axum::routing::post(create_cycle_count))
        .route("/receipts", axum::routing::post(create_receipt))
        .route("/optimize-wave", axum::routing::post(optimize_wave))
}

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
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
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    items: Vec<T>,
    total: i64,
    page: i32,
    page_size: i32,
}

pub async fn list_vehicles(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> ApiResult<Json<PaginatedResponse<erp_tms::Vehicle>>> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);
    let items = erp_tms::TMSService::new(erp_tms::SqliteTMSRepository::new(state.pool.clone()))
        .list_vehicles(page, page_size)
        .await?;
    Ok(Json(PaginatedResponse {
        total: items.len() as i64,
        items,
        page,
        page_size,
    }))
}

pub async fn create_vehicle(
    State(state): State<AppState>,
    Json(req): Json<erp_tms::CreateVehicleRequest>,
) -> ApiResult<Json<erp_tms::Vehicle>> {
    let vehicle = erp_tms::TMSService::new(erp_tms::SqliteTMSRepository::new(state.pool.clone()))
        .create_vehicle(req)
        .await?;
    Ok(Json(vehicle))
}

pub async fn get_vehicle(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<erp_tms::Vehicle>> {
    let vehicle = erp_tms::TMSService::new(erp_tms::SqliteTMSRepository::new(state.pool.clone()))
        .get_vehicle(id)
        .await?
        .ok_or_else(|| crate::error::ApiError::NotFound("Vehicle not found".into()))?;
    Ok(Json(vehicle))
}

pub async fn list_drivers(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> ApiResult<Json<PaginatedResponse<erp_tms::Driver>>> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);
    let items = erp_tms::TMSService::new(erp_tms::SqliteTMSRepository::new(state.pool.clone()))
        .list_drivers(page, page_size)
        .await?;
    Ok(Json(PaginatedResponse {
        total: items.len() as i64,
        items,
        page,
        page_size,
    }))
}

pub async fn create_driver(
    State(state): State<AppState>,
    Json(req): Json<erp_tms::CreateDriverRequest>,
) -> ApiResult<Json<erp_tms::Driver>> {
    let driver = erp_tms::TMSService::new(erp_tms::SqliteTMSRepository::new(state.pool.clone()))
        .create_driver(req)
        .await?;
    Ok(Json(driver))
}

pub async fn get_driver(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<erp_tms::Driver>> {
    let driver = erp_tms::TMSService::new(erp_tms::SqliteTMSRepository::new(state.pool.clone()))
        .get_driver(id)
        .await?
        .ok_or_else(|| crate::error::ApiError::NotFound("Driver not found".into()))?;
    Ok(Json(driver))
}

pub async fn list_loads(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> ApiResult<Json<PaginatedResponse<erp_tms::Load>>> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);
    let items = erp_tms::TMSService::new(erp_tms::SqliteTMSRepository::new(state.pool.clone()))
        .list_loads(page, page_size)
        .await?;
    Ok(Json(PaginatedResponse {
        total: items.len() as i64,
        items,
        page,
        page_size,
    }))
}

pub async fn create_load(
    State(state): State<AppState>,
    Json(req): Json<erp_tms::CreateLoadRequest>,
) -> ApiResult<Json<erp_tms::Load>> {
    let load = erp_tms::TMSService::new(erp_tms::SqliteTMSRepository::new(state.pool.clone()))
        .create_load(req)
        .await?;
    Ok(Json(load))
}

pub async fn get_load(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<erp_tms::Load>> {
    let load = erp_tms::TMSService::new(erp_tms::SqliteTMSRepository::new(state.pool.clone()))
        .get_load(id)
        .await?
        .ok_or_else(|| crate::error::ApiError::NotFound("Load not found".into()))?;
    Ok(Json(load))
}

#[derive(Debug, Deserialize)]
pub struct AssignLoadRequest {
    driver_id: Uuid,
    vehicle_id: Uuid,
}

pub async fn assign_load(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<AssignLoadRequest>,
) -> ApiResult<Json<erp_tms::Load>> {
    let load = erp_tms::TMSService::new(erp_tms::SqliteTMSRepository::new(state.pool.clone()))
        .assign_load(id, req.driver_id, req.vehicle_id)
        .await?;
    Ok(Json(load))
}

pub async fn dispatch_load(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<erp_tms::Load>> {
    let load = erp_tms::TMSService::new(erp_tms::SqliteTMSRepository::new(state.pool.clone()))
        .dispatch_load(id)
        .await?;
    Ok(Json(load))
}

pub async fn deliver_load(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<erp_tms::Load>> {
    let load = erp_tms::TMSService::new(erp_tms::SqliteTMSRepository::new(state.pool.clone()))
        .deliver_load(id)
        .await?;
    Ok(Json(load))
}

pub async fn optimize_route(
    State(state): State<AppState>,
    Json(req): Json<erp_tms::RouteOptimizationRequest>,
) -> ApiResult<Json<erp_tms::RouteOptimizationResult>> {
    let result = erp_tms::TMSService::new(erp_tms::SqliteTMSRepository::new(state.pool.clone()))
        .optimize_route(req)
        .await?;
    Ok(Json(result))
}

pub async fn audit_freight_invoice(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<erp_tms::FreightInvoice>> {
    let invoice = erp_tms::TMSService::new(erp_tms::SqliteTMSRepository::new(state.pool.clone()))
        .audit_freight_invoice(id)
        .await?;
    Ok(Json(invoice))
}

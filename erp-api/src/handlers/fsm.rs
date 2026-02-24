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
    status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    items: Vec<T>,
    total: i64,
    page: i32,
    page_size: i32,
}

pub async fn list_service_orders(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> ApiResult<Json<PaginatedResponse<erp_fsm::ServiceOrder>>> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);
    let status = query.status.and_then(|s| match s.as_str() {
        "Scheduled" => Some(erp_fsm::ServiceOrderStatus::Scheduled),
        "Dispatched" => Some(erp_fsm::ServiceOrderStatus::Dispatched),
        "InProgress" => Some(erp_fsm::ServiceOrderStatus::InProgress),
        "OnHold" => Some(erp_fsm::ServiceOrderStatus::OnHold),
        "Completed" => Some(erp_fsm::ServiceOrderStatus::Completed),
        "Cancelled" => Some(erp_fsm::ServiceOrderStatus::Cancelled),
        _ => None,
    });
    let items = erp_fsm::FSMService::new(erp_fsm::SqliteFSMRepository::new(state.pool.clone()))
        .list_service_orders(status, page, page_size)
        .await?;
    Ok(Json(PaginatedResponse {
        total: items.len() as i64,
        items,
        page,
        page_size,
    }))
}

pub async fn create_service_order(
    State(state): State<AppState>,
    Json(req): Json<erp_fsm::CreateServiceOrderRequest>,
) -> ApiResult<Json<erp_fsm::ServiceOrder>> {
    let order = erp_fsm::FSMService::new(erp_fsm::SqliteFSMRepository::new(state.pool.clone()))
        .create_service_order(req)
        .await?;
    Ok(Json(order))
}

pub async fn get_service_order(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<erp_fsm::ServiceOrder>> {
    let order = erp_fsm::FSMService::new(erp_fsm::SqliteFSMRepository::new(state.pool.clone()))
        .get_service_order(id)
        .await?
        .ok_or_else(|| crate::error::ApiError::NotFound("Service order not found".into()))?;
    Ok(Json(order))
}

pub async fn dispatch_order(
    State(state): State<AppState>,
    Json(req): Json<erp_fsm::DispatchRequest>,
) -> ApiResult<Json<erp_fsm::ServiceOrder>> {
    let order = erp_fsm::FSMService::new(erp_fsm::SqliteFSMRepository::new(state.pool.clone()))
        .dispatch_order(req)
        .await?;
    Ok(Json(order))
}

pub async fn start_service(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<erp_fsm::ServiceOrder>> {
    let order = erp_fsm::FSMService::new(erp_fsm::SqliteFSMRepository::new(state.pool.clone()))
        .start_service(id)
        .await?;
    Ok(Json(order))
}

#[derive(Debug, Deserialize)]
pub struct CompleteServiceRequest {
    resolution_notes: String,
    total_charges: i64,
}

pub async fn complete_service(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<CompleteServiceRequest>,
) -> ApiResult<Json<erp_fsm::ServiceOrder>> {
    let order = erp_fsm::FSMService::new(erp_fsm::SqliteFSMRepository::new(state.pool.clone()))
        .complete_service(id, req.resolution_notes, req.total_charges)
        .await?;
    Ok(Json(order))
}

#[derive(Debug, Deserialize)]
pub struct RecordFeedbackRequest {
    rating: i32,
    feedback: Option<String>,
}

pub async fn record_feedback(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<RecordFeedbackRequest>,
) -> ApiResult<Json<erp_fsm::ServiceOrder>> {
    let order = erp_fsm::FSMService::new(erp_fsm::SqliteFSMRepository::new(state.pool.clone()))
        .record_customer_feedback(id, req.rating, req.feedback)
        .await?;
    Ok(Json(order))
}

pub async fn list_technicians(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<erp_fsm::Technician>>> {
    let techs = erp_fsm::FSMService::new(erp_fsm::SqliteFSMRepository::new(state.pool.clone()))
        .list_available_technicians()
        .await?;
    Ok(Json(techs))
}

#[derive(Debug, Deserialize)]
pub struct CreateTechnicianRequest {
    code: String,
    first_name: String,
    last_name: String,
    phone: String,
    email: Option<String>,
    hourly_rate: i64,
}

pub async fn create_technician(
    State(state): State<AppState>,
    Json(req): Json<CreateTechnicianRequest>,
) -> ApiResult<Json<erp_fsm::Technician>> {
    let tech = erp_fsm::FSMService::new(erp_fsm::SqliteFSMRepository::new(state.pool.clone()))
        .create_technician(req.code, req.first_name, req.last_name, req.phone, req.email, req.hourly_rate)
        .await?;
    Ok(Json(tech))
}

pub async fn optimize_route(
    State(state): State<AppState>,
    Json(req): Json<erp_fsm::OptimizeRouteRequest>,
) -> ApiResult<Json<erp_fsm::RouteOptimizationResult>> {
    let result = erp_fsm::FSMService::new(erp_fsm::SqliteFSMRepository::new(state.pool.clone()))
        .optimize_route(req)
        .await?;
    Ok(Json(result))
}

#[derive(Debug, Deserialize)]
pub struct FindTechnicianRequest {
    lat: f64,
    lng: f64,
    skills: Option<Vec<String>>,
}

pub async fn find_available_technician(
    State(state): State<AppState>,
    Json(req): Json<FindTechnicianRequest>,
) -> ApiResult<Json<Option<erp_fsm::Technician>>> {
    let tech = erp_fsm::FSMService::new(erp_fsm::SqliteFSMRepository::new(state.pool.clone()))
        .find_available_technician(req.lat, req.lng, req.skills)
        .await?;
    Ok(Json(tech))
}

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

pub async fn list_items(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> ApiResult<Json<PaginatedResponse<erp_plm::PLMItem>>> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);
    let status = query.status.and_then(|s| match s.as_str() {
        "Draft" => Some(erp_plm::ItemStatus::Draft),
        "InDesign" => Some(erp_plm::ItemStatus::InDesign),
        "InReview" => Some(erp_plm::ItemStatus::InReview),
        "Approved" => Some(erp_plm::ItemStatus::Approved),
        "Released" => Some(erp_plm::ItemStatus::Released),
        "Obsolete" => Some(erp_plm::ItemStatus::Obsolete),
        _ => None,
    });
    let items = erp_plm::PLMService::new(erp_plm::SqlitePLMRepository::new(state.pool.clone()))
        .list_items(status, page, page_size)
        .await?;
    Ok(Json(PaginatedResponse {
        total: items.len() as i64,
        items,
        page,
        page_size,
    }))
}

pub async fn create_item(
    State(state): State<AppState>,
    Json(req): Json<erp_plm::CreatePLMItemRequest>,
) -> ApiResult<Json<erp_plm::PLMItem>> {
    let item = erp_plm::PLMService::new(erp_plm::SqlitePLMRepository::new(state.pool.clone()))
        .create_item(req)
        .await?;
    Ok(Json(item))
}

pub async fn get_item(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<erp_plm::PLMItem>> {
    let item = erp_plm::PLMService::new(erp_plm::SqlitePLMRepository::new(state.pool.clone()))
        .get_item(id)
        .await?
        .ok_or_else(|| crate::error::ApiError::NotFound("Item not found".into()))?;
    Ok(Json(item))
}

pub async fn release_item(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<erp_plm::PLMItem>> {
    let item = erp_plm::PLMService::new(erp_plm::SqlitePLMRepository::new(state.pool.clone()))
        .release_item(id)
        .await?;
    Ok(Json(item))
}

pub async fn create_ecr(
    State(state): State<AppState>,
    Json(req): Json<erp_plm::CreateECRRequest>,
) -> ApiResult<Json<erp_plm::EngineeringChangeRequest>> {
    let ecr = erp_plm::PLMService::new(erp_plm::SqlitePLMRepository::new(state.pool.clone()))
        .create_ecr(req, Uuid::nil())
        .await?;
    Ok(Json(ecr))
}

pub async fn submit_ecr(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<erp_plm::EngineeringChangeRequest>> {
    let ecr = erp_plm::PLMService::new(erp_plm::SqlitePLMRepository::new(state.pool.clone()))
        .submit_ecr(id)
        .await?;
    Ok(Json(ecr))
}

pub async fn approve_ecr(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<erp_plm::EngineeringChangeRequest>> {
    let ecr = erp_plm::PLMService::new(erp_plm::SqlitePLMRepository::new(state.pool.clone()))
        .approve_ecr(id, Uuid::nil())
        .await?;
    Ok(Json(ecr))
}

#[derive(Debug, Deserialize)]
pub struct RejectEcrRequest {
    reason: String,
}

pub async fn reject_ecr(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<RejectEcrRequest>,
) -> ApiResult<Json<erp_plm::EngineeringChangeRequest>> {
    let ecr = erp_plm::PLMService::new(erp_plm::SqlitePLMRepository::new(state.pool.clone()))
        .reject_ecr(id, req.reason)
        .await?;
    Ok(Json(ecr))
}

#[derive(Debug, Deserialize)]
pub struct CreateBomRequest {
    item_id: Uuid,
    name: String,
    description: Option<String>,
    bom_type: String,
    quantity: f64,
    unit_of_measure: String,
}

pub async fn create_bom(
    State(state): State<AppState>,
    Json(req): Json<CreateBomRequest>,
) -> ApiResult<Json<erp_plm::PLMBOM>> {
    let bom = erp_plm::PLMService::new(erp_plm::SqlitePLMRepository::new(state.pool.clone()))
        .create_bom(req.item_id, req.name, req.description, req.bom_type, req.quantity, req.unit_of_measure)
        .await?;
    Ok(Json(bom))
}

pub async fn create_specification(
    State(state): State<AppState>,
    Json(req): Json<erp_plm::CreateSpecificationRequest>,
) -> ApiResult<Json<erp_plm::Specification>> {
    let spec = erp_plm::PLMService::new(erp_plm::SqlitePLMRepository::new(state.pool.clone()))
        .create_specification(req)
        .await?;
    Ok(Json(spec))
}

#[derive(Debug, Deserialize)]
pub struct CreateDesignReviewRequest {
    item_id: Uuid,
    review_type: String,
    scheduled_date: chrono::DateTime<chrono::Utc>,
}

pub async fn create_design_review(
    State(state): State<AppState>,
    Json(req): Json<CreateDesignReviewRequest>,
) -> ApiResult<Json<erp_plm::DesignReview>> {
    let review = erp_plm::PLMService::new(erp_plm::SqlitePLMRepository::new(state.pool.clone()))
        .create_design_review(req.item_id, req.review_type, req.scheduled_date, None)
        .await?;
    Ok(Json(review))
}

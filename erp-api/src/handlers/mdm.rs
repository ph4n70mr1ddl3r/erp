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
    entity_type: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    items: Vec<T>,
    total: i64,
    page: i32,
    page_size: i32,
}

pub async fn create_golden_record(
    State(state): State<AppState>,
    Json(req): Json<erp_mdm::CreateGoldenRecordRequest>,
) -> ApiResult<Json<erp_mdm::GoldenRecord>> {
    let record = erp_mdm::MDMService::new(erp_mdm::SqliteMDMRepository::new(state.pool.clone()))
        .create_golden_record(req)
        .await?;
    Ok(Json(record))
}

pub async fn get_golden_record(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<erp_mdm::GoldenRecord>> {
    let record = erp_mdm::MDMService::new(erp_mdm::SqliteMDMRepository::new(state.pool.clone()))
        .get_golden_record(id)
        .await?
        .ok_or_else(|| crate::error::ApiError::NotFound("Golden record not found".into()))?;
    Ok(Json(record))
}

pub async fn create_quality_rule(
    State(state): State<AppState>,
    Json(req): Json<erp_mdm::CreateDataQualityRuleRequest>,
) -> ApiResult<Json<erp_mdm::DataQualityRule>> {
    let rule = erp_mdm::MDMService::new(erp_mdm::SqliteMDMRepository::new(state.pool.clone()))
        .create_quality_rule(req)
        .await?;
    Ok(Json(rule))
}

pub async fn run_quality_check(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<Vec<erp_mdm::DataQualityViolation>>> {
    let violations = erp_mdm::MDMService::new(erp_mdm::SqliteMDMRepository::new(state.pool.clone()))
        .run_quality_check(id)
        .await?;
    Ok(Json(violations))
}

#[derive(Debug, Deserialize)]
pub struct ResolveViolationRequest {
    resolved_by: Uuid,
    notes: Option<String>,
}

pub async fn resolve_violation(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<ResolveViolationRequest>,
) -> ApiResult<Json<erp_mdm::DataQualityViolation>> {
    let violation = erp_mdm::MDMService::new(erp_mdm::SqliteMDMRepository::new(state.pool.clone()))
        .resolve_violation(id, req.resolved_by, req.notes)
        .await?;
    Ok(Json(violation))
}

#[derive(Debug, Deserialize)]
pub struct FindDuplicatesRequest {
    entity_type: String,
}

pub async fn find_duplicates(
    State(state): State<AppState>,
    Json(req): Json<FindDuplicatesRequest>,
) -> ApiResult<Json<Vec<erp_mdm::DuplicateRecord>>> {
    let duplicates = erp_mdm::MDMService::new(erp_mdm::SqliteMDMRepository::new(state.pool.clone()))
        .find_duplicates(&req.entity_type)
        .await?;
    Ok(Json(duplicates))
}

pub async fn merge_records(
    State(state): State<AppState>,
    Json(req): Json<erp_mdm::MergeRecordsRequest>,
) -> ApiResult<Json<erp_mdm::GoldenRecord>> {
    let record = erp_mdm::MDMService::new(erp_mdm::SqliteMDMRepository::new(state.pool.clone()))
        .merge_records(req, Uuid::nil())
        .await?;
    Ok(Json(record))
}

pub async fn get_quality_dashboard(
    State(state): State<AppState>,
    Path(entity_type): Path<String>,
) -> ApiResult<Json<erp_mdm::DataQualityDashboard>> {
    let dashboard = erp_mdm::MDMService::new(erp_mdm::SqliteMDMRepository::new(state.pool.clone()))
        .get_quality_dashboard(&entity_type)
        .await?;
    Ok(Json(dashboard))
}

#[derive(Debug, Deserialize)]
pub struct CreateImportJobRequest {
    job_name: String,
    entity_type: String,
    source_file: String,
}

pub async fn create_import_job(
    State(state): State<AppState>,
    Json(req): Json<CreateImportJobRequest>,
) -> ApiResult<Json<erp_mdm::DataImportJob>> {
    let job = erp_mdm::MDMService::new(erp_mdm::SqliteMDMRepository::new(state.pool.clone()))
        .create_import_job(req.job_name, req.entity_type, req.source_file)
        .await?;
    Ok(Json(job))
}

pub async fn start_import_job(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<erp_mdm::DataImportJob>> {
    let job = erp_mdm::MDMService::new(erp_mdm::SqliteMDMRepository::new(state.pool.clone()))
        .start_import_job(id)
        .await?;
    Ok(Json(job))
}

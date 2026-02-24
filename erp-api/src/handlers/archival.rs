use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use crate::db::AppState;
use crate::error::ApiResult;
use erp_enterprise::{ArchivalService, ArchiveRequest, CreateRetentionPolicyRequest, ArchivedRecord, DataRetentionPolicy, ArchiveStats};
use erp_auth::AuthUser;
use uuid::Uuid;

pub async fn list_retention_policies(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<DataRetentionPolicy>>> {
    let svc = ArchivalService::new();
    let policies = svc.list_retention_policies(&state.pool).await?;
    Ok(Json(policies))
}

pub async fn create_retention_policy(
    State(state): State<AppState>,
    Json(req): Json<CreateRetentionPolicyRequest>,
) -> ApiResult<Json<DataRetentionPolicy>> {
    let svc = ArchivalService::new();
    let policy = svc.create_retention_policy(&state.pool, req).await?;
    Ok(Json(policy))
}

pub async fn get_retention_policy(
    State(state): State<AppState>,
    Path(entity_type): Path<String>,
) -> ApiResult<Json<DataRetentionPolicy>> {
    let svc = ArchivalService::new();
    let policy = svc.get_retention_policy(&state.pool, &entity_type).await?
        .ok_or_else(|| erp_core::Error::not_found("RetentionPolicy", &entity_type))?;
    Ok(Json(policy))
}

#[derive(Debug, Deserialize)]
pub struct ListArchivedQuery {
    pub entity_type: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

pub async fn list_archived_records(
    State(state): State<AppState>,
    Query(query): Query<ListArchivedQuery>,
) -> ApiResult<Json<Vec<ArchivedRecord>>> {
    let svc = ArchivalService::new();
    let records = svc.list_archived_records(
        &state.pool,
        query.entity_type.as_deref(),
        query.limit.unwrap_or(50),
        query.offset.unwrap_or(0),
    ).await?;
    Ok(Json(records))
}

pub async fn get_archived_record(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ArchivedRecord>> {
    let svc = ArchivalService::new();
    let record = svc.get_archived_record(&state.pool, id).await?;
    Ok(Json(record))
}

pub async fn archive_record(
    State(state): State<AppState>,
    axum::Extension(AuthUser(user)): axum::Extension<AuthUser>,
    Json(req): Json<ArchiveRequest>,
) -> ApiResult<Json<ArchivedRecord>> {
    let user_id = Uuid::parse_str(&user.user_id).map_err(|_| erp_core::Error::Unauthorized)?;
    let svc = ArchivalService::new();
    let record = svc.archive_record(&state.pool, req, user_id).await?;
    Ok(Json(record))
}

pub async fn restore_record(
    State(state): State<AppState>,
    axum::Extension(AuthUser(user)): axum::Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ArchivedRecord>> {
    let user_id = Uuid::parse_str(&user.user_id).map_err(|_| erp_core::Error::Unauthorized)?;
    let svc = ArchivalService::new();
    let record = svc.restore_record(&state.pool, id, user_id).await?;
    Ok(Json(record))
}

pub async fn delete_archived_record(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let svc = ArchivalService::new();
    svc.delete_archived_record(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn purge_expired(
    State(state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    let svc = ArchivalService::new();
    let count = svc.purge_expired_records(&state.pool).await?;
    Ok(Json(serde_json::json!({ "purged_count": count })))
}

pub async fn archival_stats(
    State(state): State<AppState>,
) -> ApiResult<Json<ArchiveStats>> {
    let svc = ArchivalService::new();
    let stats = svc.get_stats(&state.pool).await?;
    Ok(Json(stats))
}

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use crate::db::AppState;
use crate::error::ApiResult;
use erp_enterprise::{BulkOperationsService, BulkOperation};
use erp_auth::AuthUser;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ListOperationsQuery {
    pub status: Option<String>,
    pub limit: Option<i32>,
}

pub async fn list_operations(
    State(state): State<AppState>,
    Query(query): Query<ListOperationsQuery>,
) -> ApiResult<Json<Vec<BulkOperation>>> {
    let svc = BulkOperationsService::new();
    let operations = svc.list_operations(&state.pool, query.status.as_deref(), query.limit.unwrap_or(50)).await?;
    Ok(Json(operations))
}

pub async fn get_operation(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<BulkOperation>> {
    let svc = BulkOperationsService::new();
    let operation = svc.get_operation(&state.pool, id).await?;
    Ok(Json(operation))
}

#[derive(Debug, Deserialize)]
pub struct CreateOperationBody {
    pub operation_type: String,
    pub entity_type: String,
    pub total_count: i32,
}

pub async fn create_operation(
    State(state): State<AppState>,
    axum::Extension(AuthUser(user)): axum::Extension<AuthUser>,
    Json(body): Json<CreateOperationBody>,
) -> ApiResult<Json<BulkOperation>> {
    let user_id = Uuid::parse_str(&user.user_id).map_err(|_| erp_core::Error::Unauthorized)?;
    let svc = BulkOperationsService::new();
    let operation = svc.create_operation(
        &state.pool,
        &body.operation_type,
        &body.entity_type,
        body.total_count,
        user_id,
    ).await?;
    Ok(Json(operation))
}

pub async fn cancel_operation(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<BulkOperation>> {
    let svc = BulkOperationsService::new();
    let operation = svc.cancel_operation(&state.pool, id).await?;
    Ok(Json(operation))
}

#[derive(Debug, Deserialize)]
pub struct CleanupQuery {
    pub days_old: Option<i32>,
}

pub async fn cleanup_operations(
    State(state): State<AppState>,
    Query(query): Query<CleanupQuery>,
) -> ApiResult<Json<serde_json::Value>> {
    let svc = BulkOperationsService::new();
    let count = svc.cleanup_old_operations(&state.pool, query.days_old.unwrap_or(30)).await?;
    Ok(Json(serde_json::json!({ "deleted_count": count })))
}

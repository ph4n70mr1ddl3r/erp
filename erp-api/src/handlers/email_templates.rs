use axum::{
    extract::{Path, State},
    Json,
};
use serde::Deserialize;
use crate::db::AppState;
use crate::error::ApiResult;
use erp_enterprise::{EmailTemplateService, CreateEmailTemplateRequest, UpdateEmailTemplateRequest, QueueEmailRequest};

pub async fn list_templates(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<erp_enterprise::EmailTemplate>>> {
    let svc = EmailTemplateService::new();
    let templates = svc.list_templates(&state.pool).await?;
    Ok(Json(templates))
}

pub async fn get_template(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> ApiResult<Json<erp_enterprise::EmailTemplate>> {
    let svc = EmailTemplateService::new();
    let template = svc.get_template(&state.pool, &name).await?;
    Ok(Json(template))
}

pub async fn create_template(
    State(state): State<AppState>,
    Json(req): Json<CreateEmailTemplateRequest>,
) -> ApiResult<Json<erp_enterprise::EmailTemplate>> {
    let svc = EmailTemplateService::new();
    let template = svc.create_template(&state.pool, req).await?;
    Ok(Json(template))
}

pub async fn update_template(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Json(req): Json<UpdateEmailTemplateRequest>,
) -> ApiResult<Json<erp_enterprise::EmailTemplate>> {
    let svc = EmailTemplateService::new();
    let template = svc.update_template(&state.pool, &name, req).await?;
    Ok(Json(template))
}

pub async fn delete_template(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let svc = EmailTemplateService::new();
    svc.delete_template(&state.pool, &name).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn queue_email(
    State(state): State<AppState>,
    Json(req): Json<QueueEmailRequest>,
) -> ApiResult<Json<erp_enterprise::EmailQueueItem>> {
    let svc = EmailTemplateService::new();
    let item = svc.queue_email(&state.pool, req).await?;
    Ok(Json(item))
}

#[derive(Debug, Deserialize)]
pub struct GetPendingQuery {
    pub limit: Option<i32>,
}

pub async fn get_pending_emails(
    State(state): State<AppState>,
    Query(query): Query<GetPendingQuery>,
) -> ApiResult<Json<Vec<erp_enterprise::EmailQueueItem>>> {
    let svc = EmailTemplateService::new();
    let items = svc.get_pending_emails(&state.pool, query.limit.unwrap_or(100)).await?;
    Ok(Json(items))
}

pub async fn get_email_queue_stats(
    State(state): State<AppState>,
) -> ApiResult<Json<erp_enterprise::EmailQueueStats>> {
    let svc = EmailTemplateService::new();
    let stats = svc.get_queue_stats(&state.pool).await?;
    Ok(Json(stats))
}

use axum::extract::Query;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::{Pagination, WorkflowService, ApprovalService, NotificationService, Workflow, ApprovalRequest, Approval, Notification, NotificationType, WorkflowStatus, ApprovalStatus};

#[derive(Debug, Deserialize)]
pub struct CreateWorkflowRequest {
    pub name: String,
    pub entity_type: String,
    pub approval_levels: u32,
    pub auto_approve_below: Option<i64>,
    pub require_comment: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct WorkflowResponse {
    pub id: Uuid,
    pub name: String,
    pub entity_type: String,
    pub approval_levels: u32,
    pub auto_approve_below: i64,
    pub require_comment: bool,
    pub status: String,
}

impl From<Workflow> for WorkflowResponse {
    fn from(w: Workflow) -> Self {
        Self {
            id: w.id,
            name: w.name,
            entity_type: w.entity_type,
            approval_levels: w.approval_levels,
            auto_approve_below: w.auto_approve_below,
            require_comment: w.require_comment,
            status: format!("{:?}", w.status),
        }
    }
}

pub async fn list_workflows(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<WorkflowResponse>>> {
    let workflows = WorkflowService::list_workflows(&state.pool).await?;
    Ok(Json(workflows.into_iter().map(WorkflowResponse::from).collect()))
}

pub async fn create_workflow(
    State(state): State<AppState>,
    Json(req): Json<CreateWorkflowRequest>,
) -> ApiResult<Json<WorkflowResponse>> {
    let workflow = Workflow {
        id: Uuid::new_v4(),
        name: req.name,
        entity_type: req.entity_type,
        approval_levels: req.approval_levels,
        auto_approve_below: req.auto_approve_below.unwrap_or(0),
        require_comment: req.require_comment.unwrap_or(false),
        status: WorkflowStatus::Active,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    let created = WorkflowService::create_workflow(&state.pool, workflow).await?;
    Ok(Json(WorkflowResponse::from(created)))
}

#[derive(Debug, Serialize)]
pub struct ApprovalRequestResponse {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub entity_type: String,
    pub entity_id: String,
    pub current_level: u32,
    pub max_level: u32,
    pub status: String,
    pub requested_by: Option<Uuid>,
    pub requested_at: String,
    pub approvals: Vec<ApprovalResponse>,
}

#[derive(Debug, Serialize)]
pub struct ApprovalResponse {
    pub level: u32,
    pub approver_name: Option<String>,
    pub status: String,
    pub comment: Option<String>,
    pub created_at: String,
}

impl From<Approval> for ApprovalResponse {
    fn from(a: Approval) -> Self {
        Self {
            level: a.level,
            approver_name: a.approver_name,
            status: format!("{:?}", a.status),
            comment: a.comment,
            created_at: a.created_at.to_rfc3339(),
        }
    }
}

pub async fn list_pending_approvals(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> ApiResult<Json<erp_core::Paginated<ApprovalRequestResponse>>> {
    let result = ApprovalService::list_pending(&state.pool, pagination).await?;
    
    let mut responses = Vec::new();
    for req in result.items {
        let approvals = ApprovalService::get_approvals(&state.pool, req.id).await?;
        responses.push(ApprovalRequestResponse {
            id: req.id,
            workflow_id: req.workflow_id,
            entity_type: req.entity_type,
            entity_id: req.entity_id,
            current_level: req.current_level,
            max_level: req.max_level,
            status: format!("{:?}", req.status),
            requested_by: req.requested_by,
            requested_at: req.requested_at.to_rfc3339(),
            approvals: approvals.into_iter().map(ApprovalResponse::from).collect(),
        });
    }
    
    Ok(Json(erp_core::Paginated::new(
        responses,
        result.total,
        Pagination { page: result.page, per_page: result.per_page },
    )))
}

#[derive(Debug, Deserialize)]
pub struct ApprovalActionRequest {
    pub comment: Option<String>,
}

pub async fn approve_request(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<ApprovalActionRequest>,
) -> ApiResult<Json<ApprovalRequestResponse>> {
    let approver_id = Uuid::nil();
    let approver_name = "Admin";
    
    let updated = ApprovalService::approve(&state.pool, id, approver_id, approver_name, req.comment.as_deref()).await?;
    
    if updated.status == ApprovalStatus::Approved {
        let _ = NotificationService::create(
            &state.pool,
            updated.requested_by.unwrap_or_default(),
            "Request Approved",
            &format!("Your {} has been approved", updated.entity_type),
            Some(&updated.entity_type),
            Some(&updated.entity_id),
            NotificationType::ApprovalApproved,
        ).await;
    }
    
    let approvals = ApprovalService::get_approvals(&state.pool, updated.id).await?;
    
    Ok(Json(ApprovalRequestResponse {
        id: updated.id,
        workflow_id: updated.workflow_id,
        entity_type: updated.entity_type,
        entity_id: updated.entity_id,
        current_level: updated.current_level,
        max_level: updated.max_level,
        status: format!("{:?}", updated.status),
        requested_by: updated.requested_by,
        requested_at: updated.requested_at.to_rfc3339(),
        approvals: approvals.into_iter().map(ApprovalResponse::from).collect(),
    }))
}

pub async fn reject_request(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<ApprovalActionRequest>,
) -> ApiResult<Json<ApprovalRequestResponse>> {
    let approver_id = Uuid::nil();
    let approver_name = "Admin";
    let comment = req.comment.as_deref().unwrap_or("");
    
    let updated = ApprovalService::reject(&state.pool, id, approver_id, approver_name, comment).await?;
    
    let _ = NotificationService::create(
        &state.pool,
        updated.requested_by.unwrap_or_default(),
        "Request Rejected",
        &format!("Your {} has been rejected", updated.entity_type),
        Some(&updated.entity_type),
        Some(&updated.entity_id),
        NotificationType::ApprovalRejected,
    ).await;
    
    let approvals = ApprovalService::get_approvals(&state.pool, updated.id).await?;
    
    Ok(Json(ApprovalRequestResponse {
        id: updated.id,
        workflow_id: updated.workflow_id,
        entity_type: updated.entity_type,
        entity_id: updated.entity_id,
        current_level: updated.current_level,
        max_level: updated.max_level,
        status: format!("{:?}", updated.status),
        requested_by: updated.requested_by,
        requested_at: updated.requested_at.to_rfc3339(),
        approvals: approvals.into_iter().map(ApprovalResponse::from).collect(),
    }))
}

#[derive(Debug, Serialize)]
pub struct NotificationResponse {
    pub id: Uuid,
    pub title: String,
    pub message: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub notification_type: String,
    pub read: bool,
    pub created_at: String,
}

impl From<Notification> for NotificationResponse {
    fn from(n: Notification) -> Self {
        Self {
            id: n.id,
            title: n.title,
            message: n.message,
            entity_type: n.entity_type,
            entity_id: n.entity_id,
            notification_type: format!("{:?}", n.notification_type),
            read: n.read,
            created_at: n.created_at.to_rfc3339(),
        }
    }
}

pub async fn list_notifications(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<NotificationResponse>>> {
    let user_id = Uuid::nil();
    let notifications = NotificationService::list_for_user(&state.pool, user_id, false).await?;
    Ok(Json(notifications.into_iter().map(NotificationResponse::from).collect()))
}

pub async fn mark_notification_read(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    NotificationService::mark_read(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "ok" })))
}

pub async fn mark_all_notifications_read(
    State(state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    let user_id = Uuid::nil();
    NotificationService::mark_all_read(&state.pool, user_id).await?;
    Ok(Json(serde_json::json!({ "status": "ok" })))
}

pub async fn unread_notification_count(
    State(state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    let user_id = Uuid::nil();
    let count = NotificationService::unread_count(&state.pool, user_id).await?;
    Ok(Json(serde_json::json!({ "count": count })))
}

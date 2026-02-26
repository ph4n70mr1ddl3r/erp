use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::Pagination;
use erp_approval_workflow::{
    ApprovalWorkflow, ApprovalWorkflowService, ApprovalWorkflowStatus, ApprovalType,
    ApprovalRequest, ApprovalRequestService, ApprovalRequestStatus,
    CreateWorkflowRequest, CreateLevelRequest, UpdateWorkflowRequest,
    SubmitApprovalRequest, ApproveRequest, RejectRequest,
    PendingApprovalSummary,
};

#[derive(Debug, Serialize)]
pub struct WorkflowResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub document_type: String,
    pub approval_type: String,
    pub min_amount: Option<i64>,
    pub max_amount: Option<i64>,
    pub auto_approve_below: Option<i64>,
    pub escalation_hours: Option<i32>,
    pub notify_requester: bool,
    pub notify_approver: bool,
    pub allow_delegation: bool,
    pub allow_reassignment: bool,
    pub require_comments: bool,
    pub status: String,
    pub levels: Vec<LevelResponse>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct LevelResponse {
    pub id: Uuid,
    pub level_number: i32,
    pub name: String,
    pub description: Option<String>,
    pub approver_type: String,
    pub approver_ids: Vec<Uuid>,
    pub min_approvers: i32,
    pub skip_if_approved_above: bool,
    pub due_hours: Option<i32>,
    pub escalation_to: Option<Uuid>,
}

impl From<ApprovalWorkflow> for WorkflowResponse {
    fn from(w: ApprovalWorkflow) -> Self {
        Self {
            id: w.base.id,
            code: w.code,
            name: w.name,
            description: w.description,
            document_type: w.document_type,
            approval_type: format!("{:?}", w.approval_type),
            min_amount: w.min_amount,
            max_amount: w.max_amount,
            auto_approve_below: w.auto_approve_below,
            escalation_hours: w.escalation_hours,
            notify_requester: w.notify_requester,
            notify_approver: w.notify_approver,
            allow_delegation: w.allow_delegation,
            allow_reassignment: w.allow_reassignment,
            require_comments: w.require_comments,
            status: format!("{:?}", w.status),
            levels: w.levels.into_iter().map(|l| LevelResponse {
                id: l.id,
                level_number: l.level_number,
                name: l.name,
                description: l.description,
                approver_type: format!("{:?}", l.approver_type),
                approver_ids: l.approver_ids,
                min_approvers: l.min_approvers,
                skip_if_approved_above: l.skip_if_approved_above,
                due_hours: l.due_hours,
                escalation_to: l.escalation_to,
            }).collect(),
            created_at: w.base.created_at.to_rfc3339(),
            updated_at: w.base.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RequestResponse {
    pub id: Uuid,
    pub request_number: String,
    pub workflow_id: Uuid,
    pub document_type: String,
    pub document_id: Uuid,
    pub document_number: String,
    pub requested_by: Uuid,
    pub requested_at: String,
    pub amount: i64,
    pub currency: String,
    pub status: String,
    pub current_level: Option<i32>,
    pub due_date: Option<String>,
    pub approved_at: Option<String>,
    pub approved_by: Option<Uuid>,
    pub rejected_at: Option<String>,
    pub rejected_by: Option<Uuid>,
    pub rejection_reason: Option<String>,
    pub approvals: Vec<ApprovalRecordResponse>,
}

#[derive(Debug, Serialize)]
pub struct ApprovalRecordResponse {
    pub id: Uuid,
    pub level_number: i32,
    pub approver_id: Uuid,
    pub action: String,
    pub comments: Option<String>,
    pub delegated_to: Option<Uuid>,
    pub created_at: String,
}

impl From<ApprovalRequest> for RequestResponse {
    fn from(r: ApprovalRequest) -> Self {
        Self {
            id: r.id,
            request_number: r.request_number,
            workflow_id: r.workflow_id,
            document_type: r.document_type,
            document_id: r.document_id,
            document_number: r.document_number,
            requested_by: r.requested_by,
            requested_at: r.requested_at.to_rfc3339(),
            amount: r.amount,
            currency: r.currency,
            status: format!("{:?}", r.status),
            current_level: r.current_level,
            due_date: r.due_date.map(|d| d.to_rfc3339()),
            approved_at: r.approved_at.map(|d| d.to_rfc3339()),
            approved_by: r.approved_by,
            rejected_at: r.rejected_at.map(|d| d.to_rfc3339()),
            rejected_by: r.rejected_by,
            rejection_reason: r.rejection_reason,
            approvals: r.approvals.into_iter().map(|a| ApprovalRecordResponse {
                id: a.id,
                level_number: a.level_number,
                approver_id: a.approver_id,
                action: format!("{:?}", a.action),
                comments: a.comments,
                delegated_to: a.delegated_to,
                created_at: a.created_at.to_rfc3339(),
            }).collect(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateWorkflowHandlerRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub document_type: String,
    pub approval_type: Option<String>,
    pub min_amount: Option<i64>,
    pub max_amount: Option<i64>,
    pub auto_approve_below: Option<i64>,
    pub escalation_hours: Option<i32>,
    pub notify_requester: Option<bool>,
    pub notify_approver: Option<bool>,
    pub allow_delegation: Option<bool>,
    pub allow_reassignment: Option<bool>,
    pub require_comments: Option<bool>,
    pub levels: Vec<CreateLevelHandlerRequest>,
}

#[derive(Debug, Deserialize)]
pub struct CreateLevelHandlerRequest {
    pub name: String,
    pub description: Option<String>,
    pub approver_type: Option<String>,
    pub approver_ids: Vec<Uuid>,
    pub min_approvers: Option<i32>,
    pub skip_if_approved_above: Option<bool>,
    pub due_hours: Option<i32>,
    pub escalation_to: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateWorkflowHandlerRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub approval_type: Option<String>,
    pub min_amount: Option<i64>,
    pub max_amount: Option<i64>,
    pub auto_approve_below: Option<i64>,
    pub escalation_hours: Option<i32>,
    pub notify_requester: Option<bool>,
    pub notify_approver: Option<bool>,
    pub allow_delegation: Option<bool>,
    pub allow_reassignment: Option<bool>,
    pub require_comments: Option<bool>,
    pub status: Option<String>,
    pub levels: Option<Vec<CreateLevelHandlerRequest>>,
}

#[derive(Debug, Deserialize)]
pub struct SubmitApprovalHandlerRequest {
    pub document_type: String,
    pub document_id: Uuid,
    pub document_number: String,
    pub requested_by: Uuid,
    pub amount: i64,
    pub currency: Option<String>,
}

pub async fn list_workflows(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> ApiResult<Json<erp_core::Paginated<WorkflowResponse>>> {
    let service = ApprovalWorkflowService::new();
    let result = service.list_workflows(&state.pool, pagination).await?;
    
    Ok(Json(erp_core::Paginated::new(
        result.items.into_iter().map(WorkflowResponse::from).collect(),
        result.total,
        Pagination { page: result.page, per_page: result.per_page }
    )))
}

pub async fn get_workflow(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<WorkflowResponse>> {
    let service = ApprovalWorkflowService::new();
    let workflow = service.get_workflow(&state.pool, id).await?;
    Ok(Json(WorkflowResponse::from(workflow)))
}

pub async fn create_workflow(
    State(state): State<AppState>,
    Json(req): Json<CreateWorkflowHandlerRequest>,
) -> ApiResult<Json<WorkflowResponse>> {
    let service = ApprovalWorkflowService::new();
    
    let workflow_req = CreateWorkflowRequest {
        code: req.code,
        name: req.name,
        description: req.description,
        document_type: req.document_type,
        approval_type: match req.approval_type.as_deref() {
            Some("AnyApprover") => ApprovalType::AnyApprover,
            Some("AllApprovers") => ApprovalType::AllApprovers,
            _ => ApprovalType::Sequential,
        },
        min_amount: req.min_amount,
        max_amount: req.max_amount,
        auto_approve_below: req.auto_approve_below,
        escalation_hours: req.escalation_hours,
        notify_requester: req.notify_requester,
        notify_approver: req.notify_approver,
        allow_delegation: req.allow_delegation,
        allow_reassignment: req.allow_reassignment,
        require_comments: req.require_comments,
        levels: req.levels.into_iter().map(|l| CreateLevelRequest {
            name: l.name,
            description: l.description,
            approver_type: match l.approver_type.as_deref() {
                Some("Role") => erp_approval_workflow::ApproverType::Role,
                Some("Department") => erp_approval_workflow::ApproverType::Department,
                Some("Supervisor") => erp_approval_workflow::ApproverType::Supervisor,
                Some("AmountBased") => erp_approval_workflow::ApproverType::AmountBased,
                _ => erp_approval_workflow::ApproverType::SpecificUser,
            },
            approver_ids: l.approver_ids,
            min_approvers: l.min_approvers,
            skip_if_approved_above: l.skip_if_approved_above,
            due_hours: l.due_hours,
            escalation_to: l.escalation_to,
        }).collect(),
    };
    
    let workflow = service.create_workflow(&state.pool, workflow_req).await?;
    Ok(Json(WorkflowResponse::from(workflow)))
}

pub async fn update_workflow(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateWorkflowHandlerRequest>,
) -> ApiResult<Json<WorkflowResponse>> {
    let service = ApprovalWorkflowService::new();
    
    let update_req = UpdateWorkflowRequest {
        name: req.name,
        description: req.description,
        approval_type: req.approval_type.map(|t| match t.as_str() {
            "AnyApprover" => ApprovalType::AnyApprover,
            "AllApprovers" => ApprovalType::AllApprovers,
            _ => ApprovalType::Sequential,
        }),
        min_amount: req.min_amount,
        max_amount: req.max_amount,
        auto_approve_below: req.auto_approve_below,
        escalation_hours: req.escalation_hours,
        notify_requester: req.notify_requester,
        notify_approver: req.notify_approver,
        allow_delegation: req.allow_delegation,
        allow_reassignment: req.allow_reassignment,
        require_comments: req.require_comments,
        status: req.status.map(|s| match s.as_str() {
            "Inactive" => ApprovalWorkflowStatus::Inactive,
            _ => ApprovalWorkflowStatus::Active,
        }),
        levels: req.levels.map(|levels| levels.into_iter().map(|l| CreateLevelRequest {
            name: l.name,
            description: l.description,
            approver_type: match l.approver_type.as_deref() {
                Some("Role") => erp_approval_workflow::ApproverType::Role,
                Some("Department") => erp_approval_workflow::ApproverType::Department,
                Some("Supervisor") => erp_approval_workflow::ApproverType::Supervisor,
                Some("AmountBased") => erp_approval_workflow::ApproverType::AmountBased,
                _ => erp_approval_workflow::ApproverType::SpecificUser,
            },
            approver_ids: l.approver_ids,
            min_approvers: l.min_approvers,
            skip_if_approved_above: l.skip_if_approved_above,
            due_hours: l.due_hours,
            escalation_to: l.escalation_to,
        }).collect()),
    };
    
    let workflow = service.update_workflow(&state.pool, id, update_req).await?;
    Ok(Json(WorkflowResponse::from(workflow)))
}

pub async fn delete_workflow(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = ApprovalWorkflowService::new();
    service.delete_workflow(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn list_requests(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> ApiResult<Json<erp_core::Paginated<RequestResponse>>> {
    let service = ApprovalRequestService::new();
    let result = service.list_requests(&state.pool, pagination).await?;
    
    Ok(Json(erp_core::Paginated::new(
        result.items.into_iter().map(RequestResponse::from).collect(),
        result.total,
        Pagination { page: result.page, per_page: result.per_page }
    )))
}

pub async fn get_request(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<RequestResponse>> {
    let service = ApprovalRequestService::new();
    let request = service.get_request(&state.pool, id).await?;
    Ok(Json(RequestResponse::from(request)))
}

pub async fn get_pending_approvals(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Query(pagination): Query<Pagination>,
) -> ApiResult<Json<erp_core::Paginated<RequestResponse>>> {
    let service = ApprovalRequestService::new();
    let result = service.get_pending_for_approver(&state.pool, user_id, pagination).await?;
    
    Ok(Json(erp_core::Paginated::new(
        result.items.into_iter().map(RequestResponse::from).collect(),
        result.total,
        Pagination { page: result.page, per_page: result.per_page }
    )))
}

pub async fn get_pending_summary(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> ApiResult<Json<PendingApprovalSummary>> {
    let service = ApprovalRequestService::new();
    let summary = service.get_pending_summary(&state.pool, user_id).await?;
    Ok(Json(summary))
}

pub async fn submit_for_approval(
    State(state): State<AppState>,
    Json(req): Json<SubmitApprovalHandlerRequest>,
) -> ApiResult<Json<RequestResponse>> {
    let service = ApprovalRequestService::new();
    
    let submit_req = SubmitApprovalRequest {
        document_type: req.document_type,
        document_id: req.document_id,
        document_number: req.document_number,
        requested_by: req.requested_by,
        amount: req.amount,
        currency: req.currency.unwrap_or_else(|| "USD".to_string()),
    };
    
    let request = service.submit_for_approval(&state.pool, submit_req).await?;
    Ok(Json(RequestResponse::from(request)))
}

pub async fn approve_request(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<ApproveRequest>,
) -> ApiResult<Json<RequestResponse>> {
    let service = ApprovalRequestService::new();
    let request = service.approve(&state.pool, id, req.approver_id, req.comments).await?;
    Ok(Json(RequestResponse::from(request)))
}

pub async fn reject_request(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<RejectRequest>,
) -> ApiResult<Json<RequestResponse>> {
    let service = ApprovalRequestService::new();
    let request = service.reject(&state.pool, id, req.approver_id, req.reason).await?;
    Ok(Json(RequestResponse::from(request)))
}

pub async fn cancel_request(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<RequestResponse>> {
    let service = ApprovalRequestService::new();
    let request = service.get_request(&state.pool, id).await?;
    let request = service.cancel(&state.pool, id, request.requested_by).await?;
    Ok(Json(RequestResponse::from(request)))
}

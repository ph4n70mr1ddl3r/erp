use chrono::Utc;
use erp_core::{BaseEntity, Error, Paginated, Pagination, Result};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;
use crate::repository::*;

pub struct ApprovalWorkflowService {
    workflow_repo: SqliteApprovalWorkflowRepository,
    request_repo: SqliteApprovalRequestRepository,
}

impl ApprovalWorkflowService {
    pub fn new() -> Self {
        Self {
            workflow_repo: SqliteApprovalWorkflowRepository,
            request_repo: SqliteApprovalRequestRepository,
        }
    }

    pub async fn get_workflow(&self, pool: &SqlitePool, id: Uuid) -> Result<ApprovalWorkflow> {
        self.workflow_repo.find_by_id(pool, id).await
    }

    pub async fn list_workflows(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<ApprovalWorkflow>> {
        self.workflow_repo.find_all(pool, pagination).await
    }

    pub async fn create_workflow(&self, pool: &SqlitePool, req: CreateWorkflowRequest) -> Result<ApprovalWorkflow> {
        if req.code.is_empty() {
            return Err(Error::validation("Workflow code is required"));
        }
        if req.name.is_empty() {
            return Err(Error::validation("Workflow name is required"));
        }
        if req.document_type.is_empty() {
            return Err(Error::validation("Document type is required"));
        }
        if req.levels.is_empty() {
            return Err(Error::validation("At least one approval level is required"));
        }

        let workflow = ApprovalWorkflow {
            base: BaseEntity::new(),
            code: req.code,
            name: req.name,
            description: req.description,
            document_type: req.document_type,
            approval_type: req.approval_type,
            min_amount: req.min_amount,
            max_amount: req.max_amount,
            auto_approve_below: req.auto_approve_below,
            escalation_hours: req.escalation_hours,
            notify_requester: req.notify_requester.unwrap_or(true),
            notify_approver: req.notify_approver.unwrap_or(true),
            allow_delegation: req.allow_delegation.unwrap_or(true),
            allow_reassignment: req.allow_reassignment.unwrap_or(false),
            require_comments: req.require_comments.unwrap_or(false),
            status: ApprovalWorkflowStatus::Active,
            levels: req.levels.into_iter().enumerate().map(|(i, l)| ApprovalLevel {
                id: Uuid::new_v4(),
                workflow_id: Uuid::nil(),
                level_number: (i + 1) as i32,
                name: l.name,
                description: l.description,
                approver_type: l.approver_type,
                approver_ids: l.approver_ids,
                min_approvers: l.min_approvers.unwrap_or(1),
                skip_if_approved_above: l.skip_if_approved_above.unwrap_or(false),
                due_hours: l.due_hours,
                escalation_to: l.escalation_to,
            }).collect(),
        };

        self.workflow_repo.create(pool, workflow).await
    }

    pub async fn update_workflow(&self, pool: &SqlitePool, id: Uuid, req: UpdateWorkflowRequest) -> Result<ApprovalWorkflow> {
        let mut workflow = self.workflow_repo.find_by_id(pool, id).await?;

        if let Some(name) = req.name { workflow.name = name; }
        if let Some(description) = req.description { workflow.description = Some(description); }
        if let Some(approval_type) = req.approval_type { workflow.approval_type = approval_type; }
        if let Some(min_amount) = req.min_amount { workflow.min_amount = Some(min_amount); }
        if let Some(max_amount) = req.max_amount { workflow.max_amount = Some(max_amount); }
        if let Some(auto_approve_below) = req.auto_approve_below { workflow.auto_approve_below = Some(auto_approve_below); }
        if let Some(escalation_hours) = req.escalation_hours { workflow.escalation_hours = Some(escalation_hours); }
        if let Some(notify_requester) = req.notify_requester { workflow.notify_requester = notify_requester; }
        if let Some(notify_approver) = req.notify_approver { workflow.notify_approver = notify_approver; }
        if let Some(allow_delegation) = req.allow_delegation { workflow.allow_delegation = allow_delegation; }
        if let Some(allow_reassignment) = req.allow_reassignment { workflow.allow_reassignment = allow_reassignment; }
        if let Some(require_comments) = req.require_comments { workflow.require_comments = require_comments; }
        if let Some(status) = req.status { workflow.status = status; }

        if let Some(levels) = req.levels {
            workflow.levels = levels.into_iter().enumerate().map(|(i, l)| ApprovalLevel {
                id: Uuid::new_v4(),
                workflow_id: id,
                level_number: (i + 1) as i32,
                name: l.name,
                description: l.description,
                approver_type: l.approver_type,
                approver_ids: l.approver_ids,
                min_approvers: l.min_approvers.unwrap_or(1),
                skip_if_approved_above: l.skip_if_approved_above.unwrap_or(false),
                due_hours: l.due_hours,
                escalation_to: l.escalation_to,
            }).collect();
        }

        self.workflow_repo.update(pool, workflow).await
    }

    pub async fn delete_workflow(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.workflow_repo.delete(pool, id).await
    }
}

pub struct ApprovalRequestService {
    workflow_repo: SqliteApprovalWorkflowRepository,
    request_repo: SqliteApprovalRequestRepository,
}

impl ApprovalRequestService {
    pub fn new() -> Self {
        Self {
            workflow_repo: SqliteApprovalWorkflowRepository,
            request_repo: SqliteApprovalRequestRepository,
        }
    }

    pub async fn submit_for_approval(
        &self,
        pool: &SqlitePool,
        req: SubmitApprovalRequest,
    ) -> Result<ApprovalRequest> {
        let workflows = self.workflow_repo.find_by_document_type(pool, &req.document_type).await?;
        
        let workflow = workflows.into_iter()
            .filter(|w| {
                let amount_ok = match (w.min_amount, w.max_amount) {
                    (None, None) => true,
                    (Some(min), None) => req.amount >= min,
                    (None, Some(max)) => req.amount <= max,
                    (Some(min), Some(max)) => req.amount >= min && req.amount <= max,
                };
                amount_ok && w.status == ApprovalWorkflowStatus::Active
            })
            .next()
            .ok_or_else(|| Error::business_rule("No applicable approval workflow found"))?;

        if let Some(auto_approve) = workflow.auto_approve_below {
            if req.amount < auto_approve {
                let request = ApprovalRequest {
                    id: Uuid::new_v4(),
                    request_number: format!("APR-{}", Utc::now().format("%Y%m%d%H%M%S")),
                    workflow_id: workflow.base.id,
                    document_type: req.document_type,
                    document_id: req.document_id,
                    document_number: req.document_number,
                    requested_by: req.requested_by,
                    requested_at: Utc::now(),
                    amount: req.amount,
                    currency: req.currency,
                    status: ApprovalRequestStatus::Approved,
                    current_level: None,
                    due_date: None,
                    approved_at: Some(Utc::now()),
                    approved_by: Some(req.requested_by),
                    rejected_at: None,
                    rejected_by: None,
                    rejection_reason: None,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    approvals: vec![],
                };
                
                return self.request_repo.create(pool, request).await;
            }
        }

        let first_level = workflow.levels.first()
            .ok_or_else(|| Error::business_rule("Workflow has no approval levels"))?;

        let due_date = first_level.due_hours.map(|h| Utc::now() + chrono::Duration::hours(h as i64));

        let request = ApprovalRequest {
            id: Uuid::new_v4(),
            request_number: format!("APR-{}", Utc::now().format("%Y%m%d%H%M%S")),
            workflow_id: workflow.base.id,
            document_type: req.document_type,
            document_id: req.document_id,
            document_number: req.document_number,
            requested_by: req.requested_by,
            requested_at: Utc::now(),
            amount: req.amount,
            currency: req.currency,
            status: ApprovalRequestStatus::Pending,
            current_level: Some(1),
            due_date,
            approved_at: None,
            approved_by: None,
            rejected_at: None,
            rejected_by: None,
            rejection_reason: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            approvals: vec![],
        };

        self.request_repo.create(pool, request).await
    }

    pub async fn approve(
        &self,
        pool: &SqlitePool,
        request_id: Uuid,
        approver_id: Uuid,
        comments: Option<String>,
    ) -> Result<ApprovalRequest> {
        let mut request = self.request_repo.find_by_id(pool, request_id).await?;

        if request.status != ApprovalRequestStatus::Pending {
            return Err(Error::business_rule("Request is not pending approval"));
        }

        let workflow = self.workflow_repo.find_by_id(pool, request.workflow_id).await?;
        let current_level = request.current_level.unwrap_or(1);
        
        let level = workflow.levels.iter()
            .find(|l| l.level_number == current_level)
            .ok_or_else(|| Error::business_rule("Invalid approval level"))?;

        let record = ApprovalRecord {
            id: Uuid::new_v4(),
            request_id: request.id,
            level_number: current_level,
            approver_id,
            action: ApprovalAction::Approved,
            comments,
            delegated_to: None,
            created_at: Utc::now(),
        };
        request.approvals.push(record.clone());
        self.request_repo.add_approval(pool, record).await?;

        let approvals_at_level: Vec<_> = request.approvals.iter()
            .filter(|a| a.level_number == current_level && matches!(a.action, ApprovalAction::Approved))
            .collect();

        let level_complete = match workflow.approval_type {
            ApprovalType::AnyApprover => !approvals_at_level.is_empty(),
            ApprovalType::AllApprovers => approvals_at_level.len() >= level.approver_ids.len(),
            ApprovalType::Sequential => !approvals_at_level.is_empty(),
        };

        if level_complete {
            let next_level = workflow.levels.iter()
                .find(|l| l.level_number > current_level);

            match next_level {
                Some(next) => {
                    request.current_level = Some(next.level_number);
                    request.due_date = next.due_hours.map(|h| Utc::now() + chrono::Duration::hours(h as i64));
                }
                None => {
                    request.status = ApprovalRequestStatus::Approved;
                    request.approved_at = Some(Utc::now());
                    request.approved_by = Some(approver_id);
                    request.current_level = None;
                }
            }
        }

        self.request_repo.update(pool, request).await
    }

    pub async fn reject(
        &self,
        pool: &SqlitePool,
        request_id: Uuid,
        approver_id: Uuid,
        reason: String,
    ) -> Result<ApprovalRequest> {
        let mut request = self.request_repo.find_by_id(pool, request_id).await?;

        if request.status != ApprovalRequestStatus::Pending {
            return Err(Error::business_rule("Request is not pending approval"));
        }

        let current_level = request.current_level.unwrap_or(1);

        let record = ApprovalRecord {
            id: Uuid::new_v4(),
            request_id: request.id,
            level_number: current_level,
            approver_id,
            action: ApprovalAction::Rejected,
            comments: Some(reason.clone()),
            delegated_to: None,
            created_at: Utc::now(),
        };
        self.request_repo.add_approval(pool, record).await?;

        request.status = ApprovalRequestStatus::Rejected;
        request.rejected_at = Some(Utc::now());
        request.rejected_by = Some(approver_id);
        request.rejection_reason = Some(reason);

        self.request_repo.update(pool, request).await
    }

    pub async fn delegate(
        &self,
        pool: &SqlitePool,
        request_id: Uuid,
        from_approver_id: Uuid,
        to_approver_id: Uuid,
        reason: Option<String>,
    ) -> Result<ApprovalRequest> {
        let mut request = self.request_repo.find_by_id(pool, request_id).await?;

        if request.status != ApprovalRequestStatus::Pending {
            return Err(Error::business_rule("Request is not pending approval"));
        }

        let current_level = request.current_level.unwrap_or(1);

        let record = ApprovalRecord {
            id: Uuid::new_v4(),
            request_id: request.id,
            level_number: current_level,
            approver_id: from_approver_id,
            action: ApprovalAction::Delegated,
            comments: reason,
            delegated_to: Some(to_approver_id),
            created_at: Utc::now(),
        };
        self.request_repo.add_approval(pool, record).await?;

        self.request_repo.update(pool, request).await
    }

    pub async fn get_request(&self, pool: &SqlitePool, id: Uuid) -> Result<ApprovalRequest> {
        self.request_repo.find_by_id(pool, id).await
    }

    pub async fn get_pending_for_approver(&self, pool: &SqlitePool, approver_id: Uuid, pagination: Pagination) -> Result<Paginated<ApprovalRequest>> {
        self.request_repo.find_pending_for_approver(pool, approver_id, pagination).await
    }

    pub async fn list_requests(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<ApprovalRequest>> {
        self.request_repo.find_all(pool, pagination).await
    }

    pub async fn cancel(&self, pool: &SqlitePool, request_id: Uuid, cancelled_by: Uuid) -> Result<ApprovalRequest> {
        let mut request = self.request_repo.find_by_id(pool, request_id).await?;

        if request.status != ApprovalRequestStatus::Pending {
            return Err(Error::business_rule("Only pending requests can be cancelled"));
        }

        if request.requested_by != cancelled_by {
            return Err(Error::unauthorized("Only the requester can cancel"));
        }

        request.status = ApprovalRequestStatus::Cancelled;
        self.request_repo.update(pool, request).await
    }

    pub async fn get_pending_summary(&self, pool: &SqlitePool, user_id: Uuid) -> Result<PendingApprovalSummary> {
        let pending = self.request_repo.find_pending_for_approver(pool, user_id, Pagination::new(1, 1000)).await?;

        let mut by_doc_type: std::collections::HashMap<String, (i64, i64)> = std::collections::HashMap::new();
        let mut total_amount = 0i64;
        let mut overdue_count = 0i64;
        let now = Utc::now();

        for req in &pending.items {
            total_amount += req.amount;
            
            if let Some(due) = req.due_date {
                if due < now {
                    overdue_count += 1;
                }
            }

            let entry = by_doc_type.entry(req.document_type.clone()).or_insert((0, 0));
            entry.0 += 1;
            entry.1 += req.amount;
        }

        Ok(PendingApprovalSummary {
            user_id,
            pending_count: pending.total as i64,
            total_amount,
            overdue_count,
            by_document_type: by_doc_type.into_iter()
                .map(|(doc_type, (count, amount))| DocumentTypeSummary {
                    document_type: doc_type,
                    count,
                    total_amount: amount,
                })
                .collect(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWorkflowRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub document_type: String,
    pub approval_type: ApprovalType,
    pub min_amount: Option<i64>,
    pub max_amount: Option<i64>,
    pub auto_approve_below: Option<i64>,
    pub escalation_hours: Option<i32>,
    pub notify_requester: Option<bool>,
    pub notify_approver: Option<bool>,
    pub allow_delegation: Option<bool>,
    pub allow_reassignment: Option<bool>,
    pub require_comments: Option<bool>,
    pub levels: Vec<CreateLevelRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLevelRequest {
    pub name: String,
    pub description: Option<String>,
    pub approver_type: ApproverType,
    pub approver_ids: Vec<Uuid>,
    pub min_approvers: Option<i32>,
    pub skip_if_approved_above: Option<bool>,
    pub due_hours: Option<i32>,
    pub escalation_to: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWorkflowRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub approval_type: Option<ApprovalType>,
    pub min_amount: Option<i64>,
    pub max_amount: Option<i64>,
    pub auto_approve_below: Option<i64>,
    pub escalation_hours: Option<i32>,
    pub notify_requester: Option<bool>,
    pub notify_approver: Option<bool>,
    pub allow_delegation: Option<bool>,
    pub allow_reassignment: Option<bool>,
    pub require_comments: Option<bool>,
    pub status: Option<ApprovalWorkflowStatus>,
    pub levels: Option<Vec<CreateLevelRequest>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitApprovalRequest {
    pub document_type: String,
    pub document_id: Uuid,
    pub document_number: String,
    pub requested_by: Uuid,
    pub amount: i64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApproveRequest {
    pub approver_id: Uuid,
    pub comments: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RejectRequest {
    pub approver_id: Uuid,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegateRequest {
    pub from_approver_id: Uuid,
    pub to_approver_id: Uuid,
    pub reason: Option<String>,
}

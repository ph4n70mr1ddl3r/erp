use async_trait::async_trait;
use chrono::Utc;
use erp_core::{BaseEntity, Error, Paginated, Pagination, Result};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;

pub struct SqliteApprovalWorkflowRepository;

#[derive(sqlx::FromRow)]
struct RequestRow {
    id: String,
    request_number: String,
    workflow_id: String,
    document_type: String,
    document_id: String,
    document_number: String,
    requested_by: String,
    requested_at: String,
    amount: i64,
    currency: String,
    status: String,
    current_level: Option<i32>,
    due_date: Option<String>,
    approved_at: Option<String>,
    approved_by: Option<String>,
    rejected_at: Option<String>,
    rejected_by: Option<String>,
    rejection_reason: Option<String>,
    created_at: String,
    updated_at: String,
}

#[async_trait]
pub trait ApprovalWorkflowRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ApprovalWorkflow>;
    async fn find_by_code(&self, pool: &SqlitePool, code: &str) -> Result<ApprovalWorkflow>;
    async fn find_by_document_type(&self, pool: &SqlitePool, doc_type: &str) -> Result<Vec<ApprovalWorkflow>>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<ApprovalWorkflow>>;
    async fn create(&self, pool: &SqlitePool, workflow: ApprovalWorkflow) -> Result<ApprovalWorkflow>;
    async fn update(&self, pool: &SqlitePool, workflow: ApprovalWorkflow) -> Result<ApprovalWorkflow>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

#[derive(sqlx::FromRow)]
struct WorkflowRow {
    id: String,
    code: String,
    name: String,
    description: Option<String>,
    document_type: String,
    approval_type: String,
    min_amount: Option<i64>,
    max_amount: Option<i64>,
    auto_approve_below: Option<i64>,
    escalation_hours: Option<i32>,
    notify_requester: i64,
    notify_approver: i64,
    allow_delegation: i64,
    allow_reassignment: i64,
    require_comments: i64,
    status: String,
    created_at: String,
    updated_at: String,
}

#[async_trait]
impl ApprovalWorkflowRepository for SqliteApprovalWorkflowRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ApprovalWorkflow> {
        let row = sqlx::query_as::<_, WorkflowRow>(
            "SELECT id, code, name, description, document_type, approval_type, min_amount, max_amount,
             auto_approve_below, escalation_hours, notify_requester, notify_approver, allow_delegation,
             allow_reassignment, require_comments, status, created_at, updated_at
             FROM approval_workflows WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("ApprovalWorkflow", &id.to_string()))?;

        let levels = self.fetch_levels(pool, id).await?;
        Ok(row_to_workflow(row, levels))
    }

    async fn find_by_code(&self, pool: &SqlitePool, code: &str) -> Result<ApprovalWorkflow> {
        let row = sqlx::query_as::<_, WorkflowRow>(
            "SELECT id, code, name, description, document_type, approval_type, min_amount, max_amount,
             auto_approve_below, escalation_hours, notify_requester, notify_approver, allow_delegation,
             allow_reassignment, require_comments, status, created_at, updated_at
             FROM approval_workflows WHERE code = ?"
        )
        .bind(code)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("ApprovalWorkflow", code))?;

        let levels = self.fetch_levels(pool, Uuid::parse_str(&row.id).unwrap_or_default()).await?;
        Ok(row_to_workflow(row, levels))
    }

    async fn find_by_document_type(&self, pool: &SqlitePool, doc_type: &str) -> Result<Vec<ApprovalWorkflow>> {
        let rows = sqlx::query_as::<_, WorkflowRow>(
            "SELECT id, code, name, description, document_type, approval_type, min_amount, max_amount,
             auto_approve_below, escalation_hours, notify_requester, notify_approver, allow_delegation,
             allow_reassignment, require_comments, status, created_at, updated_at
             FROM approval_workflows WHERE document_type = ? AND status = 'Active'"
        )
        .bind(doc_type)
        .fetch_all(pool)
        .await?;

        let mut workflows = Vec::new();
        for row in rows {
            let id = Uuid::parse_str(&row.id).unwrap_or_default();
            let levels = self.fetch_levels(pool, id).await?;
            workflows.push(row_to_workflow(row, levels));
        }
        Ok(workflows)
    }

    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<ApprovalWorkflow>> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM approval_workflows")
            .fetch_one(pool)
            .await?;

        let rows = sqlx::query_as::<_, WorkflowRow>(
            "SELECT id, code, name, description, document_type, approval_type, min_amount, max_amount,
             auto_approve_below, escalation_hours, notify_requester, notify_approver, allow_delegation,
             allow_reassignment, require_comments, status, created_at, updated_at
             FROM approval_workflows ORDER BY name LIMIT ? OFFSET ?"
        )
        .bind(pagination.limit() as i64)
        .bind(pagination.offset() as i64)
        .fetch_all(pool)
        .await?;

        let mut workflows = Vec::new();
        for row in rows {
            let id = Uuid::parse_str(&row.id).unwrap_or_default();
            let levels = self.fetch_levels(pool, id).await?;
            workflows.push(row_to_workflow(row, levels));
        }

        Ok(Paginated::new(workflows, count.0 as u64, pagination))
    }

    async fn create(&self, pool: &SqlitePool, workflow: ApprovalWorkflow) -> Result<ApprovalWorkflow> {
        let now = Utc::now();
        let mut tx = pool.begin().await?;

        sqlx::query(
            "INSERT INTO approval_workflows (id, code, name, description, document_type, approval_type,
             min_amount, max_amount, auto_approve_below, escalation_hours, notify_requester, notify_approver,
             allow_delegation, allow_reassignment, require_comments, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(workflow.base.id.to_string())
        .bind(&workflow.code)
        .bind(&workflow.name)
        .bind(&workflow.description)
        .bind(&workflow.document_type)
        .bind(format!("{:?}", workflow.approval_type))
        .bind(workflow.min_amount)
        .bind(workflow.max_amount)
        .bind(workflow.auto_approve_below)
        .bind(workflow.escalation_hours)
        .bind(workflow.notify_requester as i64)
        .bind(workflow.notify_approver as i64)
        .bind(workflow.allow_delegation as i64)
        .bind(workflow.allow_reassignment as i64)
        .bind(workflow.require_comments as i64)
        .bind(format!("{:?}", workflow.status))
        .bind(workflow.base.created_at.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&mut *tx)
        .await?;

        for level in &workflow.levels {
            sqlx::query(
                "INSERT INTO approval_workflow_levels (id, workflow_id, level_number, name, description,
                 approver_type, approver_ids, min_approvers, skip_if_approved_above, due_hours, escalation_to)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(level.id.to_string())
            .bind(level.workflow_id.to_string())
            .bind(level.level_number)
            .bind(&level.name)
            .bind(&level.description)
            .bind(format!("{:?}", level.approver_type))
            .bind(serde_json::to_string(&level.approver_ids).unwrap_or_default())
            .bind(level.min_approvers)
            .bind(level.skip_if_approved_above as i64)
            .bind(level.due_hours)
            .bind(level.escalation_to.map(|id| id.to_string()))
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(workflow)
    }

    async fn update(&self, pool: &SqlitePool, workflow: ApprovalWorkflow) -> Result<ApprovalWorkflow> {
        let now = Utc::now();
        let mut tx = pool.begin().await?;

        let rows = sqlx::query(
            "UPDATE approval_workflows SET code=?, name=?, description=?, document_type=?, approval_type=?,
             min_amount=?, max_amount=?, auto_approve_below=?, escalation_hours=?, notify_requester=?,
             notify_approver=?, allow_delegation=?, allow_reassignment=?, require_comments=?, status=?, updated_at=?
             WHERE id=?"
        )
        .bind(&workflow.code)
        .bind(&workflow.name)
        .bind(&workflow.description)
        .bind(&workflow.document_type)
        .bind(format!("{:?}", workflow.approval_type))
        .bind(workflow.min_amount)
        .bind(workflow.max_amount)
        .bind(workflow.auto_approve_below)
        .bind(workflow.escalation_hours)
        .bind(workflow.notify_requester as i64)
        .bind(workflow.notify_approver as i64)
        .bind(workflow.allow_delegation as i64)
        .bind(workflow.allow_reassignment as i64)
        .bind(workflow.require_comments as i64)
        .bind(format!("{:?}", workflow.status))
        .bind(now.to_rfc3339())
        .bind(workflow.base.id.to_string())
        .execute(&mut *tx)
        .await?;

        if rows.rows_affected() == 0 {
            return Err(Error::not_found("ApprovalWorkflow", &workflow.base.id.to_string()));
        }

        sqlx::query("DELETE FROM approval_workflow_levels WHERE workflow_id = ?")
            .bind(workflow.base.id.to_string())
            .execute(&mut *tx)
            .await?;

        for level in &workflow.levels {
            sqlx::query(
                "INSERT INTO approval_workflow_levels (id, workflow_id, level_number, name, description,
                 approver_type, approver_ids, min_approvers, skip_if_approved_above, due_hours, escalation_to)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(level.id.to_string())
            .bind(level.workflow_id.to_string())
            .bind(level.level_number)
            .bind(&level.name)
            .bind(&level.description)
            .bind(format!("{:?}", level.approver_type))
            .bind(serde_json::to_string(&level.approver_ids).unwrap_or_default())
            .bind(level.min_approvers)
            .bind(level.skip_if_approved_above as i64)
            .bind(level.due_hours)
            .bind(level.escalation_to.map(|id| id.to_string()))
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(workflow)
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let mut tx = pool.begin().await?;
        sqlx::query("DELETE FROM approval_workflow_levels WHERE workflow_id = ?")
            .bind(id.to_string())
            .execute(&mut *tx)
            .await?;

        let rows = sqlx::query("DELETE FROM approval_workflows WHERE id = ?")
            .bind(id.to_string())
            .execute(&mut *tx)
            .await?;

        if rows.rows_affected() == 0 {
            return Err(Error::not_found("ApprovalWorkflow", &id.to_string()));
        }

        tx.commit().await?;
        Ok(())
    }
}

impl SqliteApprovalWorkflowRepository {
    async fn fetch_levels(&self, pool: &SqlitePool, workflow_id: Uuid) -> Result<Vec<ApprovalLevel>> {
        #[derive(sqlx::FromRow)]
        struct LevelRow {
            id: String,
            workflow_id: String,
            level_number: i32,
            name: String,
            description: Option<String>,
            approver_type: String,
            approver_ids: String,
            min_approvers: i32,
            skip_if_approved_above: i64,
            due_hours: Option<i32>,
            escalation_to: Option<String>,
        }

        let rows = sqlx::query_as::<_, LevelRow>(
            "SELECT id, workflow_id, level_number, name, description, approver_type, approver_ids,
             min_approvers, skip_if_approved_above, due_hours, escalation_to
             FROM approval_workflow_levels WHERE workflow_id = ? ORDER BY level_number"
        )
        .bind(workflow_id.to_string())
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| ApprovalLevel {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            workflow_id: Uuid::parse_str(&r.workflow_id).unwrap_or_default(),
            level_number: r.level_number,
            name: r.name,
            description: r.description,
            approver_type: match r.approver_type.as_str() {
                "Role" => ApproverType::Role,
                "Department" => ApproverType::Department,
                "Supervisor" => ApproverType::Supervisor,
                "AmountBased" => ApproverType::AmountBased,
                _ => ApproverType::SpecificUser,
            },
            approver_ids: serde_json::from_str(&r.approver_ids).unwrap_or_default(),
            min_approvers: r.min_approvers,
            skip_if_approved_above: r.skip_if_approved_above != 0,
            due_hours: r.due_hours,
            escalation_to: r.escalation_to.and_then(|id| Uuid::parse_str(&id).ok()),
        }).collect())
    }
}

fn row_to_workflow(row: WorkflowRow, levels: Vec<ApprovalLevel>) -> ApprovalWorkflow {
    ApprovalWorkflow {
        base: BaseEntity {
            id: Uuid::parse_str(&row.id).unwrap_or_default(),
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            created_by: None,
            updated_by: None,
        },
        code: row.code,
        name: row.name,
        description: row.description,
        document_type: row.document_type,
        approval_type: match row.approval_type.as_str() {
            "AllApprovers" => ApprovalType::AllApprovers,
            "Sequential" => ApprovalType::Sequential,
            _ => ApprovalType::AnyApprover,
        },
        min_amount: row.min_amount,
        max_amount: row.max_amount,
        auto_approve_below: row.auto_approve_below,
        escalation_hours: row.escalation_hours,
        notify_requester: row.notify_requester != 0,
        notify_approver: row.notify_approver != 0,
        allow_delegation: row.allow_delegation != 0,
        allow_reassignment: row.allow_reassignment != 0,
        require_comments: row.require_comments != 0,
        status: match row.status.as_str() {
            "Inactive" => ApprovalWorkflowStatus::Inactive,
            _ => ApprovalWorkflowStatus::Active,
        },
        levels,
    }
}

pub struct SqliteApprovalRequestRepository;

#[async_trait]
pub trait ApprovalRequestRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ApprovalRequest>;
    async fn find_by_document(&self, pool: &SqlitePool, doc_type: &str, doc_id: Uuid) -> Result<Option<ApprovalRequest>>;
    async fn find_pending_for_approver(&self, pool: &SqlitePool, approver_id: Uuid, pagination: Pagination) -> Result<Paginated<ApprovalRequest>>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<ApprovalRequest>>;
    async fn create(&self, pool: &SqlitePool, request: ApprovalRequest) -> Result<ApprovalRequest>;
    async fn update(&self, pool: &SqlitePool, request: ApprovalRequest) -> Result<ApprovalRequest>;
    async fn add_approval(&self, pool: &SqlitePool, record: ApprovalRecord) -> Result<ApprovalRecord>;
}

#[async_trait]
impl ApprovalRequestRepository for SqliteApprovalRequestRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ApprovalRequest> {
        let row = sqlx::query_as::<_, RequestRow>(
            "SELECT id, request_number, workflow_id, document_type, document_id, document_number,
             requested_by, requested_at, amount, currency, status, current_level, due_date,
             approved_at, approved_by, rejected_at, rejected_by, rejection_reason, created_at, updated_at
             FROM approval_requests WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("ApprovalRequest", &id.to_string()))?;

        let approvals = self.fetch_approvals(pool, id).await?;
        Ok(row_to_request(row, approvals))
    }

    async fn find_by_document(&self, pool: &SqlitePool, doc_type: &str, doc_id: Uuid) -> Result<Option<ApprovalRequest>> {
        let row = sqlx::query_as::<_, RequestRow>(
            "SELECT id, request_number, workflow_id, document_type, document_id, document_number,
             requested_by, requested_at, amount, currency, status, current_level, due_date,
             approved_at, approved_by, rejected_at, rejected_by, rejection_reason, created_at, updated_at
             FROM approval_requests WHERE document_type = ? AND document_id = ? ORDER BY created_at DESC LIMIT 1"
        )
        .bind(doc_type)
        .bind(doc_id.to_string())
        .fetch_optional(pool)
        .await?;

        match row {
            Some(r) => {
                let id = Uuid::parse_str(&r.id).unwrap_or_default();
                let approvals = self.fetch_approvals(pool, id).await?;
                Ok(Some(row_to_request(r, approvals)))
            }
            None => Ok(None),
        }
    }

    async fn find_pending_for_approver(&self, pool: &SqlitePool, approver_id: Uuid, pagination: Pagination) -> Result<Paginated<ApprovalRequest>> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM approval_requests ar
             JOIN approval_workflow_levels awl ON awl.workflow_id = ar.workflow_id
             WHERE ar.status = 'Pending' AND awl.level_number = ar.current_level
             AND (awl.approver_ids LIKE ? OR awl.approver_type = 'Supervisor')"
        )
        .bind(format!("%{}%", approver_id))
        .fetch_one(pool)
        .await?;

        let rows = sqlx::query_as::<_, RequestRow>(
            "SELECT ar.id, ar.request_number, ar.workflow_id, ar.document_type, ar.document_id, ar.document_number,
             ar.requested_by, ar.requested_at, ar.amount, ar.currency, ar.status, ar.current_level, ar.due_date,
             ar.approved_at, ar.approved_by, ar.rejected_at, ar.rejected_by, ar.rejection_reason, ar.created_at, ar.updated_at
             FROM approval_requests ar
             JOIN approval_workflow_levels awl ON awl.workflow_id = ar.workflow_id
             WHERE ar.status = 'Pending' AND awl.level_number = ar.current_level
             AND (awl.approver_ids LIKE ? OR awl.approver_type = 'Supervisor')
             ORDER BY ar.created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(format!("%{}%", approver_id))
        .bind(pagination.limit() as i64)
        .bind(pagination.offset() as i64)
        .fetch_all(pool)
        .await?;

        let mut requests = Vec::new();
        for row in rows {
            let id = Uuid::parse_str(&row.id).unwrap_or_default();
            let approvals = self.fetch_approvals(pool, id).await?;
            requests.push(row_to_request(row, approvals));
        }

        Ok(Paginated::new(requests, count.0 as u64, pagination))
    }

    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<ApprovalRequest>> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM approval_requests")
            .fetch_one(pool)
            .await?;

        let rows = sqlx::query_as::<_, RequestRow>(
            "SELECT id, request_number, workflow_id, document_type, document_id, document_number,
             requested_by, requested_at, amount, currency, status, current_level, due_date,
             approved_at, approved_by, rejected_at, rejected_by, rejection_reason, created_at, updated_at
             FROM approval_requests ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(pagination.limit() as i64)
        .bind(pagination.offset() as i64)
        .fetch_all(pool)
        .await?;

        let mut requests = Vec::new();
        for row in rows {
            let id = Uuid::parse_str(&row.id).unwrap_or_default();
            let approvals = self.fetch_approvals(pool, id).await?;
            requests.push(row_to_request(row, approvals));
        }

        Ok(Paginated::new(requests, count.0 as u64, pagination))
    }

    async fn create(&self, pool: &SqlitePool, request: ApprovalRequest) -> Result<ApprovalRequest> {
        let now = Utc::now();
        sqlx::query(
            "INSERT INTO approval_requests (id, request_number, workflow_id, document_type, document_id,
             document_number, requested_by, requested_at, amount, currency, status, current_level, due_date,
             approved_at, approved_by, rejected_at, rejected_by, rejection_reason, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NULL, NULL, NULL, NULL, NULL, ?, ?)"
        )
        .bind(request.id.to_string())
        .bind(&request.request_number)
        .bind(request.workflow_id.to_string())
        .bind(&request.document_type)
        .bind(request.document_id.to_string())
        .bind(&request.document_number)
        .bind(request.requested_by.to_string())
        .bind(request.requested_at.to_rfc3339())
        .bind(request.amount)
        .bind(&request.currency)
        .bind(format!("{:?}", request.status))
        .bind(request.current_level)
        .bind(request.due_date.map(|d| d.to_rfc3339()))
        .bind(request.created_at.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(request)
    }

    async fn update(&self, pool: &SqlitePool, request: ApprovalRequest) -> Result<ApprovalRequest> {
        let now = Utc::now();
        let rows = sqlx::query(
            "UPDATE approval_requests SET status=?, current_level=?, due_date=?, approved_at=?, approved_by=?,
             rejected_at=?, rejected_by=?, rejection_reason=?, updated_at=? WHERE id=?"
        )
        .bind(format!("{:?}", request.status))
        .bind(request.current_level)
        .bind(request.due_date.map(|d| d.to_rfc3339()))
        .bind(request.approved_at.map(|d| d.to_rfc3339()))
        .bind(request.approved_by.map(|id| id.to_string()))
        .bind(request.rejected_at.map(|d| d.to_rfc3339()))
        .bind(request.rejected_by.map(|id| id.to_string()))
        .bind(&request.rejection_reason)
        .bind(now.to_rfc3339())
        .bind(request.id.to_string())
        .execute(pool)
        .await?;

        if rows.rows_affected() == 0 {
            return Err(Error::not_found("ApprovalRequest", &request.id.to_string()));
        }

        Ok(request)
    }

    async fn add_approval(&self, pool: &SqlitePool, record: ApprovalRecord) -> Result<ApprovalRecord> {
        sqlx::query(
            "INSERT INTO approval_records (id, request_id, level_number, approver_id, action, comments, delegated_to, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(record.id.to_string())
        .bind(record.request_id.to_string())
        .bind(record.level_number)
        .bind(record.approver_id.to_string())
        .bind(format!("{:?}", record.action))
        .bind(&record.comments)
        .bind(record.delegated_to.map(|id| id.to_string()))
        .bind(record.created_at.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(record)
    }
}

impl SqliteApprovalRequestRepository {
    async fn fetch_approvals(&self, pool: &SqlitePool, request_id: Uuid) -> Result<Vec<ApprovalRecord>> {
        #[derive(sqlx::FromRow)]
        struct ApprovalRow {
            id: String,
            request_id: String,
            level_number: i32,
            approver_id: String,
            action: String,
            comments: Option<String>,
            delegated_to: Option<String>,
            created_at: String,
        }

        let rows = sqlx::query_as::<_, ApprovalRow>(
            "SELECT id, request_id, level_number, approver_id, action, comments, delegated_to, created_at
             FROM approval_records WHERE request_id = ? ORDER BY created_at"
        )
        .bind(request_id.to_string())
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| ApprovalRecord {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            request_id: Uuid::parse_str(&r.request_id).unwrap_or_default(),
            level_number: r.level_number,
            approver_id: Uuid::parse_str(&r.approver_id).unwrap_or_default(),
            action: match r.action.as_str() {
                "Rejected" => ApprovalAction::Rejected,
                "Delegated" => ApprovalAction::Delegated,
                "ReturnedForInfo" => ApprovalAction::ReturnedForInfo,
                _ => ApprovalAction::Approved,
            },
            comments: r.comments,
            delegated_to: r.delegated_to.and_then(|id| Uuid::parse_str(&id).ok()),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        }).collect())
    }
}

fn row_to_request(row: RequestRow, approvals: Vec<ApprovalRecord>) -> ApprovalRequest {
    ApprovalRequest {
        id: Uuid::parse_str(&row.id).unwrap_or_default(),
        request_number: row.request_number,
        workflow_id: Uuid::parse_str(&row.workflow_id).unwrap_or_default(),
        document_type: row.document_type,
        document_id: Uuid::parse_str(&row.document_id).unwrap_or_default(),
        document_number: row.document_number,
        requested_by: Uuid::parse_str(&row.requested_by).unwrap_or_default(),
        requested_at: chrono::DateTime::parse_from_rfc3339(&row.requested_at)
            .map(|d| d.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now()),
        amount: row.amount,
        currency: row.currency,
        status: match row.status.as_str() {
            "Approved" => ApprovalRequestStatus::Approved,
            "Rejected" => ApprovalRequestStatus::Rejected,
            "Cancelled" => ApprovalRequestStatus::Cancelled,
            "Escalated" => ApprovalRequestStatus::Escalated,
            _ => ApprovalRequestStatus::Pending,
        },
        current_level: row.current_level,
        due_date: row.due_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
            .map(|d| d.with_timezone(&Utc)),
        approved_at: row.approved_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
            .map(|d| d.with_timezone(&Utc)),
        approved_by: row.approved_by.and_then(|id| Uuid::parse_str(&id).ok()),
        rejected_at: row.rejected_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
            .map(|d| d.with_timezone(&Utc)),
        rejected_by: row.rejected_by.and_then(|id| Uuid::parse_str(&id).ok()),
        rejection_reason: row.rejection_reason,
        created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
            .map(|d| d.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now()),
        updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at)
            .map(|d| d.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now()),
        approvals,
    }
}

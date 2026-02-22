use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::workflow_models::*;
use crate::{Error, Result, Pagination, Paginated};

pub struct WorkflowService;

impl WorkflowService {
    pub async fn get_workflow(pool: &SqlitePool, entity_type: &str) -> Result<Option<Workflow>> {
        let row = sqlx::query_as::<_, WorkflowRow>(
            "SELECT id, name, entity_type, approval_levels, auto_approve_below, require_comment, status, created_at, updated_at
             FROM workflows WHERE entity_type = ? AND status = 'Active'"
        )
        .bind(entity_type)
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
        Ok(row.map(|r| r.into()))
    }

    pub async fn list_workflows(pool: &SqlitePool) -> Result<Vec<Workflow>> {
        let rows = sqlx::query_as::<_, WorkflowRow>(
            "SELECT id, name, entity_type, approval_levels, auto_approve_below, require_comment, status, created_at, updated_at
             FROM workflows ORDER BY name"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn create_workflow(pool: &SqlitePool, workflow: Workflow) -> Result<Workflow> {
        let now = Utc::now();
        sqlx::query(
            "INSERT INTO workflows (id, name, entity_type, approval_levels, auto_approve_below, require_comment, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(workflow.id.to_string())
        .bind(&workflow.name)
        .bind(&workflow.entity_type)
        .bind(workflow.approval_levels as i64)
        .bind(workflow.auto_approve_below)
        .bind(workflow.require_comment as i64)
        .bind(format!("{:?}", workflow.status))
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
        Ok(workflow)
    }
}

pub struct ApprovalService;

impl ApprovalService {
    pub async fn create_request(
        pool: &SqlitePool,
        workflow_id: Uuid,
        entity_type: &str,
        entity_id: &str,
        amount: i64,
        requested_by: Option<Uuid>,
    ) -> Result<ApprovalRequest> {
        let workflow = sqlx::query_as::<_, WorkflowRow>(
            "SELECT id, name, entity_type, approval_levels, auto_approve_below, require_comment, status, created_at, updated_at
             FROM workflows WHERE id = ?"
        )
        .bind(workflow_id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?
        .ok_or_else(|| Error::not_found("Workflow", &workflow_id.to_string()))?;
        
        let workflow: Workflow = workflow.into();
        let now = Utc::now();
        
        let (status, current_level) = if amount > 0 && amount < workflow.auto_approve_below {
            (ApprovalStatus::Approved, workflow.approval_levels)
        } else {
            (ApprovalStatus::Pending, 1)
        };
        
        let request = ApprovalRequest {
            id: Uuid::new_v4(),
            workflow_id,
            entity_type: entity_type.to_string(),
            entity_id: entity_id.to_string(),
            current_level,
            max_level: workflow.approval_levels,
            status: status.clone(),
            requested_by,
            requested_at: now,
            completed_at: if status == ApprovalStatus::Approved { Some(now) } else { None },
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO approval_requests (id, workflow_id, entity_type, entity_id, current_level, max_level, status, requested_by, requested_at, completed_at, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(request.id.to_string())
        .bind(request.workflow_id.to_string())
        .bind(&request.entity_type)
        .bind(&request.entity_id)
        .bind(request.current_level as i64)
        .bind(request.max_level as i64)
        .bind(format!("{:?}", request.status))
        .bind(request.requested_by.map(|id| id.to_string()))
        .bind(request.requested_at.to_rfc3339())
        .bind(request.completed_at.map(|d| d.to_rfc3339()))
        .bind(request.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
        Ok(request)
    }

    pub async fn approve(
        pool: &SqlitePool,
        request_id: Uuid,
        approver_id: Uuid,
        approver_name: &str,
        comment: Option<&str>,
    ) -> Result<ApprovalRequest> {
        let now = Utc::now();
        
        let request = Self::get_request(pool, request_id).await?;
        
        if request.status != ApprovalStatus::Pending {
            return Err(Error::business_rule("Approval request is not pending"));
        }
        
        sqlx::query(
            "INSERT INTO approvals (id, approval_request_id, level, approver_id, approver_name, status, comment, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(Uuid::new_v4().to_string())
        .bind(request_id.to_string())
        .bind(request.current_level as i64)
        .bind(approver_id.to_string())
        .bind(approver_name)
        .bind("Approved")
        .bind(comment)
        .bind(now.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
        let new_level = request.current_level + 1;
        let (status, completed_at) = if new_level > request.max_level {
            (ApprovalStatus::Approved, Some(now))
        } else {
            (ApprovalStatus::Pending, None)
        };
        
        sqlx::query(
            "UPDATE approval_requests SET current_level = ?, status = ?, completed_at = ? WHERE id = ?"
        )
        .bind(new_level as i64)
        .bind(format!("{:?}", status))
        .bind(completed_at.map(|d| d.to_rfc3339()))
        .bind(request_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
        Self::get_request(pool, request_id).await
    }

    pub async fn reject(
        pool: &SqlitePool,
        request_id: Uuid,
        approver_id: Uuid,
        approver_name: &str,
        comment: &str,
    ) -> Result<ApprovalRequest> {
        let now = Utc::now();
        
        let request = Self::get_request(pool, request_id).await?;
        
        if request.status != ApprovalStatus::Pending {
            return Err(Error::business_rule("Approval request is not pending"));
        }
        
        sqlx::query(
            "INSERT INTO approvals (id, approval_request_id, level, approver_id, approver_name, status, comment, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(Uuid::new_v4().to_string())
        .bind(request_id.to_string())
        .bind(request.current_level as i64)
        .bind(approver_id.to_string())
        .bind(approver_name)
        .bind("Rejected")
        .bind(comment)
        .bind(now.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
        sqlx::query(
            "UPDATE approval_requests SET status = ?, completed_at = ? WHERE id = ?"
        )
        .bind("Rejected")
        .bind(now.to_rfc3339())
        .bind(request_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
        Self::get_request(pool, request_id).await
    }

    pub async fn get_request(pool: &SqlitePool, id: Uuid) -> Result<ApprovalRequest> {
        let row = sqlx::query_as::<_, ApprovalRequestRow>(
            "SELECT id, workflow_id, entity_type, entity_id, current_level, max_level, status, requested_by, requested_at, completed_at, created_at
             FROM approval_requests WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?
        .ok_or_else(|| Error::not_found("ApprovalRequest", &id.to_string()))?;
        
        Ok(row.into())
    }

    pub async fn list_pending(pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<ApprovalRequest>> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM approval_requests WHERE status = 'Pending'")
            .fetch_one(pool)
            .await
            .map_err(|e| Error::Database(e.into()))?;
        
        let rows = sqlx::query_as::<_, ApprovalRequestRow>(
            "SELECT id, workflow_id, entity_type, entity_id, current_level, max_level, status, requested_by, requested_at, completed_at, created_at
             FROM approval_requests WHERE status = 'Pending' ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(pagination.limit() as i64)
        .bind(pagination.offset() as i64)
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
        Ok(Paginated::new(
            rows.into_iter().map(|r| r.into()).collect(),
            count.0 as u64,
            pagination,
        ))
    }

    pub async fn get_for_entity(pool: &SqlitePool, entity_type: &str, entity_id: &str) -> Result<Option<ApprovalRequest>> {
        let row = sqlx::query_as::<_, ApprovalRequestRow>(
            "SELECT id, workflow_id, entity_type, entity_id, current_level, max_level, status, requested_by, requested_at, completed_at, created_at
             FROM approval_requests WHERE entity_type = ? AND entity_id = ? ORDER BY created_at DESC LIMIT 1"
        )
        .bind(entity_type)
        .bind(entity_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
        Ok(row.map(|r| r.into()))
    }

    pub async fn get_approvals(pool: &SqlitePool, request_id: Uuid) -> Result<Vec<Approval>> {
        let rows = sqlx::query_as::<_, ApprovalRow>(
            "SELECT id, approval_request_id, level, approver_id, approver_name, status, comment, created_at
             FROM approvals WHERE approval_request_id = ? ORDER BY level"
        )
        .bind(request_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

pub struct NotificationService;

impl NotificationService {
    pub async fn create(
        pool: &SqlitePool,
        user_id: Uuid,
        title: &str,
        message: &str,
        entity_type: Option<&str>,
        entity_id: Option<&str>,
        notification_type: NotificationType,
    ) -> Result<Notification> {
        let now = Utc::now();
        let notification = Notification {
            id: Uuid::new_v4(),
            user_id,
            title: title.to_string(),
            message: message.to_string(),
            entity_type: entity_type.map(|s| s.to_string()),
            entity_id: entity_id.map(|s| s.to_string()),
            notification_type,
            read: false,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO notifications (id, user_id, title, message, entity_type, entity_id, notification_type, read, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(notification.id.to_string())
        .bind(notification.user_id.to_string())
        .bind(&notification.title)
        .bind(&notification.message)
        .bind(&notification.entity_type)
        .bind(&notification.entity_id)
        .bind(format!("{:?}", notification.notification_type))
        .bind(0i64)
        .bind(notification.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
        Ok(notification)
    }

    pub async fn list_for_user(pool: &SqlitePool, user_id: Uuid, unread_only: bool) -> Result<Vec<Notification>> {
        let query = if unread_only {
            "SELECT id, user_id, title, message, entity_type, entity_id, notification_type, read, created_at
             FROM notifications WHERE user_id = ? AND read = 0 ORDER BY created_at DESC LIMIT 50"
        } else {
            "SELECT id, user_id, title, message, entity_type, entity_id, notification_type, read, created_at
             FROM notifications WHERE user_id = ? ORDER BY created_at DESC LIMIT 50"
        };
        
        let rows = sqlx::query_as::<_, NotificationRow>(query)
            .bind(user_id.to_string())
            .fetch_all(pool)
            .await
            .map_err(|e| Error::Database(e.into()))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn mark_read(pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("UPDATE notifications SET read = 1 WHERE id = ?")
            .bind(id.to_string())
            .execute(pool)
            .await
            .map_err(|e| Error::Database(e.into()))?;
        Ok(())
    }

    pub async fn mark_all_read(pool: &SqlitePool, user_id: Uuid) -> Result<()> {
        sqlx::query("UPDATE notifications SET read = 1 WHERE user_id = ?")
            .bind(user_id.to_string())
            .execute(pool)
            .await
            .map_err(|e| Error::Database(e.into()))?;
        Ok(())
    }

    pub async fn unread_count(pool: &SqlitePool, user_id: Uuid) -> Result<u64> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM notifications WHERE user_id = ? AND read = 0")
            .bind(user_id.to_string())
            .fetch_one(pool)
            .await
            .map_err(|e| Error::Database(e.into()))?;
        Ok(count.0 as u64)
    }
}

#[derive(sqlx::FromRow)]
struct WorkflowRow {
    id: String,
    name: String,
    entity_type: String,
    approval_levels: i64,
    auto_approve_below: i64,
    require_comment: i64,
    status: String,
    created_at: String,
    updated_at: String,
}

impl From<WorkflowRow> for Workflow {
    fn from(r: WorkflowRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            name: r.name,
            entity_type: r.entity_type,
            approval_levels: r.approval_levels as u32,
            auto_approve_below: r.auto_approve_below,
            require_comment: r.require_comment != 0,
            status: match r.status.as_str() {
                "Inactive" => WorkflowStatus::Inactive,
                _ => WorkflowStatus::Active,
            },
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct ApprovalRequestRow {
    id: String,
    workflow_id: String,
    entity_type: String,
    entity_id: String,
    current_level: i64,
    max_level: i64,
    status: String,
    requested_by: Option<String>,
    requested_at: String,
    completed_at: Option<String>,
    created_at: String,
}

impl From<ApprovalRequestRow> for ApprovalRequest {
    fn from(r: ApprovalRequestRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            workflow_id: Uuid::parse_str(&r.workflow_id).unwrap_or_default(),
            entity_type: r.entity_type,
            entity_id: r.entity_id,
            current_level: r.current_level as u32,
            max_level: r.max_level as u32,
            status: match r.status.as_str() {
                "Approved" => ApprovalStatus::Approved,
                "Rejected" => ApprovalStatus::Rejected,
                "Cancelled" => ApprovalStatus::Cancelled,
                _ => ApprovalStatus::Pending,
            },
            requested_by: r.requested_by.and_then(|id| Uuid::parse_str(&id).ok()),
            requested_at: chrono::DateTime::parse_from_rfc3339(&r.requested_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            completed_at: r.completed_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct ApprovalRow {
    id: String,
    approval_request_id: String,
    level: i64,
    approver_id: Option<String>,
    approver_name: Option<String>,
    status: String,
    comment: Option<String>,
    created_at: String,
}

impl From<ApprovalRow> for Approval {
    fn from(r: ApprovalRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            approval_request_id: Uuid::parse_str(&r.approval_request_id).unwrap_or_default(),
            level: r.level as u32,
            approver_id: r.approver_id.and_then(|id| Uuid::parse_str(&id).ok()),
            approver_name: r.approver_name,
            status: match r.status.as_str() {
                "Rejected" => ApprovalStatus::Rejected,
                _ => ApprovalStatus::Approved,
            },
            comment: r.comment,
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct NotificationRow {
    id: String,
    user_id: String,
    title: String,
    message: String,
    entity_type: Option<String>,
    entity_id: Option<String>,
    notification_type: String,
    read: i64,
    created_at: String,
}

impl From<NotificationRow> for Notification {
    fn from(r: NotificationRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            user_id: Uuid::parse_str(&r.user_id).unwrap_or_default(),
            title: r.title,
            message: r.message,
            entity_type: r.entity_type,
            entity_id: r.entity_id,
            notification_type: match r.notification_type.as_str() {
                "ApprovalRequired" => NotificationType::ApprovalRequired,
                "ApprovalApproved" => NotificationType::ApprovalApproved,
                "ApprovalRejected" => NotificationType::ApprovalRejected,
                "Warning" => NotificationType::Warning,
                _ => NotificationType::Info,
            },
            read: r.read != 0,
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        }
    }
}

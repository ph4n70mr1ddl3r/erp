use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ApprovalWorkflowStatus {
    Active,
    Inactive,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ApprovalType {
    AnyApprover,
    AllApprovers,
    Sequential,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ApprovalRequestStatus {
    Pending,
    Approved,
    Rejected,
    Cancelled,
    Escalated,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ApprovalAction {
    Approved,
    Rejected,
    Delegated,
    ReturnedForInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalWorkflow {
    pub base: BaseEntity,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub document_type: String,
    pub approval_type: ApprovalType,
    pub min_amount: Option<i64>,
    pub max_amount: Option<i64>,
    pub auto_approve_below: Option<i64>,
    pub escalation_hours: Option<i32>,
    pub notify_requester: bool,
    pub notify_approver: bool,
    pub allow_delegation: bool,
    pub allow_reassignment: bool,
    pub require_comments: bool,
    pub status: ApprovalWorkflowStatus,
    pub levels: Vec<ApprovalLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalLevel {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub level_number: i32,
    pub name: String,
    pub description: Option<String>,
    pub approver_type: ApproverType,
    pub approver_ids: Vec<Uuid>,
    pub min_approvers: i32,
    pub skip_if_approved_above: bool,
    pub due_hours: Option<i32>,
    pub escalation_to: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ApproverType {
    SpecificUser,
    Role,
    Department,
    Supervisor,
    AmountBased,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub id: Uuid,
    pub request_number: String,
    pub workflow_id: Uuid,
    pub document_type: String,
    pub document_id: Uuid,
    pub document_number: String,
    pub requested_by: Uuid,
    pub requested_at: DateTime<Utc>,
    pub amount: i64,
    pub currency: String,
    pub status: ApprovalRequestStatus,
    pub current_level: Option<i32>,
    pub due_date: Option<DateTime<Utc>>,
    pub approved_at: Option<DateTime<Utc>>,
    pub approved_by: Option<Uuid>,
    pub rejected_at: Option<DateTime<Utc>>,
    pub rejected_by: Option<Uuid>,
    pub rejection_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub approvals: Vec<ApprovalRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRecord {
    pub id: Uuid,
    pub request_id: Uuid,
    pub level_number: i32,
    pub approver_id: Uuid,
    pub action: ApprovalAction,
    pub comments: Option<String>,
    pub delegated_to: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalDelegation {
    pub id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub document_types: Vec<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingApprovalSummary {
    pub user_id: Uuid,
    pub pending_count: i64,
    pub total_amount: i64,
    pub overdue_count: i64,
    pub by_document_type: Vec<DocumentTypeSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentTypeSummary {
    pub document_type: String,
    pub count: i64,
    pub total_amount: i64,
}

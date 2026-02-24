use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DocumentStatus {
    Draft,
    CheckedOut,
    PendingReview,
    Approved,
    Published,
    Archived,
    Deprecated,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DocumentType {
    Policy,
    Procedure,
    WorkInstruction,
    Form,
    Template,
    Contract,
    Invoice,
    Report,
    Specification,
    Drawing,
    Certificate,
    Manual,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AccessLevel {
    Public,
    Internal,
    Confidential,
    Restricted,
    TopSecret,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentFolder {
    pub base: BaseEntity,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub path: String,
    pub description: Option<String>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub base: BaseEntity,
    pub document_number: String,
    pub title: String,
    pub description: Option<String>,
    pub document_type: DocumentType,
    pub folder_id: Option<Uuid>,
    pub status: DocumentStatus,
    pub version: i32,
    pub revision: String,
    pub access_level: AccessLevel,
    pub file_name: String,
    pub file_path: String,
    pub file_size: i64,
    pub mime_type: String,
    pub checksum: String,
    pub author_id: Option<Uuid>,
    pub owner_id: Option<Uuid>,
    pub checked_out_by: Option<Uuid>,
    pub checked_out_at: Option<DateTime<Utc>>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub published_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub tags: Option<String>,
    pub metadata: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentVersion {
    pub base: BaseEntity,
    pub document_id: Uuid,
    pub version: i32,
    pub revision: String,
    pub file_path: String,
    pub file_size: i64,
    pub checksum: String,
    pub change_summary: Option<String>,
    pub changed_by: Option<Uuid>,
    pub status: DocumentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentCheckout {
    pub base: BaseEntity,
    pub document_id: Uuid,
    pub user_id: Uuid,
    pub checkout_at: DateTime<Utc>,
    pub expected_return: Option<DateTime<Utc>>,
    pub checkin_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentReview {
    pub base: BaseEntity,
    pub document_id: Uuid,
    pub version: i32,
    pub reviewer_id: Uuid,
    pub requested_at: DateTime<Utc>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub status: ReviewStatus,
    pub comments: Option<String>,
    pub approved: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReviewStatus {
    Pending,
    InReview,
    Approved,
    Rejected,
    ChangesRequested,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentPermission {
    pub base: BaseEntity,
    pub document_id: Option<Uuid>,
    pub folder_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub role_id: Option<Uuid>,
    pub can_read: bool,
    pub can_write: bool,
    pub can_delete: bool,
    pub can_share: bool,
    pub can_approve: bool,
    pub can_checkout: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentWorkflow {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub document_type: Option<String>,
    pub steps: String,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentWorkflowInstance {
    pub base: BaseEntity,
    pub workflow_id: Uuid,
    pub document_id: Uuid,
    pub current_step: i32,
    pub status: Status,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRelation {
    pub base: BaseEntity,
    pub source_document_id: Uuid,
    pub target_document_id: Uuid,
    pub relation_type: DocumentRelationType,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DocumentRelationType {
    References,
    Supersedes,
    Amends,
    Appendices,
    Parent,
    Child,
    Related,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub document_types: Option<String>,
    pub retention_years: i32,
    pub review_after_years: Option<i32>,
    pub disposition: DispositionType,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DispositionType {
    Destroy,
    Archive,
    Transfer,
    Review,
}

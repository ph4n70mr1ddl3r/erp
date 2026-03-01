use chrono::{DateTime, Utc};
use erp_core::Status;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ContractCategory {
    Purchase,
    Sales,
    Service,
    Employment,
    Lease,
    License,
    Nda,
    Partnership,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ContractStatus {
    Draft,
    InReview,
    PendingSignature,
    Active,
    Expired,
    Terminated,
    Renewed,
    Superseded,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ContractRisk {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractRecord {
    pub id: Uuid,
    pub contract_number: String,
    pub title: String,
    pub description: Option<String>,
    pub category: ContractCategory,
    pub vendor_id: Option<Uuid>,
    pub customer_id: Option<Uuid>,
    pub contract_type: String,
    pub status: ContractStatus,
    pub risk_level: ContractRisk,
    pub value: i64,
    pub currency: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub effective_date: Option<DateTime<Utc>>,
    pub auto_renew: bool,
    pub renewal_term_months: i32,
    pub notice_period_days: i32,
    pub owner_id: Uuid,
    pub department_id: Option<Uuid>,
    pub parent_contract_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractClause {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub clause_type: String,
    pub title: String,
    pub content: String,
    pub section_number: String,
    pub is_standard: bool,
    pub is_negotiable: bool,
    pub deviation_notes: Option<String>,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractClauseLibrary {
    pub id: Uuid,
    pub name: String,
    pub clause_type: String,
    pub content: String,
    pub description: Option<String>,
    pub is_mandatory: bool,
    pub risk_level: ContractRisk,
    pub version: i32,
    pub effective_date: DateTime<Utc>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractMilestone {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub due_date: DateTime<Utc>,
    pub completed_date: Option<DateTime<Utc>>,
    pub is_billing_event: bool,
    pub billing_amount: Option<i64>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractObligation {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub obligation_type: String,
    pub description: String,
    pub responsible_party: String,
    pub frequency: String,
    pub next_due_date: DateTime<Utc>,
    pub last_completed: Option<DateTime<Utc>>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractAmendment {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub amendment_number: String,
    pub title: String,
    pub description: String,
    pub effective_date: DateTime<Utc>,
    pub status: ContractStatus,
    pub changes_summary: String,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub approved_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractDocument {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub document_type: String,
    pub document_name: String,
    pub file_path: String,
    pub file_size: i64,
    pub version: i32,
    pub uploaded_by: Option<Uuid>,
    pub uploaded_at: DateTime<Utc>,
    pub is_signed: bool,
    pub signed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
    ChangesRequested,
    Delegated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractApproval {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub approver_id: Uuid,
    pub approval_level: i32,
    pub status: ApprovalStatus,
    pub comments: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalWorkflow {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub contract_type: String,
    pub min_value: Option<i64>,
    pub max_value: Option<i64>,
    pub levels: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalWorkflowLevel {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub level_number: i32,
    pub approver_type: String,
    pub approver_id: Option<Uuid>,
    pub role_id: Option<Uuid>,
    pub is_parallel: bool,
    pub timeout_days: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractSignature {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub signer_type: String,
    pub signer_id: Option<Uuid>,
    pub signer_name: String,
    pub signer_email: String,
    pub signer_title: Option<String>,
    pub status: String,
    pub signed_at: Option<DateTime<Utc>>,
    pub signature_data: Option<String>,
    pub ip_address: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractRenewal {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub renewal_type: String,
    pub new_start_date: DateTime<Utc>,
    pub new_end_date: DateTime<Utc>,
    pub new_value: Option<i64>,
    pub status: String,
    pub initiated_by: Option<Uuid>,
    pub initiated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractAlert {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub alert_type: String,
    pub alert_date: DateTime<Utc>,
    pub message: String,
    pub recipients: String,
    pub is_sent: bool,
    pub sent_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractSpend {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub committed_amount: i64,
    pub spent_amount: i64,
    pub remaining_amount: i64,
    pub utilization_percent: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractVendorPerformance {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub vendor_id: Uuid,
    pub evaluation_period: String,
    pub on_time_delivery_pct: f64,
    pub quality_score: f64,
    pub responsiveness_score: f64,
    pub compliance_score: f64,
    pub overall_score: f64,
    pub notes: Option<String>,
    pub evaluated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractRiskAssessment {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub assessment_date: DateTime<Utc>,
    pub financial_risk: String,
    pub legal_risk: String,
    pub operational_risk: String,
    pub compliance_risk: String,
    pub overall_risk: ContractRisk,
    pub mitigation_notes: Option<String>,
    pub assessed_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractType {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub default_term_months: i32,
    pub auto_renew_default: bool,
    pub required_clauses: String,
    pub approval_workflow_id: Option<Uuid>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractTerm {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub term_name: String,
    pub term_value: String,
    pub term_unit: String,
    pub is_custom: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheck {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub check_type: String,
    pub check_name: String,
    pub result: String,
    pub details: Option<String>,
    pub checked_at: DateTime<Utc>,
    pub checked_by: Option<Uuid>,
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum ItemStatus {
    Draft,
    InDesign,
    InReview,
    Approved,
    Released,
    Obsolete,
    Superseded,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum ChangeRequestStatus {
    Draft,
    Submitted,
    UnderReview,
    Approved,
    Implemented,
    Rejected,
    Cancelled,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum ChangeRequestPriority {
    Low,
    Medium,
    High,
    Critical,
    Emergency,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum DocumentType {
    Drawing,
    Specification,
    Procedure,
    WorkInstruction,
    TestReport,
    Certificate,
    Manual,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PLMItem {
    pub id: Uuid,
    pub item_number: String,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub status: ItemStatus,
    pub version: String,
    pub revision: i32,
    pub lifecycle_phase: String,
    pub owner_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub parent_item_id: Option<Uuid>,
    pub effective_date: Option<DateTime<Utc>>,
    pub obsolete_date: Option<DateTime<Utc>>,
    pub security_classification: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PLMDocument {
    pub id: Uuid,
    pub document_number: String,
    pub title: String,
    pub description: Option<String>,
    pub document_type: DocumentType,
    pub status: ItemStatus,
    pub version: String,
    pub revision: i32,
    pub file_path: Option<String>,
    pub file_size: Option<i64>,
    pub file_format: Option<String>,
    pub checksum: Option<String>,
    pub owner_id: Option<Uuid>,
    pub checked_out_by: Option<Uuid>,
    pub checked_out_at: Option<DateTime<Utc>>,
    pub effective_date: Option<DateTime<Utc>>,
    pub obsolete_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PLMBOM {
    pub id: Uuid,
    pub bom_number: String,
    pub name: String,
    pub description: Option<String>,
    pub item_id: Uuid,
    pub version: String,
    pub revision: i32,
    pub status: ItemStatus,
    pub bom_type: String,
    pub quantity: f64,
    pub unit_of_measure: String,
    pub effective_date: Option<DateTime<Utc>>,
    pub obsolete_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PLMBOMLine {
    pub id: Uuid,
    pub bom_id: Uuid,
    pub item_id: Uuid,
    pub line_number: i32,
    pub quantity: f64,
    pub unit_of_measure: String,
    pub find_number: Option<i32>,
    pub reference_designator: Option<String>,
    pub substitute_item_id: Option<Uuid>,
    pub is_phantom: bool,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineeringChangeRequest {
    pub id: Uuid,
    pub ecr_number: String,
    pub title: String,
    pub description: String,
    pub reason: String,
    pub priority: ChangeRequestPriority,
    pub status: ChangeRequestStatus,
    pub change_type: String,
    pub requested_by: Uuid,
    pub submitted_at: Option<DateTime<Utc>>,
    pub target_date: Option<DateTime<Utc>>,
    pub implemented_date: Option<DateTime<Utc>>,
    pub impact_assessment: Option<String>,
    pub cost_estimate: Option<i64>,
    pub currency: Option<String>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub rejected_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineeringChangeNotice {
    pub id: Uuid,
    pub ecn_number: String,
    pub ecr_id: Uuid,
    pub title: String,
    pub description: String,
    pub status: ChangeRequestStatus,
    pub effective_date: DateTime<Utc>,
    pub implementation_instructions: Option<String>,
    pub created_by: Uuid,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ECNAffectedItem {
    pub id: Uuid,
    pub ecn_id: Uuid,
    pub item_id: Uuid,
    pub old_revision: String,
    pub new_revision: String,
    pub old_version: String,
    pub new_version: String,
    pub change_description: String,
    pub disposition: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PLMWorkflow {
    pub id: Uuid,
    pub workflow_number: String,
    pub name: String,
    pub description: Option<String>,
    pub workflow_type: String,
    pub status: String,
    pub initiated_by: Uuid,
    pub current_step: i32,
    pub total_steps: i32,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PLMWorkflowStep {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub step_number: i32,
    pub step_name: String,
    pub step_type: String,
    pub assignee_id: Option<Uuid>,
    pub role_id: Option<Uuid>,
    pub status: String,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub completed_by: Option<Uuid>,
    pub comments: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CADFile {
    pub id: Uuid,
    pub document_id: Uuid,
    pub file_name: String,
    pub file_path: String,
    pub file_size: i64,
    pub cad_system: String,
    pub format: String,
    pub version: String,
    pub thumbnail_path: Option<String>,
    pub geometry_data: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemRelationship {
    pub id: Uuid,
    pub parent_item_id: Uuid,
    pub child_item_id: Uuid,
    pub relationship_type: String,
    pub quantity: f64,
    pub unit_of_measure: String,
    pub effective_date: Option<DateTime<Utc>>,
    pub obsolete_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Specification {
    pub id: Uuid,
    pub spec_number: String,
    pub name: String,
    pub description: Option<String>,
    pub item_id: Option<Uuid>,
    pub spec_type: String,
    pub status: ItemStatus,
    pub version: String,
    pub revision: i32,
    pub parameters: serde_json::Value,
    pub owner_id: Option<Uuid>,
    pub effective_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecificationParameter {
    pub id: Uuid,
    pub spec_id: Uuid,
    pub parameter_name: String,
    pub parameter_type: String,
    pub target_value: String,
    pub min_value: Option<String>,
    pub max_value: Option<String>,
    pub unit: Option<String>,
    pub test_method: Option<String>,
    pub is_critical: bool,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignReview {
    pub id: Uuid,
    pub review_number: String,
    pub item_id: Uuid,
    pub review_type: String,
    pub status: String,
    pub scheduled_date: DateTime<Utc>,
    pub conducted_date: Option<DateTime<Utc>>,
    pub facilitator_id: Option<Uuid>,
    pub location: Option<String>,
    pub outcome: Option<String>,
    pub action_items: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignReviewAttendee {
    pub id: Uuid,
    pub review_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub attended: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRequirement {
    pub id: Uuid,
    pub requirement_code: String,
    pub name: String,
    pub description: Option<String>,
    pub regulation: String,
    pub category: String,
    pub mandatory: bool,
    pub verification_method: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemCompliance {
    pub id: Uuid,
    pub item_id: Uuid,
    pub requirement_id: Uuid,
    pub status: String,
    pub certified: bool,
    pub certification_date: Option<DateTime<Utc>>,
    pub certification_expiry: Option<DateTime<Utc>>,
    pub certifying_body: Option<String>,
    pub certificate_number: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateECRRequest {
    pub title: String,
    pub description: String,
    pub reason: String,
    pub priority: ChangeRequestPriority,
    pub change_type: String,
    pub target_date: Option<DateTime<Utc>>,
    pub cost_estimate: Option<i64>,
    pub currency: Option<String>,
    pub affected_items: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePLMItemRequest {
    pub item_number: String,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub product_id: Option<Uuid>,
    pub parent_item_id: Option<Uuid>,
    pub security_classification: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSpecificationRequest {
    pub spec_number: String,
    pub name: String,
    pub description: Option<String>,
    pub item_id: Option<Uuid>,
    pub spec_type: String,
    pub parameters: Vec<CreateSpecParameterRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSpecParameterRequest {
    pub parameter_name: String,
    pub parameter_type: String,
    pub target_value: String,
    pub min_value: Option<String>,
    pub max_value: Option<String>,
    pub unit: Option<String>,
    pub test_method: Option<String>,
    pub is_critical: bool,
}

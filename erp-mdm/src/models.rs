use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum DataQualityScore {
    Excellent,
    Good,
    Fair,
    Poor,
    Critical,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum MatchStatus {
    Pending,
    Matched,
    Unmatched,
    Confirmed,
    Rejected,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum GovernanceStatus {
    Active,
    InReview,
    Deprecated,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterDataEntity {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_code: String,
    pub entity_name: String,
    pub source_system: String,
    pub source_id: String,
    pub golden_record_id: Option<Uuid>,
    pub quality_score: i32,
    pub completeness_score: i32,
    pub accuracy_score: i32,
    pub timeliness_score: i32,
    pub consistency_score: i32,
    pub last_verified: Option<DateTime<Utc>>,
    pub next_verification: Option<DateTime<Utc>>,
    pub status: GovernanceStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenRecord {
    pub id: Uuid,
    pub entity_type: String,
    pub golden_code: String,
    pub name: String,
    pub attributes: serde_json::Value,
    pub source_count: i32,
    pub confidence_score: i32,
    pub steward_id: Option<Uuid>,
    pub status: GovernanceStatus,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenRecordSource {
    pub id: Uuid,
    pub golden_record_id: Uuid,
    pub source_entity_id: Uuid,
    pub match_score: i32,
    pub is_primary: bool,
    pub contributed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQualityRule {
    pub id: Uuid,
    pub rule_code: String,
    pub name: String,
    pub description: Option<String>,
    pub entity_type: String,
    pub field_name: String,
    pub rule_type: String,
    pub rule_expression: String,
    pub severity: String,
    pub is_active: bool,
    pub auto_fix: bool,
    pub fix_expression: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQualityViolation {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub entity_id: Uuid,
    pub entity_type: String,
    pub field_name: String,
    pub current_value: Option<String>,
    pub expected_value: Option<String>,
    pub severity: String,
    pub status: String,
    pub detected_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<Uuid>,
    pub resolution_notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchRule {
    pub id: Uuid,
    pub rule_code: String,
    pub name: String,
    pub description: Option<String>,
    pub entity_type: String,
    pub match_type: String,
    pub blocking_rules: serde_json::Value,
    pub matching_rules: serde_json::Value,
    pub threshold_score: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub entity1_id: Uuid,
    pub entity2_id: Uuid,
    pub match_score: i32,
    pub status: MatchStatus,
    pub matched_at: DateTime<Utc>,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub decision_notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataDomain {
    pub id: Uuid,
    pub domain_code: String,
    pub name: String,
    pub description: Option<String>,
    pub parent_domain_id: Option<Uuid>,
    pub owner_id: Option<Uuid>,
    pub steward_id: Option<Uuid>,
    pub data_classification: String,
    pub retention_policy: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAttribute {
    pub id: Uuid,
    pub domain_id: Uuid,
    pub attribute_code: String,
    pub name: String,
    pub description: Option<String>,
    pub data_type: String,
    pub max_length: Option<i32>,
    pub is_required: bool,
    pub is_unique: bool,
    pub default_value: Option<String>,
    pub validation_regex: Option<String>,
    pub allowed_values: Option<serde_json::Value>,
    pub business_rules: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSteward {
    pub id: Uuid,
    pub user_id: Uuid,
    pub domain_id: Option<Uuid>,
    pub entity_types: serde_json::Value,
    pub responsibilities: serde_json::Value,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataLineage {
    pub id: Uuid,
    pub source_entity_id: Uuid,
    pub target_entity_id: Uuid,
    pub transformation_type: String,
    pub transformation_logic: Option<String>,
    pub flow_type: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataWorkflow {
    pub id: Uuid,
    pub workflow_code: String,
    pub name: String,
    pub description: Option<String>,
    pub entity_type: String,
    pub workflow_type: String,
    pub status: String,
    pub initiated_by: Option<Uuid>,
    pub current_step: i32,
    pub total_steps: i32,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataWorkflowStep {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub step_number: i32,
    pub step_name: String,
    pub action_type: String,
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
pub struct DuplicateRecord {
    pub id: Uuid,
    pub entity_type: String,
    pub primary_entity_id: Uuid,
    pub duplicate_entity_id: Uuid,
    pub similarity_score: i32,
    pub matched_fields: serde_json::Value,
    pub status: String,
    pub merge_initiated_by: Option<Uuid>,
    pub merge_initiated_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataImportJob {
    pub id: Uuid,
    pub job_name: String,
    pub entity_type: String,
    pub source_file: String,
    pub total_records: i32,
    pub processed_records: i32,
    pub success_records: i32,
    pub failed_records: i32,
    pub duplicate_records: i32,
    pub status: String,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_log: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataExportJob {
    pub id: Uuid,
    pub job_name: String,
    pub entity_type: String,
    pub filter_criteria: Option<serde_json::Value>,
    pub export_format: String,
    pub output_file: Option<String>,
    pub total_records: i32,
    pub status: String,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceData {
    pub id: Uuid,
    pub category: String,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub parent_code: Option<String>,
    pub sort_order: i32,
    pub is_active: bool,
    pub effective_date: Option<DateTime<Utc>>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub attributes: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQualityDashboard {
    pub entity_type: String,
    pub total_records: i64,
    pub records_with_issues: i64,
    pub avg_completeness: f64,
    pub avg_accuracy: f64,
    pub avg_timeliness: f64,
    pub avg_consistency: f64,
    pub overall_score: f64,
    pub issues_by_severity: serde_json::Value,
    pub issues_by_rule: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGoldenRecordRequest {
    pub entity_type: String,
    pub name: String,
    pub attributes: serde_json::Value,
    pub source_entity_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDataQualityRuleRequest {
    pub rule_code: String,
    pub name: String,
    pub description: Option<String>,
    pub entity_type: String,
    pub field_name: String,
    pub rule_type: String,
    pub rule_expression: String,
    pub severity: String,
    pub auto_fix: bool,
    pub fix_expression: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeRecordsRequest {
    pub primary_entity_id: Uuid,
    pub duplicate_entity_ids: Vec<Uuid>,
    pub merge_strategy: String,
}

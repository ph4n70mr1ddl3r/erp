use chrono::{DateTime, Utc};
use erp_core::models::BaseEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RuleType {
    Completeness,
    Uniqueness,
    Accuracy,
    Consistency,
    Validity,
    Timeliness,
    Format,
    Range,
    Pattern,
    BusinessRule,
    Referential,
    Conditional,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RuleSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQualityRule {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub rule_type: RuleType,
    pub severity: RuleSeverity,
    pub target_entity: String,
    pub target_field: String,
    pub condition: String,
    pub threshold: Option<f64>,
    pub is_active: bool,
    pub schedule: Option<String>,
    pub last_run: Option<DateTime<Utc>>,
    pub last_result: Option<QualityScore>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityScore {
    pub score: f64,
    pub grade: QualityGrade,
    pub passed_records: i64,
    pub failed_records: i64,
    pub total_records: i64,
    pub error_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum QualityGrade {
    A,
    B,
    C,
    D,
    F,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQualityExecution {
    pub base: BaseEntity,
    pub rule_id: Uuid,
    pub executed_at: DateTime<Utc>,
    pub duration_ms: i64,
    pub status: ExecutionStatus,
    pub score: QualityScore,
    pub errors: Vec<DataQualityError>,
    pub warnings: Vec<String>,
    pub records_processed: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ExecutionStatus {
    Running,
    Completed,
    Failed,
    Cancelled,
    Timeout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQualityError {
    pub record_id: Option<String>,
    pub field: String,
    pub error_type: String,
    pub message: String,
    pub actual_value: Option<String>,
    pub expected_value: Option<String>,
    pub severity: RuleSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQualityProfile {
    pub base: BaseEntity,
    pub name: String,
    pub entity: String,
    pub profile_date: DateTime<Utc>,
    pub total_records: i64,
    pub field_profiles: Vec<FieldProfile>,
    pub overall_quality_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldProfile {
    pub field_name: String,
    pub data_type: String,
    pub null_count: i64,
    pub null_percent: f64,
    pub unique_count: i64,
    pub unique_percent: f64,
    pub distinct_values: i64,
    pub min_value: Option<String>,
    pub max_value: Option<String>,
    pub avg_value: Option<f64>,
    pub std_dev: Option<f64>,
    pub pattern_match_percent: Option<f64>,
    pub top_values: Vec<ValueFrequency>,
    pub outliers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueFrequency {
    pub value: String,
    pub count: i64,
    pub percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCleansingJob {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub source_entity: String,
    pub target_entity: Option<String>,
    pub transformations: Vec<DataTransformation>,
    pub status: JobStatus,
    pub created_by: Uuid,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub records_processed: i64,
    pub records_modified: i64,
    pub records_failed: i64,
    pub error_log: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    PartiallyCompleted,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTransformation {
    pub field: String,
    pub transformation_type: TransformationType,
    pub parameters: serde_json::Value,
    pub order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TransformationType {
    Trim,
    Uppercase,
    Lowercase,
    TitleCase,
    Replace,
    RegexReplace,
    Format,
    Parse,
    Standardize,
    DefaultValue,
    RemoveNulls,
    RemoveDuplicates,
    Split,
    Concat,
    Substring,
    Math,
    Lookup,
    Conditional,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataMatchingRule {
    pub base: BaseEntity,
    pub name: String,
    pub entity: String,
    pub match_fields: Vec<MatchField>,
    pub blocking_keys: Vec<String>,
    pub match_threshold: f64,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchField {
    pub field: String,
    pub comparison_method: ComparisonMethod,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ComparisonMethod {
    Exact,
    Fuzzy,
    Soundex,
    Levenshtein,
    JaroWinkler,
    QGram,
    Numeric,
    Date,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateGroup {
    pub base: BaseEntity,
    pub entity: String,
    pub canonical_id: String,
    pub duplicate_ids: Vec<String>,
    pub match_score: f64,
    pub detected_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<Uuid>,
    pub resolution_type: Option<ResolutionType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ResolutionType {
    Merge,
    KeepBoth,
    MarkAsNotDuplicate,
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataValidationSession {
    pub base: BaseEntity,
    pub name: String,
    pub entity: String,
    pub rule_ids: Vec<Uuid>,
    pub status: SessionStatus,
    pub created_by: Uuid,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub overall_score: Option<QualityScore>,
    pub rule_results: Vec<RuleResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SessionStatus {
    Created,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleResult {
    pub rule_id: Uuid,
    pub rule_name: String,
    pub score: QualityScore,
    pub errors: Vec<DataQualityError>,
    pub execution_time_ms: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQualityDashboard {
    pub base: BaseEntity,
    pub name: String,
    pub entity_scores: Vec<EntityQualityScore>,
    pub trend_data: Vec<QualityTrendPoint>,
    pub top_issues: Vec<QualityIssue>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityQualityScore {
    pub entity: String,
    pub score: f64,
    pub grade: QualityGrade,
    pub record_count: i64,
    pub error_count: i64,
    pub change_from_previous: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityTrendPoint {
    pub date: DateTime<Utc>,
    pub score: f64,
    pub entity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityIssue {
    pub entity: String,
    pub field: String,
    pub issue_type: String,
    pub count: i64,
    pub severity: RuleSeverity,
    pub trend: IssueTrend,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum IssueTrend {
    Increasing,
    Decreasing,
    Stable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQualityThreshold {
    pub base: BaseEntity,
    pub entity: String,
    pub metric: QualityMetric,
    pub warning_threshold: f64,
    pub critical_threshold: f64,
    pub notification_emails: Vec<String>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum QualityMetric {
    Completeness,
    Accuracy,
    Consistency,
    Timeliness,
    Validity,
    Uniqueness,
    OverallScore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQualityAlert {
    pub base: BaseEntity,
    pub threshold_id: Uuid,
    pub entity: String,
    pub metric: QualityMetric,
    pub current_value: f64,
    pub threshold_value: f64,
    pub severity: AlertSeverity,
    pub triggered_at: DateTime<Utc>,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub acknowledged_by: Option<Uuid>,
    pub resolved_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AlertSeverity {
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSteward {
    pub base: BaseEntity,
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub entities: Vec<String>,
    pub responsibilities: Vec<String>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQualityReport {
    pub base: BaseEntity,
    pub name: String,
    pub report_type: ReportType,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub generated_at: DateTime<Utc>,
    pub generated_by: Uuid,
    pub summary: QualityReportSummary,
    pub entity_details: Vec<EntityReportDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReportType {
    Executive,
    Detailed,
    Trend,
    Compliance,
    Remediation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityReportSummary {
    pub overall_score: f64,
    pub overall_grade: QualityGrade,
    pub total_rules_executed: i64,
    pub total_errors_found: i64,
    pub improvement_from_previous: f64,
    pub top_issues: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityReportDetail {
    pub entity: String,
    pub score: f64,
    pub grade: QualityGrade,
    pub record_count: i64,
    pub error_breakdown: std::collections::HashMap<String, i64>,
    pub field_scores: std::collections::HashMap<String, f64>,
}

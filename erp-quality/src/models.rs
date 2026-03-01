use chrono::{DateTime, NaiveDate, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CAPASource {
    CustomerComplaint,
    InternalAudit,
    ExternalAudit,
    SupplierIssue,
    ProcessDeviation,
    ProductFailure,
    RegulatoryFinding,
    NearMiss,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CAPASeverity {
    Minor,
    Major,
    Critical,
    Catastrophic,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CAPAStatus {
    Draft,
    Submitted,
    UnderInvestigation,
    RootCauseIdentified,
    CorrectiveActionPlanned,
    InProgress,
    VerificationPending,
    Effective,
    Closed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CAPAPhase {
    Identification,
    Investigation,
    RootCauseAnalysis,
    CorrectiveAction,
    PreventiveAction,
    Verification,
    Closure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CAPA {
    pub base: BaseEntity,
    pub capa_number: String,
    pub title: String,
    pub description: String,
    pub source: CAPASource,
    pub severity: CAPASeverity,
    pub status: CAPAStatus,
    pub current_phase: CAPAPhase,
    pub product_id: Option<Uuid>,
    pub process_id: Option<Uuid>,
    pub supplier_id: Option<Uuid>,
    pub customer_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub detected_date: NaiveDate,
    pub detected_by: Option<Uuid>,
    pub assigned_to: Option<Uuid>,
    pub assigned_date: Option<NaiveDate>,
    pub target_completion_date: Option<NaiveDate>,
    pub actual_completion_date: Option<NaiveDate>,
    pub effectiveness_review_date: Option<NaiveDate>,
    pub is_effective: Option<bool>,
    pub closure_date: Option<NaiveDate>,
    pub closed_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CAPAInvestigation {
    pub id: Uuid,
    pub capa_id: Uuid,
    pub investigation_date: NaiveDate,
    pub investigator_id: Option<Uuid>,
    pub what_happened: String,
    pub when_it_happened: Option<String>,
    pub where_it_happened: Option<String>,
    pub who_was_involved: Option<String>,
    pub immediate_action_taken: Option<String>,
    pub evidence_collected: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RootCauseCategory {
    Man,
    Machine,
    Material,
    Method,
    Measurement,
    Environment,
    Management,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootCauseAnalysis {
    pub id: Uuid,
    pub capa_id: Uuid,
    pub analysis_method: String,
    pub root_cause_category: RootCauseCategory,
    pub root_cause_description: String,
    pub contributing_factors: Option<String>,
    pub is_primary: bool,
    pub verified: bool,
    pub verified_by: Option<Uuid>,
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ActionType {
    Corrective,
    Preventive,
    Containment,
    Improvement,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ActionStatus {
    Planned,
    InProgress,
    Completed,
    Verified,
    Ineffective,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CAPAAction {
    pub id: Uuid,
    pub capa_id: Uuid,
    pub action_type: ActionType,
    pub action_number: i32,
    pub description: String,
    pub root_cause_id: Option<Uuid>,
    pub responsible_person_id: Option<Uuid>,
    pub planned_date: NaiveDate,
    pub completed_date: Option<NaiveDate>,
    pub status: ActionStatus,
    pub verification_method: Option<String>,
    pub verification_date: Option<NaiveDate>,
    pub verified_by: Option<Uuid>,
    pub effectiveness_notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AuditType {
    Internal,
    External,
    Supplier,
    Customer,
    Regulatory,
    Certification,
    Surveillance,
    Recertification,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AuditStatus {
    Planned,
    Scheduled,
    InProgress,
    ReportDraft,
    ReportFinal,
    Closed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditProgram {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub audit_type: AuditType,
    pub standard_reference: Option<String>,
    pub frequency_months: i32,
    pub next_audit_date: Option<NaiveDate>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditSchedule {
    pub base: BaseEntity,
    pub audit_number: String,
    pub program_id: Option<Uuid>,
    pub title: String,
    pub audit_type: AuditType,
    pub scope: String,
    pub objectives: Option<String>,
    pub criteria: Option<String>,
    pub planned_start_date: NaiveDate,
    pub planned_end_date: NaiveDate,
    pub actual_start_date: Option<NaiveDate>,
    pub actual_end_date: Option<NaiveDate>,
    pub lead_auditor_id: Option<Uuid>,
    pub status: AuditStatus,
    pub overall_rating: Option<AuditRating>,
    pub summary: Option<String>,
    pub conclusions: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditTeamMember {
    pub id: Uuid,
    pub audit_id: Uuid,
    pub employee_id: Uuid,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditChecklist {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub standard_reference: Option<String>,
    pub items: Vec<ChecklistItem>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistItem {
    pub id: Uuid,
    pub checklist_id: Uuid,
    pub item_number: i32,
    pub clause_reference: Option<String>,
    pub question: String,
    pub guidance: Option<String>,
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditFinding {
    pub base: BaseEntity,
    pub audit_id: Uuid,
    pub finding_number: String,
    pub checklist_item_id: Option<Uuid>,
    pub finding_type: FindingType,
    pub severity: FindingSeverity,
    pub description: String,
    pub evidence: Option<String>,
    pub requirement_reference: Option<String>,
    pub auditee_response: Option<String>,
    pub capa_id: Option<Uuid>,
    pub status: FindingStatus,
    pub verified: bool,
    pub verified_by: Option<Uuid>,
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum FindingType {
    Observation,
    MinorNonconformity,
    MajorNonconformity,
    OpportunityForImprovement,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum FindingSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum FindingStatus {
    Open,
    Acknowledged,
    CAPAInitiated,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AuditRating {
    Excellent,
    Good,
    Satisfactory,
    NeedsImprovement,
    Unsatisfactory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationEquipment {
    pub base: BaseEntity,
    pub equipment_number: String,
    pub name: String,
    pub description: Option<String>,
    pub serial_number: Option<String>,
    pub model: Option<String>,
    pub manufacturer: Option<String>,
    pub equipment_type: String,
    pub location_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub responsible_person_id: Option<Uuid>,
    pub calibration_frequency_months: i32,
    pub last_calibration_date: Option<NaiveDate>,
    pub next_calibration_date: Option<NaiveDate>,
    pub calibration_status: CalibrationStatus,
    pub accuracy_class: Option<String>,
    pub measurement_range: Option<String>,
    pub resolution: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CalibrationStatus {
    Current,
    DueSoon,
    Overdue,
    OutOfService,
    Retired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationRecord {
    pub base: BaseEntity,
    pub calibration_number: String,
    pub equipment_id: Uuid,
    pub calibration_date: NaiveDate,
    pub calibration_type: CalibrationType,
    pub calibration_lab: Option<String>,
    pub lab_certificate_number: Option<String>,
    pub performed_by: Option<Uuid>,
    pub environmental_conditions: Option<String>,
    pub standards_used: Option<String>,
    pub before_calibration: Option<String>,
    pub after_calibration: Option<String>,
    pub as_found_status: AsFoundStatus,
    pub as_left_status: AsLeftStatus,
    pub result: CalibrationResult,
    pub next_calibration_date: NaiveDate,
    pub cost: i64,
    pub currency: String,
    pub certificate_file: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CalibrationType {
    Internal,
    External,
    Verification,
    Adjustment,
    Repair,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AsFoundStatus {
    InTolerance,
    OutOfTolerance,
    MinorDeviation,
    MajorDeviation,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AsLeftStatus {
    InTolerance,
    OutOfTolerance,
    Adjusted,
    Repaired,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CalibrationResult {
    Pass,
    Fail,
    PassWithCondition,
    PendingReview,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationPoint {
    pub id: Uuid,
    pub calibration_id: Uuid,
    pub point_number: i32,
    pub parameter: String,
    pub nominal_value: f64,
    pub tolerance_plus: f64,
    pub tolerance_minus: f64,
    pub as_found_value: Option<f64>,
    pub as_found_pass: Option<bool>,
    pub as_left_value: Option<f64>,
    pub as_left_pass: Option<bool>,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlPlan {
    pub base: BaseEntity,
    pub plan_number: String,
    pub name: String,
    pub description: Option<String>,
    pub product_id: Option<Uuid>,
    pub process_id: Option<Uuid>,
    pub revision: String,
    pub effective_date: NaiveDate,
    pub approval_status: ApprovalStatus,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ApprovalStatus {
    Draft,
    Submitted,
    Approved,
    Rejected,
    Superseded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlPlanItem {
    pub id: Uuid,
    pub control_plan_id: Uuid,
    pub operation_number: i32,
    pub operation_name: String,
    pub characteristic: String,
    pub specification: String,
    pub tolerance: Option<String>,
    pub inspection_method: String,
    pub inspection_frequency: String,
    pub sample_size: Option<String>,
    pub control_method: String,
    pub reaction_plan: Option<String>,
    pub responsible_role: Option<String>,
    pub gage_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentControl {
    pub base: BaseEntity,
    pub document_number: String,
    pub title: String,
    pub document_type: String,
    pub category: Option<String>,
    pub department_id: Option<Uuid>,
    pub owner_id: Option<Uuid>,
    pub current_revision: String,
    pub effective_date: Option<NaiveDate>,
    pub review_frequency_months: i32,
    pub next_review_date: Option<NaiveDate>,
    pub approval_workflow_id: Option<Uuid>,
    pub distribution_list: Option<String>,
    pub retention_years: i32,
    pub status: DocumentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DocumentStatus {
    Draft,
    InReview,
    Approved,
    Published,
    Obsolete,
    Withdrawn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRevision {
    pub id: Uuid,
    pub document_id: Uuid,
    pub revision: String,
    pub change_description: Option<String>,
    pub file_path: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub checksum: Option<String>,
    pub status: DocumentStatus,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub effective_date: Option<NaiveDate>,
    pub superseded_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierQualityRating {
    pub base: BaseEntity,
    pub vendor_id: Uuid,
    pub rating_period: String,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_deliveries: i32,
    pub on_time_deliveries: i32,
    pub rejected_lots: i32,
    pub total_lots: i32,
    pub ncr_count: i32,
    pub capa_count: i32,
    pub quality_score: f64,
    pub delivery_score: f64,
    pub responsiveness_score: f64,
    pub overall_score: f64,
    pub rating: SupplierQualityGrade,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SupplierQualityGrade {
    A,
    B,
    C,
    D,
    F,
    Probation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalProcessControl {
    pub base: BaseEntity,
    pub spc_number: String,
    pub name: String,
    pub product_id: Option<Uuid>,
    pub process_id: Option<Uuid>,
    pub characteristic: String,
    pub specification_min: f64,
    pub specification_max: f64,
    pub target_value: f64,
    pub unit: String,
    pub sample_size: i32,
    pub sampling_frequency: String,
    pub subgroup_size: i32,
    pub control_chart_type: ControlChartType,
    pub ucl: f64,
    pub lcl: f64,
    pub center_line: f64,
    pub rule_set: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ControlChartType {
    XBarR,
    XBarS,
    IMr,
    PChart,
    NPChart,
    CChart,
    UChart,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SPCMeasurement {
    pub id: Uuid,
    pub spc_id: Uuid,
    pub subgroup_number: i32,
    pub measurement_date: DateTime<Utc>,
    pub measured_by: Option<Uuid>,
    pub values: Vec<f64>,
    pub mean: f64,
    pub range: f64,
    pub std_dev: Option<f64>,
    pub out_of_control: bool,
    pub rule_violations: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureModeEffect {
    pub base: BaseEntity,
    pub fmea_number: String,
    pub name: String,
    pub fmea_type: FMEAType,
    pub product_id: Option<Uuid>,
    pub process_id: Option<Uuid>,
    pub prepared_by: Option<Uuid>,
    pub prepared_date: NaiveDate,
    pub revision: String,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum FMEAType {
    Design,
    Process,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FMEALineItem {
    pub id: Uuid,
    pub fmea_id: Uuid,
    pub item_number: i32,
    pub component_or_step: String,
    pub function: String,
    pub failure_mode: String,
    pub potential_effect: String,
    pub severity: i32,
    pub potential_cause: String,
    pub occurrence: i32,
    pub current_controls: String,
    pub detection: i32,
    pub rpn: i32,
    pub recommended_action: Option<String>,
    pub responsibility: Option<Uuid>,
    pub target_date: Option<NaiveDate>,
    pub action_taken: Option<String>,
    pub new_severity: Option<i32>,
    pub new_occurrence: Option<i32>,
    pub new_detection: Option<i32>,
    pub new_rpn: Option<i32>,
    pub created_at: DateTime<Utc>,
}

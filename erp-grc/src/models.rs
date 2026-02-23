use chrono::{DateTime, NaiveDate, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RiskCategory {
    Strategic,
    Operational,
    Financial,
    Compliance,
    Reputational,
    Technology,
    Cybersecurity,
    ThirdParty,
    Environmental,
    HumanCapital,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RiskSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RiskStatus {
    Identified,
    Assessing,
    Mitigating,
    Monitoring,
    Closed,
    Accepted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Risk {
    pub base: BaseEntity,
    pub risk_number: String,
    pub title: String,
    pub description: String,
    pub category: RiskCategory,
    pub subcategory: Option<String>,
    pub owner_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub process_id: Option<Uuid>,
    pub identified_date: NaiveDate,
    pub likelihood: i32,
    pub impact: i32,
    pub inherent_risk_score: i32,
    pub inherent_risk_level: RiskSeverity,
    pub control_effectiveness: Option<i32>,
    pub residual_likelihood: Option<i32>,
    pub residual_impact: Option<i32>,
    pub residual_risk_score: Option<i32>,
    pub residual_risk_level: Option<RiskSeverity>,
    pub target_risk_score: Option<i32>,
    pub risk_response: Option<RiskResponse>,
    pub status: RiskStatus,
    pub review_frequency_days: i32,
    pub last_review_date: Option<NaiveDate>,
    pub next_review_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RiskResponse {
    Mitigate,
    Transfer,
    Avoid,
    Accept,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub base: BaseEntity,
    pub risk_id: Uuid,
    pub assessment_date: NaiveDate,
    pub assessor_id: Option<Uuid>,
    pub likelihood_before: i32,
    pub impact_before: i32,
    pub score_before: i32,
    pub likelihood_after: Option<i32>,
    pub impact_after: Option<i32>,
    pub score_after: Option<i32>,
    pub assessment_method: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Control {
    pub base: BaseEntity,
    pub control_number: String,
    pub name: String,
    pub description: String,
    pub control_type: ControlType,
    pub control_nature: ControlNature,
    pub control_frequency: ControlFrequency,
    pub control_owner_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub process_id: Option<Uuid>,
    pub framework_reference: Option<String>,
    pub key_control: bool,
    pub automated: bool,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ControlType {
    Preventive,
    Detective,
    Corrective,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ControlNature {
    Manual,
    Automated,
    ITDependent,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ControlFrequency {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Annually,
    AdHoc,
    Continuous,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskControlMapping {
    pub id: Uuid,
    pub risk_id: Uuid,
    pub control_id: Uuid,
    pub mapping_type: String,
    pub effectiveness: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlTest {
    pub base: BaseEntity,
    pub test_number: String,
    pub control_id: Uuid,
    pub test_date: NaiveDate,
    pub tester_id: Option<Uuid>,
    pub test_type: TestType,
    pub sample_size: Option<i32>,
    pub population_size: Option<i32>,
    pub exceptions_found: i32,
    pub test_result: TestResult,
    pub effectiveness_rating: Option<i32>,
    pub findings: Option<String>,
    pub remediation_required: bool,
    pub remediation_due_date: Option<NaiveDate>,
    pub status: TestStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TestType {
    Inquiry,
    Observation,
    Inspection,
    Reperformance,
    DataAnalytics,
    Walkthrough,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TestResult {
    Effective,
    PartiallyEffective,
    Ineffective,
    DesignDeficiency,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TestStatus {
    Planned,
    InProgress,
    Completed,
    Remediation,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub base: BaseEntity,
    pub policy_number: String,
    pub title: String,
    pub description: Option<String>,
    pub category: PolicyCategory,
    pub owner_id: Option<Uuid>,
    pub approver_id: Option<Uuid>,
    pub effective_date: NaiveDate,
    pub review_frequency_months: i32,
    pub next_review_date: Option<NaiveDate>,
    pub version: String,
    pub document_path: Option<String>,
    pub status: PolicyStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PolicyCategory {
    InformationSecurity,
    DataPrivacy,
    HR,
    Finance,
    Operations,
    Ethics,
    Compliance,
    HealthSafety,
    Environmental,
    Quality,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PolicyStatus {
    Draft,
    UnderReview,
    Approved,
    Published,
    Deprecated,
    Retired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyAcknowledgment {
    pub id: Uuid,
    pub policy_id: Uuid,
    pub employee_id: Uuid,
    pub acknowledged_at: DateTime<Utc>,
    pub version: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceFramework {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub framework_type: FrameworkType,
    pub regulatory_body: Option<String>,
    pub jurisdiction: Option<String>,
    pub effective_date: Option<NaiveDate>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum FrameworkType {
    Regulatory,
    Industry,
    Internal,
    Certification,
    Contractual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRequirement {
    pub base: BaseEntity,
    pub framework_id: Uuid,
    pub requirement_code: String,
    pub title: String,
    pub description: String,
    pub parent_id: Option<Uuid>,
    pub control_owner_id: Option<Uuid>,
    pub evidence_required: bool,
    pub testing_required: bool,
    pub frequency: Option<String>,
    pub status: ComplianceStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ComplianceStatus {
    Compliant,
    NonCompliant,
    Partial,
    NotApplicable,
    NotAssessed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceAssessment {
    pub base: BaseEntity,
    pub requirement_id: Uuid,
    pub assessment_date: NaiveDate,
    pub assessor_id: Option<Uuid>,
    pub status: ComplianceStatus,
    pub evidence: Option<String>,
    pub gaps: Option<String>,
    pub remediation_plan: Option<String>,
    pub remediation_due_date: Option<NaiveDate>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Incident {
    pub base: BaseEntity,
    pub incident_number: String,
    pub title: String,
    pub description: String,
    pub incident_type: IncidentType,
    pub severity: IncidentSeverity,
    pub reported_by: Option<Uuid>,
    pub reported_date: NaiveDate,
    pub occurred_date: Option<NaiveDate>,
    pub discovered_date: Option<NaiveDate>,
    pub location: Option<String>,
    pub department_id: Option<Uuid>,
    pub affected_systems: Option<String>,
    pub affected_data: Option<String>,
    pub affected_parties: Option<String>,
    pub root_cause: Option<String>,
    pub immediate_actions: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub status: IncidentStatus,
    pub resolved_date: Option<NaiveDate>,
    pub closure_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum IncidentType {
    Security,
    Privacy,
    Fraud,
    Safety,
    Environmental,
    Operational,
    Compliance,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum IncidentSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum IncidentStatus {
    New,
    Investigating,
    Contained,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditFinding {
    pub base: BaseEntity,
    pub finding_number: String,
    pub audit_id: Option<Uuid>,
    pub title: String,
    pub description: String,
    pub finding_type: FindingType,
    pub severity: FindingSeverity,
    pub recommendation: Option<String>,
    pub management_response: Option<String>,
    pub action_plan: Option<String>,
    pub owner_id: Option<Uuid>,
    pub due_date: Option<NaiveDate>,
    pub status: FindingStatus,
    pub verified_by: Option<Uuid>,
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum FindingType {
    Observation,
    MinorNonConformity,
    MajorNonConformity,
    OpportunityForImprovement,
    BestPractice,
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
    InProgress,
    Implemented,
    Verified,
    Closed,
    Overdue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Regulation {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub regulatory_body: String,
    pub jurisdiction: String,
    pub effective_date: Option<NaiveDate>,
    pub compliance_deadline: Option<NaiveDate>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulationMapping {
    pub id: Uuid,
    pub regulation_id: Uuid,
    pub requirement_id: Uuid,
    pub relevance: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThirdPartyRisk {
    pub base: BaseEntity,
    pub vendor_id: Uuid,
    pub risk_tier: RiskTier,
    pub assessment_date: Option<NaiveDate>,
    pub next_assessment_date: Option<NaiveDate>,
    pub inherent_risk_score: Option<i32>,
    pub residual_risk_score: Option<i32>,
    pub data_access_level: DataAccessLevel,
    pub business_impact: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RiskTier {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DataAccessLevel {
    None,
    Limited,
    Moderate,
    Extensive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskHeatmap {
    pub id: Uuid,
    pub name: String,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub matrix: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KRIDefinition {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub category: RiskCategory,
    pub measurement_unit: String,
    pub calculation_method: Option<String>,
    pub data_source: Option<String>,
    pub frequency: String,
    pub threshold_green: f64,
    pub threshold_yellow: f64,
    pub threshold_red: f64,
    pub direction: DirectionType,
    pub owner_id: Option<Uuid>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DirectionType {
    LowerIsBetter,
    HigherIsBetter,
    WithinRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KRIMeasurement {
    pub id: Uuid,
    pub kpi_definition_id: Uuid,
    pub measurement_date: NaiveDate,
    pub value: f64,
    pub status: ThresholdStatus,
    pub trend: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ThresholdStatus {
    Green,
    Yellow,
    Red,
}

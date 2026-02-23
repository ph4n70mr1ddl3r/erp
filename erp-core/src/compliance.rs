use super::Status;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSubject {
    pub id: Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub identifier_type: Option<String>,
    pub identifier_value: Option<String>,
    pub verification_status: VerificationStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum VerificationStatus {
    Unverified,
    Pending,
    Verified,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRecord {
    pub id: Uuid,
    pub data_subject_id: Uuid,
    pub consent_type: String,
    pub purpose: String,
    pub legal_basis: LegalBasis,
    pub granted_at: Option<DateTime<Utc>>,
    pub withdrawn_at: Option<DateTime<Utc>>,
    pub source: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub evidence_path: Option<String>,
    pub status: ConsentStatus,
    pub expiry_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum LegalBasis {
    Consent,
    Contract,
    LegalObligation,
    VitalInterests,
    PublicTask,
    LegitimateInterest,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum ConsentStatus {
    Granted,
    Withdrawn,
    Expired,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataProcessingActivity {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub purpose: String,
    pub legal_basis: LegalBasis,
    pub data_categories: Vec<String>,
    pub data_subjects: Vec<String>,
    pub recipients: Vec<String>,
    pub third_country_transfers: Vec<String>,
    pub retention_period_days: i32,
    pub security_measures: Vec<String>,
    pub dpo_review_date: Option<NaiveDate>,
    pub status: Status,
    pub owner_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataBreach {
    pub id: Uuid,
    pub breach_number: String,
    pub title: String,
    pub description: String,
    pub breach_type: BreachType,
    pub severity: BreachSeverity,
    pub discovered_at: DateTime<Utc>,
    pub occurred_at: Option<DateTime<Utc>>,
    pub reported_at: Option<DateTime<Utc>>,
    pub affected_records: i32,
    pub affected_data_subjects: i32,
    pub data_categories: Vec<String>,
    pub containment_measures: Option<String>,
    pub remediation_measures: Option<String>,
    pub authority_notified: bool,
    pub authority_notification_date: Option<DateTime<Utc>>,
    pub subjects_notified: bool,
    pub subject_notification_date: Option<DateTime<Utc>>,
    pub status: BreachStatus,
    pub assigned_to: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BreachType {
    Confidentiality,
    Integrity,
    Availability,
    UnauthorizedAccess,
    UnauthorizedDisclosure,
    Loss,
    Destruction,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BreachSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum BreachStatus {
    Detected,
    Investigating,
    Contained,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DSARRequest {
    pub id: Uuid,
    pub request_number: String,
    pub data_subject_id: Uuid,
    pub request_type: DSARType,
    pub description: Option<String>,
    pub received_at: DateTime<Utc>,
    pub due_date: NaiveDate,
    pub completed_at: Option<DateTime<Utc>>,
    pub status: DSARStatus,
    pub assigned_to: Option<Uuid>,
    pub response: Option<String>,
    pub rejection_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DSARType {
    Access,
    Rectification,
    Erasure,
    Restriction,
    Portability,
    Objection,
    AutomatedDecision,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum DSARStatus {
    Received,
    Verification,
    InProgress,
    Completed,
    Rejected,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRetentionPolicy {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub data_category: String,
    pub retention_period_days: i32,
    pub legal_basis: Option<String>,
    pub disposal_method: DisposalMethod,
    pub review_frequency_days: i32,
    pub last_review_date: Option<NaiveDate>,
    pub next_review_date: Option<NaiveDate>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DisposalMethod {
    SecureDeletion,
    PhysicalDestruction,
    Anonymization,
    Pseudonymization,
    Archival,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyImpactAssessment {
    pub id: Uuid,
    pub name: String,
    pub project_name: String,
    pub description: String,
    pub assessor_id: Option<Uuid>,
    pub assessment_date: NaiveDate,
    pub data_types: Vec<String>,
    pub processing_purposes: Vec<String>,
    pub data_subjects: Vec<String>,
    pub risks: Vec<PIARisk>,
    pub mitigation_measures: Vec<String>,
    pub residual_risk_level: RiskLevel,
    pub recommendation: PIARecommendation,
    pub dpo_approval: bool,
    pub dpo_approved_at: Option<DateTime<Utc>>,
    pub dpo_comments: Option<String>,
    pub status: PIAStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PIARisk {
    pub id: Uuid,
    pub description: String,
    pub likelihood: RiskLevel,
    pub impact: RiskLevel,
    pub risk_level: RiskLevel,
    pub mitigation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RiskLevel {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum PIARecommendation {
    Proceed,
    ProceedWithConditions,
    DoNotProceed,
    ConsultDPO,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum PIAStatus {
    Draft,
    InProgress,
    UnderReview,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThirdPartyProcessor {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub contact_name: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub address: Option<String>,
    pub country: String,
    pub processing_activities: Vec<String>,
    pub data_categories: Vec<String>,
    pub contract_date: Option<NaiveDate>,
    pub contract_expiry: Option<NaiveDate>,
    pub dpa_signed: bool,
    pub security_assessment_date: Option<NaiveDate>,
    pub security_assessment_result: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookiePolicy {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub cookie_type: CookieType,
    pub purpose: String,
    pub provider: Option<String>,
    pub expiry: String,
    pub required: bool,
    pub consent_required: bool,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CookieType {
    Essential,
    Functional,
    Analytics,
    Marketing,
    ThirdParty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceFramework {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub framework_type: FrameworkType,
    pub version: String,
    pub applicable_regions: Vec<String>,
    pub requirements: Vec<ComplianceRequirement>,
    pub assessment_frequency_days: i32,
    pub last_assessment: Option<NaiveDate>,
    pub next_assessment: Option<NaiveDate>,
    pub compliance_status: ComplianceFrameworkStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum FrameworkType {
    GDPR,
    CCPA,
    HIPAA,
    SOX,
    ISO27001,
    PCI,
    SOC2,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRequirement {
    pub id: Uuid,
    pub code: String,
    pub title: String,
    pub description: String,
    pub is_satisfied: bool,
    pub evidence: Option<String>,
    pub gap_analysis: Option<String>,
    pub remediation_plan: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum ComplianceFrameworkStatus {
    NotStarted,
    InProgress,
    Compliant,
    PartiallyCompliant,
    NonCompliant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataInventory {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub data_type: String,
    pub source_system: String,
    pub location: String,
    pub owner_id: Option<Uuid>,
    pub data_classification: DataClassification,
    pub contains_pii: bool,
    pub contains_phi: bool,
    pub contains_pci: bool,
    pub retention_policy_id: Option<Uuid>,
    pub last_accessed: Option<DateTime<Utc>>,
    pub last_reviewed: Option<NaiveDate>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DataClassification {
    Public,
    Internal,
    Confidential,
    Restricted,
    TopSecret,
}

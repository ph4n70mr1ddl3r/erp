use chrono::{DateTime, Utc};
use erp_core::models::{BaseEntity, Money, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
#[derive(PartialEq)]
pub enum GrantStatus {
    Draft,
    Submitted,
    UnderReview,
    Awarded,
    Active,
    Suspended,
    Completed,
    Terminated,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum GrantType {
    Research,
    Project,
    Operating,
    Capital,
    Fellowship,
    Scholarship,
    Capacity,
    Technical,
    Emergency,
    Matching,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum FundingSource {
    Federal,
    State,
    Local,
    Foundation,
    Corporate,
    Individual,
    International,
    NonProfit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grant {
    pub base: BaseEntity,
    pub grant_number: String,
    pub title: String,
    pub description: Option<String>,
    pub grant_type: GrantType,
    pub status: GrantStatus,
    pub funding_source: FundingSource,
    pub funder_name: String,
    pub funder_contact: Option<String>,
    pub total_award_amount: Money,
    pub currency: String,
    pub indirect_cost_rate: f64,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub principal_investigator_id: Uuid,
    pub department_id: Option<Uuid>,
    pub program_id: Option<Uuid>,
    pub cfda_number: Option<String>,
    pub award_number: Option<String>,
    pub is_cost_sharing: bool,
    pub cost_sharing_amount: Option<Money>,
    pub reporting_frequency: ReportingFrequency,
    pub next_report_due: Option<DateTime<Utc>>,
    pub compliance_requirements: Vec<String>,
    pub special_conditions: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReportingFrequency {
    Monthly,
    Quarterly,
    SemiAnnual,
    Annual,
    Final,
    AsRequired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantBudget {
    pub base: BaseEntity,
    pub grant_id: Uuid,
    pub budget_category: BudgetCategory,
    pub description: String,
    pub approved_amount: Money,
    pub budgeted_amount: Money,
    pub expended_amount: Money,
    pub encumbered_amount: Money,
    pub available_balance: Money,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BudgetCategory {
    Personnel,
    FringeBenefits,
    Travel,
    Equipment,
    Supplies,
    Contractual,
    Construction,
    OtherDirect,
    IndirectCosts,
    CostSharing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantPersonnel {
    pub base: BaseEntity,
    pub grant_id: Uuid,
    pub employee_id: Uuid,
    pub role: PersonnelRole,
    pub effort_percent: f64,
    pub hourly_rate: Money,
    pub total_budgeted: Money,
    pub total_charged: Money,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub is_key_personnel: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PersonnelRole {
    PrincipalInvestigator,
    CoInvestigator,
    ProjectDirector,
    SeniorPersonnel,
    PostDoc,
    GraduateStudent,
    UndergraduateStudent,
    Consultant,
    Staff,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantTransaction {
    pub base: BaseEntity,
    pub grant_id: Uuid,
    pub budget_category: BudgetCategory,
    pub transaction_type: TransactionType,
    pub transaction_date: DateTime<Utc>,
    pub amount: Money,
    pub description: String,
    pub reference_number: Option<String>,
    pub invoice_id: Option<Uuid>,
    pub journal_entry_id: Option<Uuid>,
    pub approved_by: Option<Uuid>,
    pub cost_sharing_flag: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TransactionType {
    Expenditure,
    Encumbrance,
    BudgetAdjustment,
    CostTransfer,
    Refund,
    IndirectCost,
    CostSharing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantMilestone {
    pub base: BaseEntity,
    pub grant_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub due_date: DateTime<Utc>,
    pub completed_date: Option<DateTime<Utc>>,
    pub status: MilestoneStatus,
    pub deliverables: Vec<String>,
    pub is_payment_trigger: bool,
    pub payment_amount: Option<Money>,
    pub completed_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MilestoneStatus {
    NotStarted,
    InProgress,
    Completed,
    Overdue,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantReport {
    pub base: BaseEntity,
    pub grant_id: Uuid,
    pub report_type: ReportType,
    pub reporting_period_start: DateTime<Utc>,
    pub reporting_period_end: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub submitted_date: Option<DateTime<Utc>>,
    pub status: ReportStatus,
    pub prepared_by: Option<Uuid>,
    pub approved_by: Option<Uuid>,
    pub notes: Option<String>,
    pub attachment_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReportType {
    Financial,
    Technical,
    Progress,
    Final,
    Audit,
    Equipment,
    Invention,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReportStatus {
    Draft,
    UnderReview,
    Submitted,
    Accepted,
    RevisionRequired,
    Overdue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantCompliance {
    pub base: BaseEntity,
    pub grant_id: Uuid,
    pub requirement_type: ComplianceRequirement,
    pub description: String,
    pub is_mandatory: bool,
    pub due_date: Option<DateTime<Utc>>,
    pub completed_date: Option<DateTime<Utc>>,
    pub status: ComplianceStatus,
    pub responsible_party: Option<Uuid>,
    pub documentation_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ComplianceRequirement {
    IRBApproval,
    IACUCApproval,
    HumanSubjects,
    AnimalSubjects,
    Biosafety,
    ExportControl,
    DataSharing,
    OpenAccess,
    FinancialConflict,
    SubrecipientMonitoring,
    CostPrinciples,
    EffortReporting,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ComplianceStatus {
    NotApplicable,
    Pending,
    InProgress,
    Compliant,
    NonCompliant,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantSubaward {
    pub base: BaseEntity,
    pub grant_id: Uuid,
    pub subrecipient_id: Uuid,
    pub subrecipient_name: String,
    pub subaward_number: String,
    pub total_amount: Money,
    pub disbursed_amount: Money,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub status: Status,
    pub contact_person: Option<String>,
    pub contact_email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantCloseout {
    pub base: BaseEntity,
    pub grant_id: Uuid,
    pub closeout_date: DateTime<Utc>,
    pub final_expenditure: Money,
    pub unexpended_balance: Money,
    pub final_report_submitted: bool,
    pub equipment_inventory_complete: bool,
    pub inventions_reported: bool,
    pub subawards_closed: bool,
    pub status: CloseoutStatus,
    pub closed_by: Option<Uuid>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CloseoutStatus {
    Initiated,
    InProgress,
    PendingReports,
    PendingApprovals,
    Completed,
}

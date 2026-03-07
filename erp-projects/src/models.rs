use chrono::{DateTime, Utc};
use erp_core::BaseEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub base: BaseEntity,
    pub project_number: String,
    pub name: String,
    pub description: Option<String>,
    pub customer_id: Option<Uuid>,
    pub project_type: ProjectType,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub budget: i64,
    pub billable: bool,
    pub billing_method: BillingMethod,
    pub project_manager: Option<Uuid>,
    pub status: ProjectStatus,
    pub percent_complete: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ProjectType {
    Internal,
    External,
    Billable,
    NonBillable,
    Maintenance,
    Development,
    Consulting,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BillingMethod {
    FixedPrice,
    TimeAndMaterials,
    Milestone,
    Retainer,
    Hourly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ProjectStatus {
    Planning,
    Active,
    OnHold,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTask {
    pub base: BaseEntity,
    pub project_id: Uuid,
    pub task_number: i32,
    pub name: String,
    pub description: Option<String>,
    pub parent_task_id: Option<Uuid>,
    pub assigned_to: Option<Uuid>,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub estimated_hours: Option<f64>,
    pub actual_hours: f64,
    pub percent_complete: i32,
    pub priority: TaskPriority,
    pub status: TaskStatus,
    pub billable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TaskStatus {
    NotStarted,
    InProgress,
    Completed,
    OnHold,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMilestone {
    pub base: BaseEntity,
    pub project_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub planned_date: DateTime<Utc>,
    pub actual_date: Option<DateTime<Utc>>,
    pub billing_amount: i64,
    pub billing_status: BillingStatus,
    pub status: MilestoneStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BillingStatus {
    NotBilled,
    PartiallyBilled,
    FullyBilled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MilestoneStatus {
    Planned,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectExpense {
    pub base: BaseEntity,
    pub project_id: Uuid,
    pub expense_type: ExpenseType,
    pub description: String,
    pub amount: i64,
    pub expense_date: DateTime<Utc>,
    pub billable: bool,
    pub invoiced: bool,
    pub invoice_id: Option<Uuid>,
    pub status: ExpenseStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ExpenseType {
    Travel,
    Materials,
    Equipment,
    Software,
    Services,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ExpenseStatus {
    Pending,
    Approved,
    Rejected,
    Invoiced,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timesheet {
    pub base: BaseEntity,
    pub timesheet_number: String,
    pub employee_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_hours: f64,
    pub overtime_hours: f64,
    pub status: TimesheetStatus,
    pub submitted_at: Option<DateTime<Utc>>,
    pub approved_at: Option<DateTime<Utc>>,
    pub approved_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TimesheetStatus {
    Draft,
    Submitted,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimesheetEntry {
    pub base: BaseEntity,
    pub timesheet_id: Uuid,
    pub project_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub entry_date: DateTime<Utc>,
    pub hours: f64,
    pub description: Option<String>,
    pub billable: bool,
    pub hourly_rate: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectBilling {
    pub base: BaseEntity,
    pub billing_number: String,
    pub project_id: Uuid,
    pub billing_type: ProjectBillingType,
    pub milestone_id: Option<Uuid>,
    pub period_start: Option<DateTime<Utc>>,
    pub period_end: Option<DateTime<Utc>>,
    pub amount: i64,
    pub invoice_id: Option<Uuid>,
    pub status: ProjectBillingStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ProjectBillingType {
    Milestone,
    TimeBased,
    Fixed,
    Retainer,
    Progress,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ProjectBillingStatus {
    Draft,
    Invoiced,
    Paid,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub base: BaseEntity,
    pub name: String,
    pub category: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSkill {
    pub base: BaseEntity,
    pub employee_id: Uuid,
    pub skill_id: Uuid,
    pub proficiency_level: i32, // 1-5
    pub years_experience: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ResourceRequestStatus {
    Draft,
    Pending,
    Fulfilled,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequest {
    pub base: BaseEntity,
    pub project_id: Uuid,
    pub task_id: Option<Uuid>,
    pub skill_id: Uuid,
    pub min_proficiency: i32,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub hours_required: f64,
    pub status: ResourceRequestStatus,
    pub requested_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAllocation {
    pub base: BaseEntity,
    pub request_id: Option<Uuid>,
    pub project_id: Uuid,
    pub employee_id: Uuid,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub allocation_percent: i32,
    pub billable_rate: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTemplate {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub project_type: ProjectType,
    pub billing_method: BillingMethod,
    pub billable: bool,
    pub tasks: Vec<ProjectTemplateTask>,
    pub milestones: Vec<ProjectTemplateMilestone>,
    pub status: erp_core::Status,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTemplateTask {
    pub base: BaseEntity,
    pub template_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub relative_start_day: i32,
    pub duration_days: i32,
    pub estimated_hours: Option<f64>,
    pub priority: TaskPriority,
    pub billable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTemplateMilestone {
    pub base: BaseEntity,
    pub template_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub relative_day: i32,
    pub billing_amount: i64,
}


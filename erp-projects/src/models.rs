use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
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
    pub created_at: DateTime<Utc>,
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
    pub id: Uuid,
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
    pub id: Uuid,
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
    pub id: Uuid,
    pub project_id: Uuid,
    pub expense_type: ExpenseType,
    pub description: String,
    pub amount: i64,
    pub expense_date: DateTime<Utc>,
    pub billable: bool,
    pub invoiced: bool,
    pub invoice_id: Option<Uuid>,
    pub status: ExpenseStatus,
    pub created_at: DateTime<Utc>,
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
    pub id: Uuid,
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
    pub created_at: DateTime<Utc>,
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
    pub id: Uuid,
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
    pub id: Uuid,
    pub billing_number: String,
    pub project_id: Uuid,
    pub billing_type: ProjectBillingType,
    pub milestone_id: Option<Uuid>,
    pub period_start: Option<DateTime<Utc>>,
    pub period_end: Option<DateTime<Utc>>,
    pub amount: i64,
    pub invoice_id: Option<Uuid>,
    pub status: ProjectBillingStatus,
    pub created_at: DateTime<Utc>,
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
    Submitted,
    Approved,
    Invoiced,
    Paid,
    Cancelled,
}

use chrono::{DateTime, NaiveDate, Utc};
use erp_core::{Address, BaseEntity, ContactInfo, Money, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub base: BaseEntity,
    pub employee_number: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub contact: ContactInfo,
    pub address: Address,
    pub birth_date: NaiveDate,
    pub hire_date: NaiveDate,
    pub termination_date: Option<NaiveDate>,
    pub department_id: Option<Uuid>,
    pub position_id: Option<Uuid>,
    pub manager_id: Option<Uuid>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Department {
    pub base: BaseEntity,
    pub code: String,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub manager_id: Option<Uuid>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub base: BaseEntity,
    pub code: String,
    pub title: String,
    pub department_id: Option<Uuid>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attendance {
    pub base: BaseEntity,
    pub employee_id: Uuid,
    pub date: NaiveDate,
    pub check_in: Option<DateTime<Utc>>,
    pub check_out: Option<DateTime<Utc>>,
    pub work_hours: f64,
    pub overtime_hours: f64,
    pub status: AttendanceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AttendanceStatus {
    Present,
    Absent,
    Late,
    HalfDay,
    Leave,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveRequest {
    pub base: BaseEntity,
    pub employee_id: Uuid,
    pub leave_type: LeaveType,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub days: f64,
    pub reason: Option<String>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum LeaveType {
    Annual,
    Sick,
    Personal,
    Maternity,
    Paternity,
    Unpaid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payroll {
    pub base: BaseEntity,
    pub employee_id: Uuid,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub base_salary: Money,
    pub overtime: Money,
    pub bonuses: Money,
    pub deductions: Money,
    pub net_salary: Money,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryStructure {
    pub base: BaseEntity,
    pub employee_id: Uuid,
    pub base_salary: Money,
    pub allowances: Vec<Allowance>,
    pub effective_date: NaiveDate,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Allowance {
    pub id: Uuid,
    pub name: String,
    pub amount: Money,
    pub taxable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveTypeDef {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub days_per_year: i64,
    pub carry_over: bool,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveBalance {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub leave_type_id: Uuid,
    pub year: i32,
    pub entitled: i64,
    pub used: i64,
    pub remaining: i64,
    pub carried_over: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveRequestExtended {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub leave_type_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub days: i64,
    pub reason: Option<String>,
    pub status: LeaveRequestStatus,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub rejection_reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum LeaveRequestStatus {
    Pending,
    Approved,
    Rejected,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseCategory {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseReport {
    pub id: Uuid,
    pub report_number: String,
    pub employee_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub total_amount: i64,
    pub status: ExpenseReportStatus,
    pub submitted_at: Option<DateTime<Utc>>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub rejected_at: Option<DateTime<Utc>>,
    pub rejection_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum ExpenseReportStatus {
    Draft,
    Submitted,
    Approved,
    Rejected,
    Paid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseLine {
    pub id: Uuid,
    pub expense_report_id: Uuid,
    pub category_id: Uuid,
    pub expense_date: NaiveDate,
    pub description: String,
    pub amount: i64,
    pub currency: String,
    pub receipt_path: Option<String>,
}

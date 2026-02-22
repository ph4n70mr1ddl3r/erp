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

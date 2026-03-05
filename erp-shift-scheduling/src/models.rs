use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use erp_core::BaseEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ShiftStatus {
    Draft,
    Active,
    Inactive,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AssignmentStatus {
    Scheduled,
    Confirmed,
    InProgress,
    Completed,
    Absent,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shift {
    pub base: BaseEntity,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub break_minutes: i32,
    pub grace_period_minutes: i32,
    pub color_code: Option<String>,
    pub status: ShiftStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub base: BaseEntity,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub department_id: Option<Uuid>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub status: ScheduleStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ScheduleStatus {
    Draft,
    Published,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiftAssignment {
    pub id: Uuid,
    pub schedule_id: Uuid,
    pub shift_id: Uuid,
    pub employee_id: Uuid,
    pub assignment_date: NaiveDate,
    pub actual_start_time: Option<DateTime<Utc>>,
    pub actual_end_time: Option<DateTime<Utc>>,
    pub overtime_minutes: i32,
    pub notes: Option<String>,
    pub status: AssignmentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleEntry {
    pub id: Uuid,
    pub schedule_id: Uuid,
    pub day_of_week: i32,
    pub shift_id: Uuid,
    pub employee_id: Uuid,
    pub is_recurring: bool,
    pub effective_from: Option<NaiveDate>,
    pub effective_to: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiftSwapRequest {
    pub id: Uuid,
    pub schedule_id: Uuid,
    pub requesting_employee_id: Uuid,
    pub target_employee_id: Uuid,
    pub request_date: NaiveDate,
    pub reason: Option<String>,
    pub status: SwapRequestStatus,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SwapRequestStatus {
    Pending,
    Approved,
    Rejected,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleSummary {
    pub schedule_id: Uuid,
    pub schedule_name: String,
    pub total_shifts: i64,
    pub total_assignments: i64,
    pub employees_scheduled: i64,
    pub pending_swaps: i64,
    pub coverage_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyScheduleView {
    pub date: NaiveDate,
    pub assignments: Vec<ShiftAssignmentView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiftAssignmentView {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub employee_name: String,
    pub shift_id: Uuid,
    pub shift_name: String,
    pub shift_code: String,
    pub start_time: String,
    pub end_time: String,
    pub status: String,
    pub notes: Option<String>,
}

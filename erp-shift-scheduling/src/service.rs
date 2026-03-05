use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use erp_core::{BaseEntity, Error, Paginated, Pagination, Result};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;
use crate::repository::*;

pub struct ShiftSchedulingService {
    shift_repo: SqliteShiftRepository,
    schedule_repo: SqliteScheduleRepository,
    assignment_repo: SqliteShiftAssignmentRepository,
}

impl Default for ShiftSchedulingService {
    fn default() -> Self {
        Self::new()
    }
}

impl ShiftSchedulingService {
    pub fn new() -> Self {
        Self {
            shift_repo: SqliteShiftRepository,
            schedule_repo: SqliteScheduleRepository,
            assignment_repo: SqliteShiftAssignmentRepository,
        }
    }

    pub async fn get_shift(&self, pool: &SqlitePool, id: Uuid) -> Result<Shift> {
        self.shift_repo.find_by_id(pool, id).await
    }

    pub async fn list_shifts(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Shift>> {
        self.shift_repo.find_all(pool, pagination).await
    }

    pub async fn list_active_shifts(&self, pool: &SqlitePool) -> Result<Vec<Shift>> {
        self.shift_repo.find_active(pool).await
    }

    pub async fn create_shift(&self, pool: &SqlitePool, req: CreateShiftRequest) -> Result<Shift> {
        if req.code.is_empty() {
            return Err(Error::validation("Shift code is required"));
        }
        if req.name.is_empty() {
            return Err(Error::validation("Shift name is required"));
        }

        let shift = Shift {
            base: BaseEntity::new(),
            code: req.code,
            name: req.name,
            description: req.description,
            start_time: req.start_time,
            end_time: req.end_time,
            break_minutes: req.break_minutes.unwrap_or(30),
            grace_period_minutes: req.grace_period_minutes.unwrap_or(15),
            color_code: req.color_code,
            status: ShiftStatus::Active,
        };

        self.shift_repo.create(pool, shift).await
    }

    pub async fn update_shift(&self, pool: &SqlitePool, id: Uuid, req: UpdateShiftRequest) -> Result<Shift> {
        let mut shift = self.shift_repo.find_by_id(pool, id).await?;

        if let Some(name) = req.name { shift.name = name; }
        if let Some(description) = req.description { shift.description = Some(description); }
        if let Some(start_time) = req.start_time { shift.start_time = start_time; }
        if let Some(end_time) = req.end_time { shift.end_time = end_time; }
        if let Some(break_minutes) = req.break_minutes { shift.break_minutes = break_minutes; }
        if let Some(grace_period_minutes) = req.grace_period_minutes { shift.grace_period_minutes = grace_period_minutes; }
        if let Some(color_code) = req.color_code { shift.color_code = Some(color_code); }
        if let Some(status) = req.status { shift.status = status; }

        self.shift_repo.update(pool, shift).await
    }

    pub async fn delete_shift(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.shift_repo.delete(pool, id).await
    }

    pub async fn get_schedule(&self, pool: &SqlitePool, id: Uuid) -> Result<Schedule> {
        self.schedule_repo.find_by_id(pool, id).await
    }

    pub async fn list_schedules(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Schedule>> {
        self.schedule_repo.find_all(pool, pagination).await
    }

    pub async fn create_schedule(&self, pool: &SqlitePool, req: CreateScheduleRequest) -> Result<Schedule> {
        if req.code.is_empty() {
            return Err(Error::validation("Schedule code is required"));
        }
        if req.name.is_empty() {
            return Err(Error::validation("Schedule name is required"));
        }
        if req.end_date < req.start_date {
            return Err(Error::validation("End date must be after start date"));
        }

        let schedule = Schedule {
            base: BaseEntity::new(),
            code: req.code,
            name: req.name,
            description: req.description,
            department_id: req.department_id,
            start_date: req.start_date,
            end_date: req.end_date,
            status: ScheduleStatus::Draft,
        };

        self.schedule_repo.create(pool, schedule).await
    }

    pub async fn update_schedule(&self, pool: &SqlitePool, id: Uuid, req: UpdateScheduleRequest) -> Result<Schedule> {
        let mut schedule = self.schedule_repo.find_by_id(pool, id).await?;

        if let Some(name) = req.name { schedule.name = name; }
        if let Some(description) = req.description { schedule.description = Some(description); }
        if let Some(department_id) = req.department_id { schedule.department_id = Some(department_id); }
        if let Some(start_date) = req.start_date { schedule.start_date = start_date; }
        if let Some(end_date) = req.end_date { schedule.end_date = end_date; }
        if let Some(status) = req.status { schedule.status = status; }

        self.schedule_repo.update(pool, schedule).await
    }

    pub async fn delete_schedule(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.schedule_repo.delete(pool, id).await
    }

    pub async fn publish_schedule(&self, pool: &SqlitePool, id: Uuid) -> Result<Schedule> {
        let mut schedule = self.schedule_repo.find_by_id(pool, id).await?;
        schedule.status = ScheduleStatus::Published;
        self.schedule_repo.update(pool, schedule).await
    }

    pub async fn get_assignment(&self, pool: &SqlitePool, id: Uuid) -> Result<ShiftAssignment> {
        self.assignment_repo.find_by_id(pool, id).await
    }

    pub async fn list_schedule_assignments(&self, pool: &SqlitePool, schedule_id: Uuid) -> Result<Vec<ShiftAssignment>> {
        self.assignment_repo.find_by_schedule(pool, schedule_id).await
    }

    pub async fn list_employee_assignments(&self, pool: &SqlitePool, employee_id: Uuid, from: NaiveDate, to: NaiveDate) -> Result<Vec<ShiftAssignment>> {
        self.assignment_repo.find_by_employee(pool, employee_id, from, to).await
    }

    pub async fn create_assignment(&self, pool: &SqlitePool, req: CreateAssignmentRequest) -> Result<ShiftAssignment> {
        let schedule = self.schedule_repo.find_by_id(pool, req.schedule_id).await?;
        if schedule.end_date < req.assignment_date || schedule.start_date > req.assignment_date {
            return Err(Error::validation("Assignment date must be within schedule period"));
        }

        let assignment = ShiftAssignment {
            id: Uuid::new_v4(),
            schedule_id: req.schedule_id,
            shift_id: req.shift_id,
            employee_id: req.employee_id,
            assignment_date: req.assignment_date,
            actual_start_time: None,
            actual_end_time: None,
            overtime_minutes: 0,
            notes: req.notes,
            status: AssignmentStatus::Scheduled,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.assignment_repo.create(pool, assignment).await
    }

    pub async fn update_assignment(&self, pool: &SqlitePool, id: Uuid, req: UpdateAssignmentRequest) -> Result<ShiftAssignment> {
        let mut assignment = self.assignment_repo.find_by_id(pool, id).await?;

        if let Some(notes) = req.notes { assignment.notes = Some(notes); }
        if let Some(status) = req.status { assignment.status = status; }
        if let Some(actual_start_time) = req.actual_start_time { assignment.actual_start_time = Some(actual_start_time); }
        if let Some(actual_end_time) = req.actual_end_time { assignment.actual_end_time = Some(actual_end_time); }
        if let Some(overtime_minutes) = req.overtime_minutes { assignment.overtime_minutes = overtime_minutes; }

        self.assignment_repo.update(pool, assignment).await
    }

    pub async fn delete_assignment(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.assignment_repo.delete(pool, id).await
    }

    pub async fn clock_in(&self, pool: &SqlitePool, id: Uuid) -> Result<ShiftAssignment> {
        let mut assignment = self.assignment_repo.find_by_id(pool, id).await?;
        
        if assignment.status != AssignmentStatus::Scheduled && assignment.status != AssignmentStatus::Confirmed {
            return Err(Error::business_rule("Assignment is not in a clockable state"));
        }

        assignment.actual_start_time = Some(Utc::now());
        assignment.status = AssignmentStatus::InProgress;
        
        self.assignment_repo.update(pool, assignment).await
    }

    pub async fn clock_out(&self, pool: &SqlitePool, id: Uuid) -> Result<ShiftAssignment> {
        let mut assignment = self.assignment_repo.find_by_id(pool, id).await?;
        
        if assignment.status != AssignmentStatus::InProgress {
            return Err(Error::business_rule("Assignment is not in progress"));
        }

        assignment.actual_end_time = Some(Utc::now());
        assignment.status = AssignmentStatus::Completed;
        
        self.assignment_repo.update(pool, assignment).await
    }

    pub async fn get_daily_schedule(&self, pool: &SqlitePool, schedule_id: Uuid, date: NaiveDate) -> Result<Vec<ShiftAssignment>> {
        self.assignment_repo.find_by_date(pool, schedule_id, date).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShiftRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub break_minutes: Option<i32>,
    pub grace_period_minutes: Option<i32>,
    pub color_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateShiftRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
    pub break_minutes: Option<i32>,
    pub grace_period_minutes: Option<i32>,
    pub color_code: Option<String>,
    pub status: Option<ShiftStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateScheduleRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub department_id: Option<Uuid>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateScheduleRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub department_id: Option<Uuid>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub status: Option<ScheduleStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAssignmentRequest {
    pub schedule_id: Uuid,
    pub shift_id: Uuid,
    pub employee_id: Uuid,
    pub assignment_date: NaiveDate,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAssignmentRequest {
    pub notes: Option<String>,
    pub status: Option<AssignmentStatus>,
    pub actual_start_time: Option<DateTime<Utc>>,
    pub actual_end_time: Option<DateTime<Utc>>,
    pub overtime_minutes: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ShiftStatus, ScheduleStatus, AssignmentStatus};
    use chrono::{NaiveDate, NaiveTime};

    #[test]
    fn test_create_shift_request_validation() {
        let req = CreateShiftRequest {
            code: "MORNING".to_string(),
            name: "Morning Shift".to_string(),
            description: Some("8 AM to 4 PM shift".to_string()),
            start_time: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            end_time: NaiveTime::from_hms_opt(16, 0, 0).unwrap(),
            break_minutes: Some(30),
            grace_period_minutes: Some(15),
            color_code: Some("#3B82F6".to_string()),
        };
        assert_eq!(req.code, "MORNING");
        assert_eq!(req.name, "Morning Shift");
        assert_eq!(req.start_time, NaiveTime::from_hms_opt(8, 0, 0).unwrap());
        assert_eq!(req.end_time, NaiveTime::from_hms_opt(16, 0, 0).unwrap());
    }

    #[test]
    fn test_create_schedule_request_validation() {
        let req = CreateScheduleRequest {
            code: "WEEK-2024-01".to_string(),
            name: "Week 1 January 2024".to_string(),
            description: Some("First week of January".to_string()),
            department_id: None,
            start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2024, 1, 7).unwrap(),
        };
        assert_eq!(req.code, "WEEK-2024-01");
        assert_eq!(req.name, "Week 1 January 2024");
        assert_eq!(req.start_date, NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
        assert_eq!(req.end_date, NaiveDate::from_ymd_opt(2024, 1, 7).unwrap());
    }

    #[test]
    fn test_shift_status_values() {
        assert_eq!(format!("{:?}", ShiftStatus::Draft), "Draft");
        assert_eq!(format!("{:?}", ShiftStatus::Active), "Active");
        assert_eq!(format!("{:?}", ShiftStatus::Inactive), "Inactive");
    }

    #[test]
    fn test_schedule_status_values() {
        assert_eq!(format!("{:?}", ScheduleStatus::Draft), "Draft");
        assert_eq!(format!("{:?}", ScheduleStatus::Published), "Published");
        assert_eq!(format!("{:?}", ScheduleStatus::Archived), "Archived");
    }

    #[test]
    fn test_assignment_status_values() {
        assert_eq!(format!("{:?}", AssignmentStatus::Scheduled), "Scheduled");
        assert_eq!(format!("{:?}", AssignmentStatus::Confirmed), "Confirmed");
        assert_eq!(format!("{:?}", AssignmentStatus::InProgress), "InProgress");
        assert_eq!(format!("{:?}", AssignmentStatus::Completed), "Completed");
        assert_eq!(format!("{:?}", AssignmentStatus::Absent), "Absent");
        assert_eq!(format!("{:?}", AssignmentStatus::Cancelled), "Cancelled");
    }

    #[test]
    fn test_create_assignment_request_validation() {
        let req = CreateAssignmentRequest {
            schedule_id: uuid::Uuid::new_v4(),
            shift_id: uuid::Uuid::new_v4(),
            employee_id: uuid::Uuid::new_v4(),
            assignment_date: NaiveDate::from_ymd_opt(2024, 1, 3).unwrap(),
            notes: Some("Holiday coverage".to_string()),
        };
        assert!(req.notes.is_some());
        assert_eq!(req.assignment_date, NaiveDate::from_ymd_opt(2024, 1, 3).unwrap());
    }
}

use async_trait::async_trait;
use chrono::{NaiveDate, NaiveTime, Utc};
use erp_core::{BaseEntity, Error, Paginated, Pagination, Result};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;

pub struct SqliteShiftRepository;

#[derive(sqlx::FromRow)]
struct ShiftRow {
    id: String,
    code: String,
    name: String,
    description: Option<String>,
    start_time: String,
    end_time: String,
    break_minutes: i32,
    grace_period_minutes: i32,
    color_code: Option<String>,
    status: String,
    created_at: String,
    updated_at: String,
}

#[async_trait]
pub trait ShiftRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Shift>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Shift>>;
    async fn find_active(&self, pool: &SqlitePool) -> Result<Vec<Shift>>;
    async fn create(&self, pool: &SqlitePool, shift: Shift) -> Result<Shift>;
    async fn update(&self, pool: &SqlitePool, shift: Shift) -> Result<Shift>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

#[async_trait]
impl ShiftRepository for SqliteShiftRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Shift> {
        let row = sqlx::query_as::<_, ShiftRow>(
            "SELECT id, code, name, description, start_time, end_time, break_minutes,
             grace_period_minutes, color_code, status, created_at, updated_at
             FROM shifts WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("Shift", &id.to_string()))?;

        Ok(row_to_shift(row))
    }

    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Shift>> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM shifts")
            .fetch_one(pool)
            .await?;

        let rows = sqlx::query_as::<_, ShiftRow>(
            "SELECT id, code, name, description, start_time, end_time, break_minutes,
             grace_period_minutes, color_code, status, created_at, updated_at
             FROM shifts ORDER BY start_time LIMIT ? OFFSET ?"
        )
        .bind(pagination.limit() as i64)
        .bind(pagination.offset() as i64)
        .fetch_all(pool)
        .await?;

        Ok(Paginated::new(
            rows.into_iter().map(row_to_shift).collect(),
            count.0 as u64,
            pagination,
        ))
    }

    async fn find_active(&self, pool: &SqlitePool) -> Result<Vec<Shift>> {
        let rows = sqlx::query_as::<_, ShiftRow>(
            "SELECT id, code, name, description, start_time, end_time, break_minutes,
             grace_period_minutes, color_code, status, created_at, updated_at
             FROM shifts WHERE status = 'Active' ORDER BY start_time"
        )
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(row_to_shift).collect())
    }

    async fn create(&self, pool: &SqlitePool, shift: Shift) -> Result<Shift> {
        let now = Utc::now();
        sqlx::query(
            "INSERT INTO shifts (id, code, name, description, start_time, end_time,
             break_minutes, grace_period_minutes, color_code, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(shift.base.id.to_string())
        .bind(&shift.code)
        .bind(&shift.name)
        .bind(&shift.description)
        .bind(shift.start_time.format("%H:%M").to_string())
        .bind(shift.end_time.format("%H:%M").to_string())
        .bind(shift.break_minutes)
        .bind(shift.grace_period_minutes)
        .bind(&shift.color_code)
        .bind(format!("{:?}", shift.status))
        .bind(shift.base.created_at.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(shift)
    }

    async fn update(&self, pool: &SqlitePool, shift: Shift) -> Result<Shift> {
        let now = Utc::now();
        let rows = sqlx::query(
            "UPDATE shifts SET code=?, name=?, description=?, start_time=?, end_time=?,
             break_minutes=?, grace_period_minutes=?, color_code=?, status=?, updated_at=? WHERE id=?"
        )
        .bind(&shift.code)
        .bind(&shift.name)
        .bind(&shift.description)
        .bind(shift.start_time.format("%H:%M").to_string())
        .bind(shift.end_time.format("%H:%M").to_string())
        .bind(shift.break_minutes)
        .bind(shift.grace_period_minutes)
        .bind(&shift.color_code)
        .bind(format!("{:?}", shift.status))
        .bind(now.to_rfc3339())
        .bind(shift.base.id.to_string())
        .execute(pool)
        .await?;

        if rows.rows_affected() == 0 {
            return Err(Error::not_found("Shift", &shift.base.id.to_string()));
        }

        Ok(shift)
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let rows = sqlx::query("DELETE FROM shifts WHERE id = ?")
            .bind(id.to_string())
            .execute(pool)
            .await?;

        if rows.rows_affected() == 0 {
            return Err(Error::not_found("Shift", &id.to_string()));
        }

        Ok(())
    }
}

fn row_to_shift(row: ShiftRow) -> Shift {
    Shift {
        base: BaseEntity {
            id: Uuid::parse_str(&row.id).unwrap_or_default(),
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            created_by: None,
            updated_by: None,
        },
        code: row.code,
        name: row.name,
        description: row.description,
        start_time: NaiveTime::parse_from_str(&row.start_time, "%H:%M").unwrap_or_default(),
        end_time: NaiveTime::parse_from_str(&row.end_time, "%H:%M").unwrap_or_default(),
        break_minutes: row.break_minutes,
        grace_period_minutes: row.grace_period_minutes,
        color_code: row.color_code,
        status: match row.status.as_str() {
            "Draft" => ShiftStatus::Draft,
            "Inactive" => ShiftStatus::Inactive,
            _ => ShiftStatus::Active,
        },
    }
}

pub struct SqliteScheduleRepository;

#[derive(sqlx::FromRow)]
struct ScheduleRow {
    id: String,
    code: String,
    name: String,
    description: Option<String>,
    department_id: Option<String>,
    start_date: String,
    end_date: String,
    status: String,
    created_at: String,
    updated_at: String,
}

#[async_trait]
pub trait ScheduleRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Schedule>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Schedule>>;
    async fn create(&self, pool: &SqlitePool, schedule: Schedule) -> Result<Schedule>;
    async fn update(&self, pool: &SqlitePool, schedule: Schedule) -> Result<Schedule>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

#[async_trait]
impl ScheduleRepository for SqliteScheduleRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Schedule> {
        let row = sqlx::query_as::<_, ScheduleRow>(
            "SELECT id, code, name, description, department_id, start_date, end_date,
             status, created_at, updated_at FROM schedules WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("Schedule", &id.to_string()))?;

        Ok(row_to_schedule(row))
    }

    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Schedule>> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM schedules")
            .fetch_one(pool)
            .await?;

        let rows = sqlx::query_as::<_, ScheduleRow>(
            "SELECT id, code, name, description, department_id, start_date, end_date,
             status, created_at, updated_at FROM schedules ORDER BY start_date DESC LIMIT ? OFFSET ?"
        )
        .bind(pagination.limit() as i64)
        .bind(pagination.offset() as i64)
        .fetch_all(pool)
        .await?;

        Ok(Paginated::new(
            rows.into_iter().map(row_to_schedule).collect(),
            count.0 as u64,
            pagination,
        ))
    }

    async fn create(&self, pool: &SqlitePool, schedule: Schedule) -> Result<Schedule> {
        let now = Utc::now();
        sqlx::query(
            "INSERT INTO schedules (id, code, name, description, department_id, start_date,
             end_date, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(schedule.base.id.to_string())
        .bind(&schedule.code)
        .bind(&schedule.name)
        .bind(&schedule.description)
        .bind(schedule.department_id.map(|id| id.to_string()))
        .bind(schedule.start_date.to_string())
        .bind(schedule.end_date.to_string())
        .bind(format!("{:?}", schedule.status))
        .bind(schedule.base.created_at.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(schedule)
    }

    async fn update(&self, pool: &SqlitePool, schedule: Schedule) -> Result<Schedule> {
        let now = Utc::now();
        let rows = sqlx::query(
            "UPDATE schedules SET code=?, name=?, description=?, department_id=?,
             start_date=?, end_date=?, status=?, updated_at=? WHERE id=?"
        )
        .bind(&schedule.code)
        .bind(&schedule.name)
        .bind(&schedule.description)
        .bind(schedule.department_id.map(|id| id.to_string()))
        .bind(schedule.start_date.to_string())
        .bind(schedule.end_date.to_string())
        .bind(format!("{:?}", schedule.status))
        .bind(now.to_rfc3339())
        .bind(schedule.base.id.to_string())
        .execute(pool)
        .await?;

        if rows.rows_affected() == 0 {
            return Err(Error::not_found("Schedule", &schedule.base.id.to_string()));
        }

        Ok(schedule)
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let mut tx = pool.begin().await?;
        
        sqlx::query("DELETE FROM shift_assignments WHERE schedule_id = ?")
            .bind(id.to_string())
            .execute(&mut *tx)
            .await?;
        
        sqlx::query("DELETE FROM schedule_entries WHERE schedule_id = ?")
            .bind(id.to_string())
            .execute(&mut *tx)
            .await?;
        
        let rows = sqlx::query("DELETE FROM schedules WHERE id = ?")
            .bind(id.to_string())
            .execute(&mut *tx)
            .await?;

        if rows.rows_affected() == 0 {
            return Err(Error::not_found("Schedule", &id.to_string()));
        }

        tx.commit().await?;
        Ok(())
    }
}

fn row_to_schedule(row: ScheduleRow) -> Schedule {
    Schedule {
        base: BaseEntity {
            id: Uuid::parse_str(&row.id).unwrap_or_default(),
            created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            created_by: None,
            updated_by: None,
        },
        code: row.code,
        name: row.name,
        description: row.description,
        department_id: row.department_id.and_then(|id| Uuid::parse_str(&id).ok()),
        start_date: NaiveDate::parse_from_str(&row.start_date, "%Y-%m-%d").unwrap_or_default(),
        end_date: NaiveDate::parse_from_str(&row.end_date, "%Y-%m-%d").unwrap_or_default(),
        status: match row.status.as_str() {
            "Draft" => ScheduleStatus::Draft,
            "Archived" => ScheduleStatus::Archived,
            _ => ScheduleStatus::Published,
        },
    }
}

pub struct SqliteShiftAssignmentRepository;

#[derive(sqlx::FromRow)]
struct AssignmentRow {
    id: String,
    schedule_id: String,
    shift_id: String,
    employee_id: String,
    assignment_date: String,
    actual_start_time: Option<String>,
    actual_end_time: Option<String>,
    overtime_minutes: i32,
    notes: Option<String>,
    status: String,
    created_at: String,
    updated_at: String,
}

#[async_trait]
pub trait ShiftAssignmentRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ShiftAssignment>;
    async fn find_by_schedule(&self, pool: &SqlitePool, schedule_id: Uuid) -> Result<Vec<ShiftAssignment>>;
    async fn find_by_employee(&self, pool: &SqlitePool, employee_id: Uuid, from: NaiveDate, to: NaiveDate) -> Result<Vec<ShiftAssignment>>;
    async fn find_by_date(&self, pool: &SqlitePool, schedule_id: Uuid, date: NaiveDate) -> Result<Vec<ShiftAssignment>>;
    async fn create(&self, pool: &SqlitePool, assignment: ShiftAssignment) -> Result<ShiftAssignment>;
    async fn update(&self, pool: &SqlitePool, assignment: ShiftAssignment) -> Result<ShiftAssignment>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

#[async_trait]
impl ShiftAssignmentRepository for SqliteShiftAssignmentRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ShiftAssignment> {
        let row = sqlx::query_as::<_, AssignmentRow>(
            "SELECT id, schedule_id, shift_id, employee_id, assignment_date, actual_start_time,
             actual_end_time, overtime_minutes, notes, status, created_at, updated_at
             FROM shift_assignments WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("ShiftAssignment", &id.to_string()))?;

        Ok(row_to_assignment(row))
    }

    async fn find_by_schedule(&self, pool: &SqlitePool, schedule_id: Uuid) -> Result<Vec<ShiftAssignment>> {
        let rows = sqlx::query_as::<_, AssignmentRow>(
            "SELECT id, schedule_id, shift_id, employee_id, assignment_date, actual_start_time,
             actual_end_time, overtime_minutes, notes, status, created_at, updated_at
             FROM shift_assignments WHERE schedule_id = ? ORDER BY assignment_date, shift_id"
        )
        .bind(schedule_id.to_string())
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(row_to_assignment).collect())
    }

    async fn find_by_employee(&self, pool: &SqlitePool, employee_id: Uuid, from: NaiveDate, to: NaiveDate) -> Result<Vec<ShiftAssignment>> {
        let rows = sqlx::query_as::<_, AssignmentRow>(
            "SELECT id, schedule_id, shift_id, employee_id, assignment_date, actual_start_time,
             actual_end_time, overtime_minutes, notes, status, created_at, updated_at
             FROM shift_assignments WHERE employee_id = ? AND assignment_date >= ? AND assignment_date <= ?
             ORDER BY assignment_date"
        )
        .bind(employee_id.to_string())
        .bind(from.to_string())
        .bind(to.to_string())
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(row_to_assignment).collect())
    }

    async fn find_by_date(&self, pool: &SqlitePool, schedule_id: Uuid, date: NaiveDate) -> Result<Vec<ShiftAssignment>> {
        let rows = sqlx::query_as::<_, AssignmentRow>(
            "SELECT id, schedule_id, shift_id, employee_id, assignment_date, actual_start_time,
             actual_end_time, overtime_minutes, notes, status, created_at, updated_at
             FROM shift_assignments WHERE schedule_id = ? AND assignment_date = ?"
        )
        .bind(schedule_id.to_string())
        .bind(date.to_string())
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(row_to_assignment).collect())
    }

    async fn create(&self, pool: &SqlitePool, assignment: ShiftAssignment) -> Result<ShiftAssignment> {
        let now = Utc::now();
        sqlx::query(
            "INSERT INTO shift_assignments (id, schedule_id, shift_id, employee_id, assignment_date,
             actual_start_time, actual_end_time, overtime_minutes, notes, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(assignment.id.to_string())
        .bind(assignment.schedule_id.to_string())
        .bind(assignment.shift_id.to_string())
        .bind(assignment.employee_id.to_string())
        .bind(assignment.assignment_date.to_string())
        .bind(assignment.actual_start_time.map(|t| t.to_rfc3339()))
        .bind(assignment.actual_end_time.map(|t| t.to_rfc3339()))
        .bind(assignment.overtime_minutes)
        .bind(&assignment.notes)
        .bind(format!("{:?}", assignment.status))
        .bind(assignment.created_at.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(assignment)
    }

    async fn update(&self, pool: &SqlitePool, assignment: ShiftAssignment) -> Result<ShiftAssignment> {
        let now = Utc::now();
        let rows = sqlx::query(
            "UPDATE shift_assignments SET actual_start_time=?, actual_end_time=?,
             overtime_minutes=?, notes=?, status=?, updated_at=? WHERE id=?"
        )
        .bind(assignment.actual_start_time.map(|t| t.to_rfc3339()))
        .bind(assignment.actual_end_time.map(|t| t.to_rfc3339()))
        .bind(assignment.overtime_minutes)
        .bind(&assignment.notes)
        .bind(format!("{:?}", assignment.status))
        .bind(now.to_rfc3339())
        .bind(assignment.id.to_string())
        .execute(pool)
        .await?;

        if rows.rows_affected() == 0 {
            return Err(Error::not_found("ShiftAssignment", &assignment.id.to_string()));
        }

        Ok(assignment)
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let rows = sqlx::query("DELETE FROM shift_assignments WHERE id = ?")
            .bind(id.to_string())
            .execute(pool)
            .await?;

        if rows.rows_affected() == 0 {
            return Err(Error::not_found("ShiftAssignment", &id.to_string()));
        }

        Ok(())
    }
}

fn row_to_assignment(row: AssignmentRow) -> ShiftAssignment {
    ShiftAssignment {
        id: Uuid::parse_str(&row.id).unwrap_or_default(),
        schedule_id: Uuid::parse_str(&row.schedule_id).unwrap_or_default(),
        shift_id: Uuid::parse_str(&row.shift_id).unwrap_or_default(),
        employee_id: Uuid::parse_str(&row.employee_id).unwrap_or_default(),
        assignment_date: NaiveDate::parse_from_str(&row.assignment_date, "%Y-%m-%d").unwrap_or_default(),
        actual_start_time: row.actual_start_time.and_then(|t| chrono::DateTime::parse_from_rfc3339(&t).ok())
            .map(|d| d.with_timezone(&Utc)),
        actual_end_time: row.actual_end_time.and_then(|t| chrono::DateTime::parse_from_rfc3339(&t).ok())
            .map(|d| d.with_timezone(&Utc)),
        overtime_minutes: row.overtime_minutes,
        notes: row.notes,
        status: match row.status.as_str() {
            "Confirmed" => AssignmentStatus::Confirmed,
            "InProgress" => AssignmentStatus::InProgress,
            "Completed" => AssignmentStatus::Completed,
            "Absent" => AssignmentStatus::Absent,
            "Cancelled" => AssignmentStatus::Cancelled,
            _ => AssignmentStatus::Scheduled,
        },
        created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
            .map(|d| d.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now()),
        updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at)
            .map(|d| d.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now()),
    }
}

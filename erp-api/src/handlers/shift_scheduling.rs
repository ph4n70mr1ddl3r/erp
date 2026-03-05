use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::Pagination;
use erp_shift_scheduling::{
    Shift, Schedule, ShiftAssignment, ShiftStatus, ScheduleStatus, AssignmentStatus,
    ShiftSchedulingService, CreateShiftRequest, UpdateShiftRequest,
    CreateScheduleRequest, UpdateScheduleRequest,
    CreateAssignmentRequest, UpdateAssignmentRequest,
};

#[derive(Debug, Serialize)]
pub struct ShiftResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub start_time: String,
    pub end_time: String,
    pub break_minutes: i32,
    pub grace_period_minutes: i32,
    pub color_code: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Shift> for ShiftResponse {
    fn from(s: Shift) -> Self {
        Self {
            id: s.base.id,
            code: s.code,
            name: s.name,
            description: s.description,
            start_time: s.start_time.format("%H:%M").to_string(),
            end_time: s.end_time.format("%H:%M").to_string(),
            break_minutes: s.break_minutes,
            grace_period_minutes: s.grace_period_minutes,
            color_code: s.color_code,
            status: format!("{:?}", s.status),
            created_at: s.base.created_at.to_rfc3339(),
            updated_at: s.base.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ScheduleResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub department_id: Option<Uuid>,
    pub start_date: String,
    pub end_date: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Schedule> for ScheduleResponse {
    fn from(s: Schedule) -> Self {
        Self {
            id: s.base.id,
            code: s.code,
            name: s.name,
            description: s.description,
            department_id: s.department_id,
            start_date: s.start_date.to_string(),
            end_date: s.end_date.to_string(),
            status: format!("{:?}", s.status),
            created_at: s.base.created_at.to_rfc3339(),
            updated_at: s.base.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AssignmentResponse {
    pub id: Uuid,
    pub schedule_id: Uuid,
    pub shift_id: Uuid,
    pub employee_id: Uuid,
    pub assignment_date: String,
    pub actual_start_time: Option<String>,
    pub actual_end_time: Option<String>,
    pub overtime_minutes: i32,
    pub notes: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<ShiftAssignment> for AssignmentResponse {
    fn from(a: ShiftAssignment) -> Self {
        Self {
            id: a.id,
            schedule_id: a.schedule_id,
            shift_id: a.shift_id,
            employee_id: a.employee_id,
            assignment_date: a.assignment_date.to_string(),
            actual_start_time: a.actual_start_time.map(|t| t.to_rfc3339()),
            actual_end_time: a.actual_end_time.map(|t| t.to_rfc3339()),
            overtime_minutes: a.overtime_minutes,
            notes: a.notes,
            status: format!("{:?}", a.status),
            created_at: a.created_at.to_rfc3339(),
            updated_at: a.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateShiftHandlerRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub start_time: String,
    pub end_time: String,
    pub break_minutes: Option<i32>,
    pub grace_period_minutes: Option<i32>,
    pub color_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateShiftHandlerRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub break_minutes: Option<i32>,
    pub grace_period_minutes: Option<i32>,
    pub color_code: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateScheduleHandlerRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub department_id: Option<Uuid>,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateScheduleHandlerRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub department_id: Option<Uuid>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAssignmentHandlerRequest {
    pub schedule_id: Uuid,
    pub shift_id: Uuid,
    pub employee_id: Uuid,
    pub assignment_date: String,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAssignmentHandlerRequest {
    pub notes: Option<String>,
    pub status: Option<String>,
    pub actual_start_time: Option<String>,
    pub actual_end_time: Option<String>,
    pub overtime_minutes: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct EmployeeAssignmentsQuery {
    pub employee_id: Uuid,
    pub from_date: String,
    pub to_date: String,
}

pub async fn list_shifts(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> ApiResult<Json<erp_core::Paginated<ShiftResponse>>> {
    let service = ShiftSchedulingService::new();
    let result = service.list_shifts(&state.pool, pagination).await?;
    
    Ok(Json(erp_core::Paginated::new(
        result.items.into_iter().map(ShiftResponse::from).collect(),
        result.total,
        Pagination { page: result.page, per_page: result.per_page }
    )))
}

pub async fn list_active_shifts(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<ShiftResponse>>> {
    let service = ShiftSchedulingService::new();
    let shifts = service.list_active_shifts(&state.pool).await?;
    Ok(Json(shifts.into_iter().map(ShiftResponse::from).collect()))
}

pub async fn get_shift(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ShiftResponse>> {
    let service = ShiftSchedulingService::new();
    let shift = service.get_shift(&state.pool, id).await?;
    Ok(Json(ShiftResponse::from(shift)))
}

pub async fn create_shift(
    State(state): State<AppState>,
    Json(req): Json<CreateShiftHandlerRequest>,
) -> ApiResult<Json<ShiftResponse>> {
    let service = ShiftSchedulingService::new();
    
    let start_time = NaiveTime::parse_from_str(&req.start_time, "%H:%M")
        .map_err(|_| erp_core::Error::validation("Invalid start time format, use HH:MM"))?;
    let end_time = NaiveTime::parse_from_str(&req.end_time, "%H:%M")
        .map_err(|_| erp_core::Error::validation("Invalid end time format, use HH:MM"))?;
    
    let shift_req = CreateShiftRequest {
        code: req.code,
        name: req.name,
        description: req.description,
        start_time,
        end_time,
        break_minutes: req.break_minutes,
        grace_period_minutes: req.grace_period_minutes,
        color_code: req.color_code,
    };
    
    let shift = service.create_shift(&state.pool, shift_req).await?;
    Ok(Json(ShiftResponse::from(shift)))
}

pub async fn update_shift(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateShiftHandlerRequest>,
) -> ApiResult<Json<ShiftResponse>> {
    let service = ShiftSchedulingService::new();
    
    let update_req = UpdateShiftRequest {
        name: req.name,
        description: req.description,
        start_time: req.start_time.and_then(|t| NaiveTime::parse_from_str(&t, "%H:%M").ok()),
        end_time: req.end_time.and_then(|t| NaiveTime::parse_from_str(&t, "%H:%M").ok()),
        break_minutes: req.break_minutes,
        grace_period_minutes: req.grace_period_minutes,
        color_code: req.color_code,
        status: req.status.and_then(|s| match s.as_str() {
            "Draft" => Some(ShiftStatus::Draft),
            "Inactive" => Some(ShiftStatus::Inactive),
            "Active" => Some(ShiftStatus::Active),
            _ => None,
        }),
    };
    
    let shift = service.update_shift(&state.pool, id, update_req).await?;
    Ok(Json(ShiftResponse::from(shift)))
}

pub async fn delete_shift(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = ShiftSchedulingService::new();
    service.delete_shift(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn list_schedules(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> ApiResult<Json<erp_core::Paginated<ScheduleResponse>>> {
    let service = ShiftSchedulingService::new();
    let result = service.list_schedules(&state.pool, pagination).await?;
    
    Ok(Json(erp_core::Paginated::new(
        result.items.into_iter().map(ScheduleResponse::from).collect(),
        result.total,
        Pagination { page: result.page, per_page: result.per_page }
    )))
}

pub async fn get_schedule(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ScheduleResponse>> {
    let service = ShiftSchedulingService::new();
    let schedule = service.get_schedule(&state.pool, id).await?;
    Ok(Json(ScheduleResponse::from(schedule)))
}

pub async fn create_schedule(
    State(state): State<AppState>,
    Json(req): Json<CreateScheduleHandlerRequest>,
) -> ApiResult<Json<ScheduleResponse>> {
    let service = ShiftSchedulingService::new();
    
    let start_date = NaiveDate::parse_from_str(&req.start_date, "%Y-%m-%d")
        .map_err(|_| erp_core::Error::validation("Invalid start date format, use YYYY-MM-DD"))?;
    let end_date = NaiveDate::parse_from_str(&req.end_date, "%Y-%m-%d")
        .map_err(|_| erp_core::Error::validation("Invalid end date format, use YYYY-MM-DD"))?;
    
    let schedule_req = CreateScheduleRequest {
        code: req.code,
        name: req.name,
        description: req.description,
        department_id: req.department_id,
        start_date,
        end_date,
    };
    
    let schedule = service.create_schedule(&state.pool, schedule_req).await?;
    Ok(Json(ScheduleResponse::from(schedule)))
}

pub async fn update_schedule(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateScheduleHandlerRequest>,
) -> ApiResult<Json<ScheduleResponse>> {
    let service = ShiftSchedulingService::new();
    
    let update_req = UpdateScheduleRequest {
        name: req.name,
        description: req.description,
        department_id: req.department_id,
        start_date: req.start_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()),
        end_date: req.end_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()),
        status: req.status.and_then(|s| match s.as_str() {
            "Draft" => Some(ScheduleStatus::Draft),
            "Archived" => Some(ScheduleStatus::Archived),
            "Published" => Some(ScheduleStatus::Published),
            _ => None,
        }),
    };
    
    let schedule = service.update_schedule(&state.pool, id, update_req).await?;
    Ok(Json(ScheduleResponse::from(schedule)))
}

pub async fn publish_schedule(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ScheduleResponse>> {
    let service = ShiftSchedulingService::new();
    let schedule = service.publish_schedule(&state.pool, id).await?;
    Ok(Json(ScheduleResponse::from(schedule)))
}

pub async fn delete_schedule(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = ShiftSchedulingService::new();
    service.delete_schedule(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn list_assignments(
    State(state): State<AppState>,
    Path(schedule_id): Path<Uuid>,
) -> ApiResult<Json<Vec<AssignmentResponse>>> {
    let service = ShiftSchedulingService::new();
    let assignments = service.list_schedule_assignments(&state.pool, schedule_id).await?;
    Ok(Json(assignments.into_iter().map(AssignmentResponse::from).collect()))
}

pub async fn list_employee_assignments(
    State(state): State<AppState>,
    Query(query): Query<EmployeeAssignmentsQuery>,
) -> ApiResult<Json<Vec<AssignmentResponse>>> {
    let service = ShiftSchedulingService::new();
    
    let from_date = NaiveDate::parse_from_str(&query.from_date, "%Y-%m-%d")
        .map_err(|_| erp_core::Error::validation("Invalid from_date format, use YYYY-MM-DD"))?;
    let to_date = NaiveDate::parse_from_str(&query.to_date, "%Y-%m-%d")
        .map_err(|_| erp_core::Error::validation("Invalid to_date format, use YYYY-MM-DD"))?;
    
    let assignments = service.list_employee_assignments(&state.pool, query.employee_id, from_date, to_date).await?;
    Ok(Json(assignments.into_iter().map(AssignmentResponse::from).collect()))
}

pub async fn get_assignment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<AssignmentResponse>> {
    let service = ShiftSchedulingService::new();
    let assignment = service.get_assignment(&state.pool, id).await?;
    Ok(Json(AssignmentResponse::from(assignment)))
}

pub async fn create_assignment(
    State(state): State<AppState>,
    Json(req): Json<CreateAssignmentHandlerRequest>,
) -> ApiResult<Json<AssignmentResponse>> {
    let service = ShiftSchedulingService::new();
    
    let assignment_date = NaiveDate::parse_from_str(&req.assignment_date, "%Y-%m-%d")
        .map_err(|_| erp_core::Error::validation("Invalid assignment_date format, use YYYY-MM-DD"))?;
    
    let assignment_req = CreateAssignmentRequest {
        schedule_id: req.schedule_id,
        shift_id: req.shift_id,
        employee_id: req.employee_id,
        assignment_date,
        notes: req.notes,
    };
    
    let assignment = service.create_assignment(&state.pool, assignment_req).await?;
    Ok(Json(AssignmentResponse::from(assignment)))
}

pub async fn update_assignment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateAssignmentHandlerRequest>,
) -> ApiResult<Json<AssignmentResponse>> {
    let service = ShiftSchedulingService::new();
    
    let update_req = UpdateAssignmentRequest {
        notes: req.notes,
        status: req.status.and_then(|s| match s.as_str() {
            "Scheduled" => Some(AssignmentStatus::Scheduled),
            "Confirmed" => Some(AssignmentStatus::Confirmed),
            "InProgress" => Some(AssignmentStatus::InProgress),
            "Completed" => Some(AssignmentStatus::Completed),
            "Absent" => Some(AssignmentStatus::Absent),
            "Cancelled" => Some(AssignmentStatus::Cancelled),
            _ => None,
        }),
        actual_start_time: req.actual_start_time.and_then(|t| chrono::DateTime::parse_from_rfc3339(&t).ok())
            .map(|d| d.with_timezone(&chrono::Utc)),
        actual_end_time: req.actual_end_time.and_then(|t| chrono::DateTime::parse_from_rfc3339(&t).ok())
            .map(|d| d.with_timezone(&chrono::Utc)),
        overtime_minutes: req.overtime_minutes,
    };
    
    let assignment = service.update_assignment(&state.pool, id, update_req).await?;
    Ok(Json(AssignmentResponse::from(assignment)))
}

pub async fn delete_assignment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = ShiftSchedulingService::new();
    service.delete_assignment(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn clock_in(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<AssignmentResponse>> {
    let service = ShiftSchedulingService::new();
    let assignment = service.clock_in(&state.pool, id).await?;
    Ok(Json(AssignmentResponse::from(assignment)))
}

pub async fn clock_out(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<AssignmentResponse>> {
    let service = ShiftSchedulingService::new();
    let assignment = service.clock_out(&state.pool, id).await?;
    Ok(Json(AssignmentResponse::from(assignment)))
}

#[derive(Debug, Deserialize)]
pub struct DailyScheduleQuery {
    pub date: String,
}

pub async fn get_daily_schedule(
    State(state): State<AppState>,
    Path(schedule_id): Path<Uuid>,
    Query(query): Query<DailyScheduleQuery>,
) -> ApiResult<Json<Vec<AssignmentResponse>>> {
    let service = ShiftSchedulingService::new();
    
    let date = NaiveDate::parse_from_str(&query.date, "%Y-%m-%d")
        .map_err(|_| erp_core::Error::validation("Invalid date format, use YYYY-MM-DD"))?;
    
    let assignments = service.get_daily_schedule(&state.pool, schedule_id, date).await?;
    Ok(Json(assignments.into_iter().map(AssignmentResponse::from).collect()))
}

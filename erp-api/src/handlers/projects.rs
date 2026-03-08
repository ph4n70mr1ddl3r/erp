use axum::{
    extract::State,
    Json,
};
use axum::extract::Path;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;
use crate::db::AppState;
use crate::error::ApiResult;
use crate::handlers::auth::AuthUser;
use erp_core::{Pagination, BaseEntity};
use erp_projects::{Project, ProjectTask, ProjectMilestone, ProjectStatus, TaskStatus, MilestoneStatus, Timesheet, BillingStatus, ProjectType, BillingMethod, TaskPriority, TimesheetStatus};

#[derive(Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
    pub customer_id: Option<String>,
    pub project_manager_id: Option<String>,
    pub start_date: String,
    pub end_date: Option<String>,
    pub budget: Option<i64>,
    pub billing_type: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateProjectStatusRequest {
    pub status: String,
}

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    pub project_id: String,
    pub name: String,
    pub description: Option<String>,
    pub assigned_to: Option<String>,
    pub start_date: String,
    pub due_date: Option<String>,
    pub estimated_hours: Option<f64>,
}

#[derive(Deserialize)]
pub struct CreateMilestoneRequest {
    pub project_id: String,
    pub name: String,
    pub description: Option<String>,
    pub planned_date: String,
    pub billing_amount: Option<i64>,
}

#[derive(Deserialize)]
pub struct CreateTimesheetRequest {
    pub employee_id: String,
    pub period_start: String,
    pub period_end: String,
}

#[derive(Deserialize)]
pub struct AddTimesheetEntryRequest {
    pub timesheet_id: String,
    pub project_id: Option<String>,
    pub task_id: Option<String>,
    pub date: String,
    pub hours: f64,
    pub description: Option<String>,
    pub billable: Option<bool>,
    pub hourly_rate: Option<i64>,
}

#[derive(Serialize)]
pub struct ProjectResponse {
    pub id: String,
    pub project_number: String,
    pub name: String,
    pub description: Option<String>,
    pub customer_id: Option<String>,
    pub project_manager_id: Option<String>,
    pub status: String,
    pub start_date: String,
    pub end_date: Option<String>,
    pub budget: i64,
    pub percent_complete: i32,
    pub created_at: String,
}

impl From<Project> for ProjectResponse {
    fn from(p: Project) -> Self {
        Self {
            id: p.base.id.to_string(),
            project_number: p.project_number,
            name: p.name,
            description: p.description,
            customer_id: p.customer_id.map(|id| id.to_string()),
            project_manager_id: p.project_manager.map(|id| id.to_string()),
            status: format!("{:?}", p.status),
            start_date: p.start_date.to_rfc3339(),
            end_date: p.end_date.map(|d| d.to_rfc3339()),
            budget: p.budget,
            percent_complete: p.percent_complete,
            created_at: p.base.created_at.to_rfc3339(),
        }
    }
}

pub async fn list_projects(State(state): State<AppState>) -> ApiResult<Json<Vec<ProjectResponse>>> {
    let result = state.project_svc.list_projects(Pagination::new(1, 100)).await?;
    Ok(Json(result.items.into_iter().map(|p| p.into()).collect()))
}

pub async fn create_project(
    State(state): State<AppState>,
    axum::Extension(user): axum::Extension<AuthUser>,
    Json(req): Json<CreateProjectRequest>,
) -> ApiResult<Json<ProjectResponse>> {
    let project = Project {
        base: BaseEntity::new(),
        project_number: String::new(),
        name: req.name,
        description: req.description,
        customer_id: req.customer_id.and_then(|id| Uuid::parse_str(&id).ok()),
        project_type: ProjectType::Internal,
        start_date: chrono::DateTime::parse_from_rfc3339(&req.start_date)
            .map(|d| d.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now()),
        end_date: req.end_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
            .map(|d| d.with_timezone(&Utc)),
        budget: req.budget.unwrap_or(0),
        billable: true,
        billing_method: match req.billing_type.as_deref() {
            Some("TimeAndMaterials") => BillingMethod::TimeAndMaterials,
            Some("Milestone") => BillingMethod::Milestone,
            Some("Retainer") => BillingMethod::Retainer,
            Some("Hourly") => BillingMethod::Hourly,
            _ => BillingMethod::FixedPrice,
        },
        project_manager: req.project_manager_id.and_then(|id| Uuid::parse_str(&id).ok()),
        status: ProjectStatus::Active,
        percent_complete: 0,
    };
    let project = state.project_svc.create_project(project, Some(user.user_id())).await?;
    Ok(Json(project.into()))
}

pub async fn get_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<ProjectResponse>> {
    let id = Uuid::parse_str(&id).map_err(|_| erp_core::Error::validation("Invalid project id"))?;
    let project = state.project_svc.get_project(id).await?;
    Ok(Json(project.into()))
}

pub async fn update_status(
    State(state): State<AppState>,
    axum::Extension(user): axum::Extension<AuthUser>,
    Path(id): Path<String>,
    Json(req): Json<UpdateProjectStatusRequest>,
) -> ApiResult<Json<ProjectResponse>> {
    let id = Uuid::parse_str(&id).map_err(|_| erp_core::Error::validation("Invalid project id"))?;
    let status = match req.status.as_str() {
        "OnHold" => ProjectStatus::OnHold,
        "Completed" => ProjectStatus::Completed,
        "Cancelled" => ProjectStatus::Cancelled,
        _ => ProjectStatus::Active,
    };
    let project = state.project_svc.update_status(id, status, Some(user.user_id())).await?;
    Ok(Json(project.into()))
}

#[derive(Serialize)]
pub struct TaskResponse {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub description: Option<String>,
    pub assigned_to: Option<String>,
    pub status: String,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    pub estimated_hours: Option<f64>,
    pub actual_hours: f64,
    pub percent_complete: i32,
}

impl From<ProjectTask> for TaskResponse {
    fn from(t: ProjectTask) -> Self {
        Self {
            id: t.base.id.to_string(),
            project_id: t.project_id.to_string(),
            name: t.name,
            description: t.description,
            assigned_to: t.assigned_to.map(|id| id.to_string()),
            status: format!("{:?}", t.status),
            start_date: Some(t.start_date.to_rfc3339()),
            due_date: t.end_date.map(|d| d.to_rfc3339()),
            estimated_hours: t.estimated_hours,
            actual_hours: t.actual_hours,
            percent_complete: t.percent_complete,
        }
    }
}

pub async fn list_tasks(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> ApiResult<Json<Vec<TaskResponse>>> {
    let project_id = Uuid::parse_str(&project_id).map_err(|_| erp_core::Error::validation("Invalid project id"))?;
    let tasks = state.project_svc.list_tasks_by_project(project_id).await?;
    Ok(Json(tasks.into_iter().map(|t| t.into()).collect()))
}

pub async fn create_task(
    State(state): State<AppState>,
    axum::Extension(user): axum::Extension<AuthUser>,
    Json(req): Json<CreateTaskRequest>,
) -> ApiResult<Json<TaskResponse>> {
    let project_id = Uuid::parse_str(&req.project_id).map_err(|_| erp_core::Error::validation("Invalid project id"))?;
    let task = ProjectTask {
        base: BaseEntity::new(),
        project_id,
        task_number: 0,
        name: req.name,
        description: req.description,
        parent_task_id: None,
        assigned_to: req.assigned_to.and_then(|id| Uuid::parse_str(&id).ok()),
        start_date: chrono::DateTime::parse_from_rfc3339(&req.start_date)
            .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
        end_date: req.due_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
            .map(|d| d.with_timezone(&Utc)),
        estimated_hours: req.estimated_hours,
        actual_hours: 0.0,
        percent_complete: 0,
        priority: TaskPriority::Medium,
        status: TaskStatus::NotStarted,
        billable: true,
    };
    let task = state.project_svc.add_task(task, Some(user.user_id())).await?;
    Ok(Json(task.into()))
}

pub async fn complete_task(
    State(state): State<AppState>,
    axum::Extension(user): axum::Extension<AuthUser>,
    Path(id): Path<String>,
) -> ApiResult<Json<TaskResponse>> {
    let id = Uuid::parse_str(&id).map_err(|_| erp_core::Error::validation("Invalid task id"))?;
    let task = state.project_svc.complete_task(id, Some(user.user_id())).await?;
    Ok(Json(task.into()))
}

#[derive(Serialize)]
pub struct MilestoneResponse {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub description: Option<String>,
    pub planned_date: String,
    pub actual_date: Option<String>,
    pub status: String,
    pub billing_amount: i64,
}

impl From<ProjectMilestone> for MilestoneResponse {
    fn from(m: ProjectMilestone) -> Self {
        Self {
            id: m.base.id.to_string(),
            project_id: m.project_id.to_string(),
            name: m.name,
            description: m.description,
            planned_date: m.planned_date.to_rfc3339(),
            actual_date: m.actual_date.map(|d| d.to_rfc3339()),
            status: format!("{:?}", m.status),
            billing_amount: m.billing_amount,
        }
    }
}

pub async fn list_milestones(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> ApiResult<Json<Vec<MilestoneResponse>>> {
    let project_id = Uuid::parse_str(&project_id).map_err(|_| erp_core::Error::validation("Invalid project id"))?;
    let milestones = state.project_svc.list_milestones_by_project(project_id).await?;
    Ok(Json(milestones.into_iter().map(|m| m.into()).collect()))
}

pub async fn create_milestone(
    State(state): State<AppState>,
    axum::Extension(user): axum::Extension<AuthUser>,
    Json(req): Json<CreateMilestoneRequest>,
) -> ApiResult<Json<MilestoneResponse>> {
    let project_id = Uuid::parse_str(&req.project_id).map_err(|_| erp_core::Error::validation("Invalid project id"))?;
    let milestone = ProjectMilestone {
        base: BaseEntity::new(),
        project_id,
        name: req.name,
        description: req.description,
        planned_date: chrono::DateTime::parse_from_rfc3339(&req.planned_date)
            .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
        actual_date: None,
        status: MilestoneStatus::Planned,
        billing_amount: req.billing_amount.unwrap_or(0),
        billing_status: BillingStatus::NotBilled,
    };
    let milestone = state.project_svc.add_milestone(milestone, Some(user.user_id())).await?;
    Ok(Json(milestone.into()))
}

pub async fn complete_milestone(
    State(state): State<AppState>,
    axum::Extension(user): axum::Extension<AuthUser>,
    Path(id): Path<String>,
) -> ApiResult<Json<MilestoneResponse>> {
    let id = Uuid::parse_str(&id).map_err(|_| erp_core::Error::validation("Invalid milestone id"))?;
    let milestone = state.project_svc.complete_milestone(id, Some(user.user_id())).await?;
    Ok(Json(milestone.into()))
}

#[derive(Serialize)]
pub struct TimesheetResponse {
    pub id: String,
    pub timesheet_number: String,
    pub employee_id: String,
    pub period_start: String,
    pub period_end: String,
    pub total_hours: f64,
    pub status: String,
    pub submitted_at: Option<String>,
    pub approved_at: Option<String>,
}

impl From<Timesheet> for TimesheetResponse {
    fn from(t: Timesheet) -> Self {
        Self {
            id: t.base.id.to_string(),
            timesheet_number: t.timesheet_number,
            employee_id: t.employee_id.to_string(),
            period_start: t.period_start.to_rfc3339(),
            period_end: t.period_end.to_rfc3339(),
            total_hours: t.total_hours,
            status: format!("{:?}", t.status),
            submitted_at: t.submitted_at.map(|d| d.to_rfc3339()),
            approved_at: t.approved_at.map(|d| d.to_rfc3339()),
        }
    }
}

pub async fn list_timesheets(State(state): State<AppState>) -> ApiResult<Json<Vec<TimesheetResponse>>> {
    let result = state.timesheet_svc.list_timesheets(Pagination::new(1, 100)).await?;
    Ok(Json(result.items.into_iter().map(|t| t.into()).collect()))
}

pub async fn create_timesheet(
    State(state): State<AppState>,
    axum::Extension(user): axum::Extension<AuthUser>,
    Json(req): Json<CreateTimesheetRequest>,
) -> ApiResult<Json<TimesheetResponse>> {
    let employee_id = Uuid::parse_str(&req.employee_id).map_err(|_| erp_core::Error::validation("Invalid employee id"))?;
    let timesheet = Timesheet {
        base: BaseEntity::new(),
        timesheet_number: String::new(),
        employee_id,
        period_start: chrono::DateTime::parse_from_rfc3339(&req.period_start)
            .map(|d| d.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now()),
        period_end: chrono::DateTime::parse_from_rfc3339(&req.period_end)
            .map(|d| d.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now()),
        total_hours: 0.0,
        overtime_hours: 0.0,
        status: TimesheetStatus::Draft,
        submitted_at: None,
        approved_at: None,
        approved_by: None,
    };
    let timesheet = state.timesheet_svc.create_timesheet(timesheet, Some(user.user_id())).await?;
    Ok(Json(timesheet.into()))
}

pub async fn submit_timesheet(
    State(state): State<AppState>,
    axum::Extension(user): axum::Extension<AuthUser>,
    Path(id): Path<String>,
) -> ApiResult<Json<&'static str>> {
    let id = Uuid::parse_str(&id).map_err(|_| erp_core::Error::validation("Invalid timesheet id"))?;
    state.timesheet_svc.submit_timesheet(id, Some(user.user_id())).await?;
    Ok(Json("submitted"))
}

pub async fn approve_timesheet(
    State(state): State<AppState>,
    axum::Extension(user): axum::Extension<AuthUser>,
    Path(id): Path<String>,
) -> ApiResult<Json<&'static str>> {
    let id = Uuid::parse_str(&id).map_err(|_| erp_core::Error::validation("Invalid timesheet id"))?;
    state.timesheet_svc.approve_timesheet(id, user.user_id()).await?;
    Ok(Json("approved"))
}

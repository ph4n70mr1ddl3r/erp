use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::AppState;
use crate::error::ApiResult;

#[derive(Debug, Deserialize)]
pub struct CreateJobRequest {
    pub name: String,
    pub handler: String,
    pub payload: Option<serde_json::Value>,
    pub priority: Option<String>,
    pub scheduled_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct JobResponse {
    pub id: Uuid,
    pub name: String,
    pub job_type: String,
    pub handler: String,
    pub status: String,
    pub priority: String,
    pub run_count: i64,
    pub success_count: i64,
    pub failure_count: i64,
    pub next_run_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateScheduleRequest {
    pub name: String,
    pub job_name: String,
    pub handler: String,
    pub schedule_type: String,
    pub cron_expression: Option<String>,
    pub interval_minutes: Option<i32>,
    pub payload: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct JobListQuery {
    pub status: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

pub async fn submit_job(
    State(state): State<AppState>,
    Json(req): Json<CreateJobRequest>,
) -> ApiResult<Json<JobResponse>> {
    let priority = req.priority.as_deref()
        .map(parse_job_priority)
        .transpose()?;
    
    let scheduled_at = req.scheduled_at.as_deref()
        .map(|s| chrono::DateTime::parse_from_rfc3339(s))
        .transpose()
        .map_err(|e| crate::error::ApiError::BadRequest(format!("Invalid scheduled_at: {}", e)))?
        .map(|dt| dt.with_timezone(&chrono::Utc));
    
    let service = erp_jobs::JobService::new();
    let job = service.submit(
        &state.pool,
        req.name,
        req.handler,
        req.payload,
        priority,
        scheduled_at,
        None,
    ).await?;
    
    Ok(Json(JobResponse {
        id: job.base.id,
        name: job.name,
        job_type: format!("{:?}", job.job_type),
        handler: job.handler,
        status: format!("{:?}", job.status),
        priority: format!("{:?}", job.priority),
        run_count: job.run_count,
        success_count: job.success_count,
        failure_count: job.failure_count,
        next_run_at: job.next_run_at.map(|dt| dt.to_rfc3339()),
        created_at: job.created_at.to_rfc3339(),
    }))
}

pub async fn list_jobs(
    State(state): State<AppState>,
    Query(query): Query<JobListQuery>,
) -> ApiResult<Json<Vec<JobResponse>>> {
    let status = query.status.as_deref()
        .map(parse_job_status)
        .transpose()?;
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);
    
    let service = erp_jobs::JobService::new();
    let jobs = service.list(&state.pool, status, limit, offset).await?;
    
    let response: Vec<JobResponse> = jobs.into_iter().map(|j| JobResponse {
        id: j.base.id,
        name: j.name,
        job_type: format!("{:?}", j.job_type),
        handler: j.handler,
        status: format!("{:?}", j.status),
        priority: format!("{:?}", j.priority),
        run_count: j.run_count,
        success_count: j.success_count,
        failure_count: j.failure_count,
        next_run_at: j.next_run_at.map(|dt| dt.to_rfc3339()),
        created_at: j.created_at.to_rfc3339(),
    }).collect();
    
    Ok(Json(response))
}

pub async fn get_job(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<JobResponse>> {
    let service = erp_jobs::JobService::new();
    let job = service.get(&state.pool, id).await?
        .ok_or_else(|| crate::error::ApiError::NotFound("Job not found".into()))?;
    
    Ok(Json(JobResponse {
        id: job.base.id,
        name: job.name,
        job_type: format!("{:?}", job.job_type),
        handler: job.handler,
        status: format!("{:?}", job.status),
        priority: format!("{:?}", job.priority),
        run_count: job.run_count,
        success_count: job.success_count,
        failure_count: job.failure_count,
        next_run_at: job.next_run_at.map(|dt| dt.to_rfc3339()),
        created_at: job.created_at.to_rfc3339(),
    }))
}

pub async fn cancel_job(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let service = erp_jobs::JobService::new();
    service.cancel(&state.pool, id).await?;
    Ok(StatusCode::OK)
}

pub async fn retry_job(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<JobResponse>> {
    let service = erp_jobs::JobService::new();
    let job = service.retry(&state.pool, id).await?;
    
    Ok(Json(JobResponse {
        id: job.base.id,
        name: job.name,
        job_type: format!("{:?}", job.job_type),
        handler: job.handler,
        status: format!("{:?}", job.status),
        priority: format!("{:?}", job.priority),
        run_count: job.run_count,
        success_count: job.success_count,
        failure_count: job.failure_count,
        next_run_at: job.next_run_at.map(|dt| dt.to_rfc3339()),
        created_at: job.created_at.to_rfc3339(),
    }))
}

pub async fn create_schedule(
    State(state): State<AppState>,
    Json(req): Json<CreateScheduleRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let schedule_type = parse_schedule_type(&req.schedule_type)?;
    
    let service = erp_jobs::JobScheduleService::new();
    let schedule = service.create(
        &state.pool,
        req.name,
        req.job_name,
        req.handler,
        schedule_type,
        req.cron_expression,
        req.interval_minutes,
        req.payload,
    ).await?;
    
    Ok(Json(serde_json::json!({
        "id": schedule.base.id,
        "name": schedule.name,
        "schedule_type": format!("{:?}", schedule.schedule_type),
        "enabled": schedule.enabled,
        "next_scheduled_run": schedule.next_scheduled_run.map(|dt| dt.to_rfc3339()),
    })))
}

pub async fn list_schedules(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<serde_json::Value>>> {
    let service = erp_jobs::JobScheduleService::new();
    let schedules = service.list(&state.pool).await?;
    
    let response: Vec<serde_json::Value> = schedules.into_iter().map(|s| serde_json::json!({
        "id": s.base.id,
        "name": s.name,
        "job_name": s.job_name,
        "handler": s.handler,
        "schedule_type": format!("{:?}", s.schedule_type),
        "cron_expression": s.cron_expression,
        "interval_minutes": s.interval_minutes,
        "enabled": s.enabled,
        "next_scheduled_run": s.next_scheduled_run.map(|dt| dt.to_rfc3339()),
        "last_run": s.last_run.map(|dt| dt.to_rfc3339()),
    })).collect();
    
    Ok(Json(response))
}

pub async fn enable_schedule(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let service = erp_jobs::JobScheduleService::new();
    service.enable(&state.pool, id).await?;
    Ok(StatusCode::OK)
}

pub async fn disable_schedule(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let service = erp_jobs::JobScheduleService::new();
    service.disable(&state.pool, id).await?;
    Ok(StatusCode::OK)
}

fn parse_job_priority(s: &str) -> anyhow::Result<erp_jobs::JobPriority> {
    match s {
        "Low" => Ok(erp_jobs::JobPriority::Low),
        "Normal" => Ok(erp_jobs::JobPriority::Normal),
        "High" => Ok(erp_jobs::JobPriority::High),
        "Critical" => Ok(erp_jobs::JobPriority::Critical),
        _ => Err(anyhow::anyhow!("Invalid job priority: {}", s)),
    }
}

fn parse_job_status(s: &str) -> anyhow::Result<erp_jobs::JobStatus> {
    match s {
        "Pending" => Ok(erp_jobs::JobStatus::Pending),
        "Scheduled" => Ok(erp_jobs::JobStatus::Scheduled),
        "Running" => Ok(erp_jobs::JobStatus::Running),
        "Completed" => Ok(erp_jobs::JobStatus::Completed),
        "Failed" => Ok(erp_jobs::JobStatus::Failed),
        "Cancelled" => Ok(erp_jobs::JobStatus::Cancelled),
        "Paused" => Ok(erp_jobs::JobStatus::Paused),
        _ => Err(anyhow::anyhow!("Invalid job status: {}", s)),
    }
}

fn parse_schedule_type(s: &str) -> anyhow::Result<erp_jobs::ScheduleType> {
    match s {
        "Cron" => Ok(erp_jobs::ScheduleType::Cron),
        "Interval" => Ok(erp_jobs::ScheduleType::Interval),
        "Daily" => Ok(erp_jobs::ScheduleType::Daily),
        "Weekly" => Ok(erp_jobs::ScheduleType::Weekly),
        "Monthly" => Ok(erp_jobs::ScheduleType::Monthly),
        "SpecificTimes" => Ok(erp_jobs::ScheduleType::SpecificTimes),
        _ => Err(anyhow::anyhow!("Invalid schedule type: {}", s)),
    }
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/", axum::routing::post(submit_job).get(list_jobs))
        .route("/:id", axum::routing::get(get_job))
        .route("/:id/cancel", axum::routing::post(cancel_job))
        .route("/:id/retry", axum::routing::post(retry_job))
        .route("/schedules", axum::routing::post(create_schedule).get(list_schedules))
        .route("/schedules/:id/enable", axum::routing::post(enable_schedule))
        .route("/schedules/:id/disable", axum::routing::post(disable_schedule))
}

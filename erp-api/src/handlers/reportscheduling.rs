use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::AppState;
use erp_reportscheduling::*;

#[derive(Deserialize)]
pub struct CreateScheduleRequest {
    pub name: String,
    pub description: Option<String>,
    pub report_type: String,
    pub report_config: serde_json::Value,
    pub frequency: ScheduleFrequency,
    pub format: ReportFormat,
    pub delivery_method: DeliveryMethod,
    pub recipients: Vec<String>,
}

#[derive(Deserialize)]
pub struct ListSchedulesQuery {
    pub owner_id: Option<Uuid>,
    pub status: Option<ScheduleStatus>,
}

#[derive(Serialize)]
pub struct ScheduleResponse {
    pub id: Uuid,
    pub name: String,
    pub report_type: String,
    pub frequency: ScheduleFrequency,
    pub status: ScheduleStatus,
    pub next_run_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<ReportSchedule> for ScheduleResponse {
    fn from(s: ReportSchedule) -> Self {
        Self {
            id: s.base.id,
            name: s.name,
            report_type: s.report_type,
            frequency: s.frequency,
            status: s.status,
            next_run_at: s.next_run_at,
        }
    }
}

pub async fn create_schedule(
    State(state): State<AppState>,
    Json(req): Json<CreateScheduleRequest>,
) -> Result<Json<ScheduleResponse>, StatusCode> {
    let user_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    let service = ReportScheduleService::new();
    let schedule = service
        .create(
            &state.pool,
            req.name,
            req.description,
            req.report_type,
            req.report_config,
            req.frequency,
            req.format,
            req.delivery_method,
            req.recipients,
            user_id,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(ScheduleResponse::from(schedule)))
}

pub async fn list_schedules(
    State(state): State<AppState>,
    Query(query): Query<ListSchedulesQuery>,
) -> Result<Json<Vec<ScheduleResponse>>, StatusCode> {
    let service = ReportScheduleService::new();
    let schedules = service
        .list(&state.pool, query.owner_id, query.status)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(schedules.into_iter().map(ScheduleResponse::from).collect()))
}

pub async fn get_schedule(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ScheduleResponse>, StatusCode> {
    let service = ReportScheduleService::new();
    let schedule = service
        .get(&state.pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(ScheduleResponse::from(schedule)))
}

pub async fn pause_schedule(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let service = ReportScheduleService::new();
    service.pause(&state.pool, id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

pub async fn resume_schedule(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let service = ReportScheduleService::new();
    service.resume(&state.pool, id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

pub async fn delete_schedule(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let service = ReportScheduleService::new();
    service.delete(&state.pool, id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

pub async fn get_executions(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<ScheduleExecution>>, StatusCode> {
    let service = ReportScheduleService::new();
    let executions = service
        .get_executions(&state.pool, id, 50)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(executions))
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/", axum::routing::post(create_schedule).get(list_schedules))
        .route("/:id", axum::routing::get(get_schedule).delete(delete_schedule))
        .route("/:id/pause", axum::routing::post(pause_schedule))
        .route("/:id/resume", axum::routing::post(resume_schedule))
        .route("/:id/executions", axum::routing::get(get_executions))
}

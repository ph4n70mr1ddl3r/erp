use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::BaseEntity;
use erp_backup::{BackupService, BackupSchedule, BackupType, BackupStorageType, BackupRecord};

#[derive(Serialize)]
pub struct BackupScheduleResponse {
    pub id: Uuid,
    pub name: String,
    pub backup_type: String,
    pub schedule_cron: String,
    pub retention_days: i32,
    pub is_active: bool,
    pub last_run: Option<String>,
    pub next_run: Option<String>,
}

impl From<BackupSchedule> for BackupScheduleResponse {
    fn from(s: BackupSchedule) -> Self {
        Self {
            id: s.base.id,
            name: s.name,
            backup_type: format!("{:?}", s.backup_type),
            schedule_cron: s.schedule_cron,
            retention_days: s.retention_days,
            is_active: s.is_active,
            last_run: s.last_run.map(|d| d.to_rfc3339()),
            next_run: s.next_run.map(|d| d.to_rfc3339()),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateScheduleRequest {
    pub name: String,
    pub schedule_cron: String,
    pub retention_days: Option<i32>,
    pub max_backups: Option<i32>,
}

pub async fn list_schedules(State(state): State<AppState>) -> ApiResult<Json<Vec<BackupScheduleResponse>>> {
    let service = BackupService::new();
    let schedules = service.list_schedules(&state.pool).await?;
    Ok(Json(schedules.into_iter().map(BackupScheduleResponse::from).collect()))
}

pub async fn create_schedule(
    State(state): State<AppState>,
    Json(req): Json<CreateScheduleRequest>,
) -> ApiResult<Json<BackupScheduleResponse>> {
    let service = BackupService::new();
    let schedule = BackupSchedule {
        base: BaseEntity::new(),
        name: req.name,
        backup_type: BackupType::Full,
        schedule_cron: req.schedule_cron,
        retention_days: req.retention_days.unwrap_or(30),
        max_backups: req.max_backups.unwrap_or(10),
        compression: true,
        encryption_enabled: false,
        encryption_key_id: None,
        storage_type: BackupStorageType::Local,
        storage_path: "./backups".to_string(),
        include_attachments: true,
        is_active: true,
        last_run: None,
        next_run: None,
    };
    let created = service.create_schedule(&state.pool, schedule).await?;
    Ok(Json(BackupScheduleResponse::from(created)))
}

#[derive(Serialize)]
pub struct BackupRecordResponse {
    pub id: Uuid,
    pub backup_type: String,
    pub status: String,
    pub file_path: String,
    pub file_size_bytes: i64,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub duration_seconds: Option<i64>,
    pub is_restorable: bool,
}

impl From<BackupRecord> for BackupRecordResponse {
    fn from(b: BackupRecord) -> Self {
        Self {
            id: b.base.id,
            backup_type: format!("{:?}", b.backup_type),
            status: format!("{:?}", b.status),
            file_path: b.file_path,
            file_size_bytes: b.file_size_bytes,
            started_at: b.started_at.to_rfc3339(),
            completed_at: b.completed_at.map(|d| d.to_rfc3339()),
            duration_seconds: b.duration_seconds,
            is_restorable: b.is_restorable,
        }
    }
}

#[derive(Deserialize)]
pub struct ListBackupsQuery {
    pub limit: Option<i32>,
}

pub async fn list_backups(
    State(state): State<AppState>,
    Query(query): Query<ListBackupsQuery>,
) -> ApiResult<Json<Vec<BackupRecordResponse>>> {
    let service = BackupService::new();
    let backups = service.list_backups(&state.pool, query.limit.unwrap_or(50)).await?;
    Ok(Json(backups.into_iter().map(BackupRecordResponse::from).collect()))
}

pub async fn execute_backup(State(state): State<AppState>) -> ApiResult<Json<BackupRecordResponse>> {
    let service = BackupService::new();
    let backup = service.execute_backup(&state.pool, None).await?;
    Ok(Json(BackupRecordResponse::from(backup)))
}

pub async fn get_backup(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<BackupRecordResponse>> {
    let service = BackupService::new();
    let backup = service.get_backup(&state.pool, id).await?
        .ok_or_else(|| anyhow::anyhow!("Backup not found"))?;
    Ok(Json(BackupRecordResponse::from(backup)))
}

pub async fn delete_backup(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = BackupService::new();
    service.delete_backup(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "deleted" })))
}

pub async fn restore_backup(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = BackupService::new();
    let restore = service.restore_backup(&state.pool, id, None).await?;
    Ok(Json(serde_json::json!({
        "status": "restored",
        "records_restored": restore.records_restored
    })))
}

pub async fn verify_backup(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = BackupService::new();
    let verification = service.verify_backup(&state.pool, id).await?;
    Ok(Json(serde_json::json!({
        "status": format!("{:?}", verification.status),
        "file_readable": verification.file_readable,
        "schema_valid": verification.schema_valid
    })))
}

#[derive(Serialize)]
pub struct StorageStatsResponse {
    pub total_size_bytes: i64,
    pub backup_count: i32,
}

pub async fn storage_stats(State(state): State<AppState>) -> ApiResult<Json<StorageStatsResponse>> {
    let service = BackupService::new();
    let stats = service.get_storage_stats(&state.pool).await?;
    Ok(Json(StorageStatsResponse {
        total_size_bytes: stats.total_size_bytes,
        backup_count: stats.backup_count,
    }))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/schedules", get(list_schedules).post(create_schedule))
        .route("/", get(list_backups).post(execute_backup))
        .route("/:id", get(get_backup).delete(delete_backup))
        .route("/:id/restore", post(restore_backup))
        .route("/:id/verify", post(verify_backup))
        .route("/stats", get(storage_stats))
}

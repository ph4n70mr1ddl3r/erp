use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::BaseEntity;
use erp_monitoring::{
    MonitoringService, HealthCheck, HealthCheckType, AlertRule, Alert, AlertStatus,
    AlertSeverity, AlertCondition, MetricType, SystemStatus, CurrentMetrics,
};

#[derive(Serialize)]
pub struct SystemStatusResponse {
    pub overall_status: String,
    pub active_alerts: i32,
    pub cpu_percent: f64,
    pub memory_percent: f64,
    pub disk_percent: f64,
    pub database_connections: i64,
    pub active_users: i64,
    pub last_updated: String,
}

pub async fn system_status(State(state): State<AppState>) -> ApiResult<Json<SystemStatusResponse>> {
    let service = MonitoringService::new();
    let status = service.get_system_status(&state.pool).await?;
    Ok(Json(SystemStatusResponse {
        overall_status: format!("{:?}", status.overall_status),
        active_alerts: status.active_alerts,
        cpu_percent: status.metrics.cpu_percent,
        memory_percent: status.metrics.memory_percent,
        disk_percent: status.metrics.disk_percent,
        database_connections: status.metrics.database_connections,
        active_users: status.metrics.active_users,
        last_updated: status.last_updated.to_rfc3339(),
    }))
}

pub async fn collect_metrics(State(state): State<AppState>) -> ApiResult<Json<serde_json::Value>> {
    let service = MonitoringService::new();
    let metrics = service.collect_system_metrics(&state.pool).await?;
    Ok(Json(serde_json::json!({
        "collected": metrics.len(),
        "metrics": metrics.iter().map(|m| serde_json::json!({
            "name": m.metric_name,
            "value": m.value,
            "unit": m.unit
        })).collect::<Vec<_>>()
    })))
}

#[derive(Deserialize)]
pub struct CreateHealthCheckRequest {
    pub name: String,
    pub check_type: String,
    pub endpoint: Option<String>,
    pub interval_seconds: Option<i32>,
}

#[derive(Serialize)]
pub struct HealthCheckResponse {
    pub id: Uuid,
    pub name: String,
    pub check_type: String,
    pub is_active: bool,
    pub last_status: Option<String>,
    pub last_response_time_ms: Option<i64>,
}

pub async fn list_health_checks(State(state): State<AppState>) -> ApiResult<Json<Vec<HealthCheckResponse>>> {
    let service = MonitoringService::new();
    let checks = service.list_health_checks(&state.pool).await?;
    Ok(Json(checks.into_iter().map(|c| HealthCheckResponse {
        id: c.base.id,
        name: c.name,
        check_type: format!("{:?}", c.check_type),
        is_active: c.is_active,
        last_status: c.last_status.map(|s| format!("{:?}", s)),
        last_response_time_ms: c.last_response_time_ms,
    }).collect()))
}

pub async fn create_health_check(
    State(state): State<AppState>,
    Json(req): Json<CreateHealthCheckRequest>,
) -> ApiResult<Json<HealthCheckResponse>> {
    let service = MonitoringService::new();
    let check_type = match req.check_type.as_str() {
        "Database" => HealthCheckType::Database,
        "Http" => HealthCheckType::Http,
        "Tcp" => HealthCheckType::Tcp,
        _ => HealthCheckType::Custom,
    };
    let check = HealthCheck {
        base: BaseEntity::new(),
        name: req.name,
        check_type,
        endpoint: req.endpoint,
        timeout_seconds: 30,
        interval_seconds: req.interval_seconds.unwrap_or(60),
        is_active: true,
        last_check: None,
        last_status: None,
        last_response_time_ms: None,
        consecutive_failures: 0,
        last_error: None,
    };
    let created = service.create_health_check(&state.pool, check).await?;
    Ok(Json(HealthCheckResponse {
        id: created.base.id,
        name: created.name,
        check_type: format!("{:?}", created.check_type),
        is_active: created.is_active,
        last_status: None,
        last_response_time_ms: None,
    }))
}

pub async fn run_health_check(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = MonitoringService::new();
    let result = service.run_health_check(&state.pool, id).await?;
    Ok(Json(serde_json::json!({
        "status": format!("{:?}", result.status),
        "response_time_ms": result.response_time_ms,
        "message": result.message
    })))
}

pub async fn run_all_health_checks(State(state): State<AppState>) -> ApiResult<Json<serde_json::Value>> {
    let service = MonitoringService::new();
    let results = service.run_all_health_checks(&state.pool).await?;
    Ok(Json(serde_json::json!({
        "checks_run": results.len(),
        "results": results.iter().map(|r| serde_json::json!({
            "status": format!("{:?}", r.status),
            "response_time_ms": r.response_time_ms
        })).collect::<Vec<_>>()
    })))
}

#[derive(Deserialize)]
pub struct CreateAlertRuleRequest {
    pub name: String,
    pub metric_name: String,
    pub threshold: f64,
    pub severity: String,
}

pub async fn list_alert_rules(State(state): State<AppState>) -> ApiResult<Json<Vec<serde_json::Value>>> {
    let service = MonitoringService::new();
    let rules = service.list_alert_rules(&state.pool).await?;
    Ok(Json(rules.into_iter().map(|r| serde_json::json!({
        "id": r.base.id,
        "name": r.name,
        "metric_name": r.metric_name,
        "threshold": r.threshold,
        "severity": format!("{:?}", r.severity),
        "is_active": r.is_active
    })).collect()))
}

pub async fn create_alert_rule(
    State(state): State<AppState>,
    Json(req): Json<CreateAlertRuleRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = MonitoringService::new();
    let severity = match req.severity.as_str() {
        "Critical" => AlertSeverity::Critical,
        "Emergency" => AlertSeverity::Emergency,
        "Info" => AlertSeverity::Info,
        _ => AlertSeverity::Warning,
    };
    let rule = AlertRule {
        base: BaseEntity::new(),
        name: req.name,
        metric_type: MetricType::Custom,
        metric_name: req.metric_name,
        condition: AlertCondition::GreaterThan,
        threshold: req.threshold,
        duration_minutes: 5,
        severity,
        notification_channels: None,
        is_active: true,
        last_triggered: None,
    };
    let created = service.create_alert_rule(&state.pool, rule).await?;
    Ok(Json(serde_json::json!({
        "id": created.base.id,
        "status": "created"
    })))
}

#[derive(Serialize)]
pub struct AlertResponse {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub status: String,
    pub severity: String,
    pub message: String,
    pub triggered_at: String,
}

pub async fn list_alerts(State(state): State<AppState>) -> ApiResult<Json<Vec<AlertResponse>>> {
    let service = MonitoringService::new();
    let alerts = service.list_alerts(&state.pool, None, 100).await?;
    Ok(Json(alerts.into_iter().map(|a| AlertResponse {
        id: a.base.id,
        rule_id: a.rule_id,
        status: format!("{:?}", a.status),
        severity: format!("{:?}", a.severity),
        message: a.message,
        triggered_at: a.triggered_at.to_rfc3339(),
    }).collect()))
}

pub async fn acknowledge_alert(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = MonitoringService::new();
    let _ = service.acknowledge_alert(&state.pool, id, Uuid::nil()).await?;
    Ok(Json(serde_json::json!({ "status": "acknowledged" })))
}

pub async fn resolve_alert(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = MonitoringService::new();
    let _ = service.resolve_alert(&state.pool, id, None).await?;
    Ok(Json(serde_json::json!({ "status": "resolved" })))
}

#[derive(Serialize)]
pub struct DatabaseStatsResponse {
    pub size_bytes: i64,
    pub table_count: i32,
    pub index_count: i32,
}

pub async fn database_stats(State(state): State<AppState>) -> ApiResult<Json<DatabaseStatsResponse>> {
    let service = MonitoringService::new();
    let stats = service.get_database_stats(&state.pool).await?;
    Ok(Json(DatabaseStatsResponse {
        size_bytes: stats.size_bytes,
        table_count: stats.table_count,
        index_count: stats.index_count,
    }))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/status", get(system_status))
        .route("/metrics/collect", post(collect_metrics))
        .route("/health-checks", get(list_health_checks).post(create_health_check))
        .route("/health-checks/:id/run", post(run_health_check))
        .route("/health-checks/run-all", post(run_all_health_checks))
        .route("/alert-rules", get(list_alert_rules).post(create_alert_rule))
        .route("/alerts", get(list_alerts))
        .route("/alerts/:id/acknowledge", post(acknowledge_alert))
        .route("/alerts/:id/resolve", post(resolve_alert))
        .route("/database-stats", get(database_stats))
}

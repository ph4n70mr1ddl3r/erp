use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use erp_core::Result;
use crate::models::*;

#[async_trait]
pub trait MonitoringRepository: Send + Sync {
    async fn create_metric(&self, pool: &SqlitePool, metric: SystemMetric) -> Result<SystemMetric>;
    async fn list_metrics(&self, pool: &SqlitePool, metric_type: Option<MetricType>, limit: i32) -> Result<Vec<SystemMetric>>;
    async fn create_health_check(&self, pool: &SqlitePool, check: HealthCheck) -> Result<HealthCheck>;
    async fn get_health_check(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<HealthCheck>>;
    async fn list_health_checks(&self, pool: &SqlitePool) -> Result<Vec<HealthCheck>>;
    async fn update_health_check(&self, pool: &SqlitePool, check: HealthCheck) -> Result<HealthCheck>;
    async fn create_health_result(&self, pool: &SqlitePool, result: HealthCheckResult) -> Result<HealthCheckResult>;
    async fn list_health_results(&self, pool: &SqlitePool, check_id: Uuid, limit: i32) -> Result<Vec<HealthCheckResult>>;
    async fn create_alert_rule(&self, pool: &SqlitePool, rule: AlertRule) -> Result<AlertRule>;
    async fn list_alert_rules(&self, pool: &SqlitePool) -> Result<Vec<AlertRule>>;
    async fn create_alert(&self, pool: &SqlitePool, alert: Alert) -> Result<Alert>;
    async fn list_alerts(&self, pool: &SqlitePool, status: Option<AlertStatus>, limit: i32) -> Result<Vec<Alert>>;
    async fn update_alert(&self, pool: &SqlitePool, alert: Alert) -> Result<Alert>;
}

pub struct SqliteMonitoringRepository;

#[async_trait]
impl MonitoringRepository for SqliteMonitoringRepository {
    async fn create_metric(&self, pool: &SqlitePool, metric: SystemMetric) -> Result<SystemMetric> {
        sqlx::query!(
            r#"INSERT INTO system_metrics (id, metric_type, metric_name, value, unit, tags, recorded_at, hostname, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            metric.base.id.to_string(),
            format!("{:?}", metric.metric_type),
            metric.metric_name,
            metric.value,
            metric.unit,
            metric.tags,
            metric.recorded_at.to_rfc3339(),
            metric.hostname,
            metric.base.created_at.to_rfc3339(),
            metric.base.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(metric)
    }

    async fn list_metrics(&self, pool: &SqlitePool, metric_type: Option<MetricType>, limit: i32) -> Result<Vec<SystemMetric>> {
        let rows = if let Some(mt) = metric_type {
            sqlx::query!(
                r#"SELECT id, metric_type, metric_name, value, unit, tags, recorded_at, hostname, created_at, updated_at
                   FROM system_metrics WHERE metric_type = ? ORDER BY recorded_at DESC LIMIT ?"#,
                format!("{:?}", mt), limit
            ).fetch_all(pool).await?
        } else {
            sqlx::query!(
                r#"SELECT id, metric_type, metric_name, value, unit, tags, recorded_at, hostname, created_at, updated_at
                   FROM system_metrics ORDER BY recorded_at DESC LIMIT ?"#,
                limit
            ).fetch_all(pool).await?
        };
        
        Ok(rows.into_iter().map(|r| SystemMetric {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            metric_type: MetricType::Custom,
            metric_name: r.metric_name,
            value: r.value,
            unit: r.unit,
            tags: r.tags,
            recorded_at: chrono::DateTime::parse_from_rfc3339(&r.recorded_at).unwrap().with_timezone(&chrono::Utc),
            hostname: r.hostname,
        }).collect())
    }

    async fn create_health_check(&self, pool: &SqlitePool, check: HealthCheck) -> Result<HealthCheck> {
        sqlx::query!(
            r#"INSERT INTO health_checks (id, name, check_type, endpoint, timeout_seconds, interval_seconds,
               is_active, last_check, last_status, last_response_time_ms, consecutive_failures, last_error, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            check.base.id.to_string(),
            check.name,
            format!("{:?}", check.check_type),
            check.endpoint,
            check.timeout_seconds,
            check.interval_seconds,
            check.is_active as i32,
            check.last_check.map(|d| d.to_rfc3339()),
            check.last_status.map(|s| format!("{:?}", s)),
            check.last_response_time_ms,
            check.consecutive_failures,
            check.last_error,
            check.base.created_at.to_rfc3339(),
            check.base.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(check)
    }

    async fn get_health_check(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<HealthCheck>> {
        let row = sqlx::query!(
            r#"SELECT id, name, check_type, endpoint, timeout_seconds, interval_seconds, is_active,
               last_check, last_status, last_response_time_ms, consecutive_failures, last_error, created_at, updated_at
               FROM health_checks WHERE id = ?"#,
            id.to_string()
        ).fetch_optional(pool).await?;
        
        Ok(row.map(|r| HealthCheck {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: r.name,
            check_type: HealthCheckType::Database,
            endpoint: r.endpoint,
            timeout_seconds: r.timeout_seconds,
            interval_seconds: r.interval_seconds,
            is_active: r.is_active == 1,
            last_check: r.last_check.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            last_status: r.last_status.map(|_| HealthStatus::Healthy),
            last_response_time_ms: r.last_response_time_ms,
            consecutive_failures: r.consecutive_failures,
            last_error: r.last_error,
        }))
    }

    async fn list_health_checks(&self, pool: &SqlitePool) -> Result<Vec<HealthCheck>> {
        let rows = sqlx::query!(
            r#"SELECT id, name, check_type, endpoint, timeout_seconds, interval_seconds, is_active,
               last_check, last_status, last_response_time_ms, consecutive_failures, last_error, created_at, updated_at
               FROM health_checks ORDER BY name"#
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| HealthCheck {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: r.name,
            check_type: HealthCheckType::Database,
            endpoint: r.endpoint,
            timeout_seconds: r.timeout_seconds,
            interval_seconds: r.interval_seconds,
            is_active: r.is_active == 1,
            last_check: r.last_check.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            last_status: r.last_status.map(|_| HealthStatus::Healthy),
            last_response_time_ms: r.last_response_time_ms,
            consecutive_failures: r.consecutive_failures,
            last_error: r.last_error,
        }).collect())
    }

    async fn update_health_check(&self, pool: &SqlitePool, check: HealthCheck) -> Result<HealthCheck> {
        sqlx::query!(
            r#"UPDATE health_checks SET last_check = ?, last_status = ?, last_response_time_ms = ?,
               consecutive_failures = ?, last_error = ?, updated_at = ? WHERE id = ?"#,
            check.last_check.map(|d| d.to_rfc3339()),
            check.last_status.map(|s| format!("{:?}", s)),
            check.last_response_time_ms,
            check.consecutive_failures,
            check.last_error,
            check.base.updated_at.to_rfc3339(),
            check.base.id.to_string(),
        ).execute(pool).await?;
        Ok(check)
    }

    async fn create_health_result(&self, pool: &SqlitePool, result: HealthCheckResult) -> Result<HealthCheckResult> {
        sqlx::query!(
            r#"INSERT INTO health_check_results (id, check_id, status, response_time_ms, message, details, checked_at, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            result.base.id.to_string(),
            result.check_id.to_string(),
            format!("{:?}", result.status),
            result.response_time_ms,
            result.message,
            result.details,
            result.checked_at.to_rfc3339(),
            result.base.created_at.to_rfc3339(),
            result.base.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(result)
    }

    async fn list_health_results(&self, pool: &SqlitePool, check_id: Uuid, limit: i32) -> Result<Vec<HealthCheckResult>> {
        let rows = sqlx::query!(
            r#"SELECT id, check_id, status, response_time_ms, message, details, checked_at, created_at, updated_at
               FROM health_check_results WHERE check_id = ? ORDER BY checked_at DESC LIMIT ?"#,
            check_id.to_string(), limit
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| HealthCheckResult {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            check_id: Uuid::parse_str(&r.check_id).unwrap(),
            status: HealthStatus::Healthy,
            response_time_ms: r.response_time_ms,
            message: r.message,
            details: r.details,
            checked_at: chrono::DateTime::parse_from_rfc3339(&r.checked_at).unwrap().with_timezone(&chrono::Utc),
        }).collect())
    }

    async fn create_alert_rule(&self, pool: &SqlitePool, rule: AlertRule) -> Result<AlertRule> {
        sqlx::query!(
            r#"INSERT INTO alert_rules (id, name, metric_type, metric_name, condition, threshold, duration_minutes,
               severity, notification_channels, is_active, last_triggered, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            rule.base.id.to_string(),
            rule.name,
            format!("{:?}", rule.metric_type),
            rule.metric_name,
            format!("{:?}", rule.condition),
            rule.threshold,
            rule.duration_minutes,
            format!("{:?}", rule.severity),
            rule.notification_channels,
            rule.is_active as i32,
            rule.last_triggered.map(|d| d.to_rfc3339()),
            rule.base.created_at.to_rfc3339(),
            rule.base.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(rule)
    }

    async fn list_alert_rules(&self, pool: &SqlitePool) -> Result<Vec<AlertRule>> {
        let rows = sqlx::query!(
            r#"SELECT id, name, metric_type, metric_name, condition, threshold, duration_minutes,
               severity, notification_channels, is_active, last_triggered, created_at, updated_at
               FROM alert_rules WHERE is_active = 1 ORDER BY name"#
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| AlertRule {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: r.name,
            metric_type: MetricType::Custom,
            metric_name: r.metric_name,
            condition: AlertCondition::GreaterThan,
            threshold: r.threshold,
            duration_minutes: r.duration_minutes,
            severity: AlertSeverity::Warning,
            notification_channels: r.notification_channels,
            is_active: r.is_active == 1,
            last_triggered: r.last_triggered.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
        }).collect())
    }

    async fn create_alert(&self, pool: &SqlitePool, alert: Alert) -> Result<Alert> {
        sqlx::query!(
            r#"INSERT INTO alerts (id, rule_id, status, severity, message, value, threshold, triggered_at,
               acknowledged_at, acknowledged_by, resolved_at, resolution_note, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            alert.base.id.to_string(),
            alert.rule_id.to_string(),
            format!("{:?}", alert.status),
            format!("{:?}", alert.severity),
            alert.message,
            alert.value,
            alert.threshold,
            alert.triggered_at.to_rfc3339(),
            alert.acknowledged_at.map(|d| d.to_rfc3339()),
            alert.acknowledged_by.map(|id| id.to_string()),
            alert.resolved_at.map(|d| d.to_rfc3339()),
            alert.resolution_note,
            alert.base.created_at.to_rfc3339(),
            alert.base.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(alert)
    }

    async fn list_alerts(&self, pool: &SqlitePool, status: Option<AlertStatus>, limit: i32) -> Result<Vec<Alert>> {
        let rows = if let Some(s) = status {
            sqlx::query!(
                r#"SELECT id, rule_id, status, severity, message, value, threshold, triggered_at,
                   acknowledged_at, acknowledged_by, resolved_at, resolution_note, created_at, updated_at
                   FROM alerts WHERE status = ? ORDER BY triggered_at DESC LIMIT ?"#,
                format!("{:?}", s), limit
            ).fetch_all(pool).await?
        } else {
            sqlx::query!(
                r#"SELECT id, rule_id, status, severity, message, value, threshold, triggered_at,
                   acknowledged_at, acknowledged_by, resolved_at, resolution_note, created_at, updated_at
                   FROM alerts ORDER BY triggered_at DESC LIMIT ?"#,
                limit
            ).fetch_all(pool).await?
        };
        
        Ok(rows.into_iter().map(|r| Alert {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            rule_id: Uuid::parse_str(&r.rule_id).unwrap(),
            status: AlertStatus::Firing,
            severity: AlertSeverity::Warning,
            message: r.message,
            value: r.value,
            threshold: r.threshold,
            triggered_at: chrono::DateTime::parse_from_rfc3339(&r.triggered_at).unwrap().with_timezone(&chrono::Utc),
            acknowledged_at: r.acknowledged_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            acknowledged_by: r.acknowledged_by.and_then(|id| Uuid::parse_str(&id).ok()),
            resolved_at: r.resolved_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            resolution_note: r.resolution_note,
        }).collect())
    }

    async fn update_alert(&self, pool: &SqlitePool, alert: Alert) -> Result<Alert> {
        sqlx::query!(
            r#"UPDATE alerts SET status = ?, acknowledged_at = ?, acknowledged_by = ?, resolved_at = ?,
               resolution_note = ?, updated_at = ? WHERE id = ?"#,
            format!("{:?}", alert.status),
            alert.acknowledged_at.map(|d| d.to_rfc3339()),
            alert.acknowledged_by.map(|id| id.to_string()),
            alert.resolved_at.map(|d| d.to_rfc3339()),
            alert.resolution_note,
            alert.base.updated_at.to_rfc3339(),
            alert.base.id.to_string(),
        ).execute(pool).await?;
        Ok(alert)
    }
}

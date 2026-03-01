use async_trait::async_trait;
use sqlx::{SqlitePool, Row};
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
        let id = metric.base.id.to_string();
        let metric_type = format!("{:?}", metric.metric_type);
        let recorded_at = metric.recorded_at.to_rfc3339();
        let created_at = metric.base.created_at.to_rfc3339();
        let updated_at = metric.base.updated_at.to_rfc3339();
        sqlx::query(
            r#"INSERT INTO system_metrics (id, metric_type, metric_name, value, unit, tags, recorded_at, hostname, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(&id)
        .bind(&metric_type)
        .bind(&metric.metric_name)
        .bind(metric.value)
        .bind(&metric.unit)
        .bind(&metric.tags)
        .bind(&recorded_at)
        .bind(&metric.hostname)
        .bind(&created_at)
        .bind(&updated_at)
        .execute(pool).await?;
        Ok(metric)
    }

    async fn list_metrics(&self, pool: &SqlitePool, metric_type: Option<MetricType>, limit: i32) -> Result<Vec<SystemMetric>> {
        if let Some(mt) = metric_type {
            let mt_str = format!("{:?}", mt);
            let rows = sqlx::query(
                r#"SELECT id, metric_type, metric_name, value, unit, tags, recorded_at, hostname, created_at, updated_at
                   FROM system_metrics WHERE metric_type = ? ORDER BY recorded_at DESC LIMIT ?"#
            )
            .bind(&mt_str)
            .bind(limit)
            .fetch_all(pool).await?;
            
            Ok(rows.into_iter().map(|r| SystemMetric {
                base: erp_core::BaseEntity {
                    id: Uuid::parse_str(&r.get::<String, _>("id")).unwrap(),
                    created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                    created_by: None,
                    updated_by: None,
                },
                metric_type: MetricType::Custom,
                metric_name: r.get::<String, _>("metric_name"),
                value: r.get::<f64, _>("value"),
                unit: r.get::<String, _>("unit"),
                tags: r.get::<Option<String>, _>("tags"),
                recorded_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("recorded_at")).unwrap().with_timezone(&chrono::Utc),
                hostname: r.get::<Option<String>, _>("hostname"),
            }).collect())
        } else {
            let rows = sqlx::query(
                r#"SELECT id, metric_type, metric_name, value, unit, tags, recorded_at, hostname, created_at, updated_at
                   FROM system_metrics ORDER BY recorded_at DESC LIMIT ?"#
            )
            .bind(limit)
            .fetch_all(pool).await?;
            
            Ok(rows.into_iter().map(|r| SystemMetric {
                base: erp_core::BaseEntity {
                    id: Uuid::parse_str(&r.get::<String, _>("id")).unwrap(),
                    created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                    created_by: None,
                    updated_by: None,
                },
                metric_type: MetricType::Custom,
                metric_name: r.get::<String, _>("metric_name"),
                value: r.get::<f64, _>("value"),
                unit: r.get::<String, _>("unit"),
                tags: r.get::<Option<String>, _>("tags"),
                recorded_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("recorded_at")).unwrap().with_timezone(&chrono::Utc),
                hostname: r.get::<Option<String>, _>("hostname"),
            }).collect())
        }
    }

    async fn create_health_check(&self, pool: &SqlitePool, check: HealthCheck) -> Result<HealthCheck> {
        let id = check.base.id.to_string();
        let check_type = format!("{:?}", check.check_type);
        let last_check = check.last_check.map(|d| d.to_rfc3339());
        let last_status = check.last_status.as_ref().map(|s| format!("{:?}", s));
        let created_at = check.base.created_at.to_rfc3339();
        let updated_at = check.base.updated_at.to_rfc3339();
        sqlx::query(
            r#"INSERT INTO health_checks (id, name, check_type, endpoint, timeout_seconds, interval_seconds,
               is_active, last_check, last_status, last_response_time_ms, consecutive_failures, last_error, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(&id)
        .bind(&check.name)
        .bind(&check_type)
        .bind(&check.endpoint)
        .bind(check.timeout_seconds)
        .bind(check.interval_seconds)
        .bind(check.is_active as i32)
        .bind(&last_check)
        .bind(&last_status)
        .bind(check.last_response_time_ms)
        .bind(check.consecutive_failures)
        .bind(&check.last_error)
        .bind(&created_at)
        .bind(&updated_at)
        .execute(pool).await?;
        Ok(check)
    }

    async fn get_health_check(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<HealthCheck>> {
        let id_str = id.to_string();
        let row = sqlx::query(
            r#"SELECT id, name, check_type, endpoint, timeout_seconds, interval_seconds, is_active,
               last_check, last_status, last_response_time_ms, consecutive_failures, last_error, created_at, updated_at
               FROM health_checks WHERE id = ?"#
        )
        .bind(&id_str)
        .fetch_optional(pool).await?;
        
        Ok(row.map(|r| {
            let last_status_val: Option<String> = r.get("last_status");
            let last_status = last_status_val.map(|_| HealthStatus::Healthy);
            HealthCheck {
                base: erp_core::BaseEntity {
                    id: Uuid::parse_str(&r.get::<String, _>("id")).unwrap(),
                    created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                    created_by: None,
                    updated_by: None,
                },
                name: r.get::<String, _>("name"),
                check_type: HealthCheckType::Database,
                endpoint: r.get::<Option<String>, _>("endpoint"),
                timeout_seconds: r.get::<i32, _>("timeout_seconds"),
                interval_seconds: r.get::<i32, _>("interval_seconds"),
                is_active: r.get::<i32, _>("is_active") == 1,
                last_check: r.get::<Option<String>, _>("last_check")
                    .and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
                last_status,
                last_response_time_ms: r.get::<Option<i32>, _>("last_response_time_ms").map(|v| v as i64),
                consecutive_failures: r.get::<i32, _>("consecutive_failures"),
                last_error: r.get::<Option<String>, _>("last_error"),
            }
        }))
    }

    async fn list_health_checks(&self, pool: &SqlitePool) -> Result<Vec<HealthCheck>> {
        let rows = sqlx::query(
            r#"SELECT id, name, check_type, endpoint, timeout_seconds, interval_seconds, is_active,
               last_check, last_status, last_response_time_ms, consecutive_failures, last_error, created_at, updated_at
               FROM health_checks ORDER BY name"#
        )
        .fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| {
            let last_status_val: Option<String> = r.get("last_status");
            let last_status = last_status_val.map(|_| HealthStatus::Healthy);
            HealthCheck {
                base: erp_core::BaseEntity {
                    id: Uuid::parse_str(&r.get::<String, _>("id")).unwrap(),
                    created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                    created_by: None,
                    updated_by: None,
                },
                name: r.get::<String, _>("name"),
                check_type: HealthCheckType::Database,
                endpoint: r.get::<Option<String>, _>("endpoint"),
                timeout_seconds: r.get::<i32, _>("timeout_seconds"),
                interval_seconds: r.get::<i32, _>("interval_seconds"),
                is_active: r.get::<i32, _>("is_active") == 1,
                last_check: r.get::<Option<String>, _>("last_check")
                    .and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
                last_status,
                last_response_time_ms: r.get::<Option<i32>, _>("last_response_time_ms").map(|v| v as i64),
                consecutive_failures: r.get::<i32, _>("consecutive_failures"),
                last_error: r.get::<Option<String>, _>("last_error"),
            }
        }).collect())
    }

    async fn update_health_check(&self, pool: &SqlitePool, check: HealthCheck) -> Result<HealthCheck> {
        let last_check = check.last_check.map(|d| d.to_rfc3339());
        let last_status = check.last_status.as_ref().map(|s| format!("{:?}", s));
        let updated_at = check.base.updated_at.to_rfc3339();
        let id = check.base.id.to_string();
        sqlx::query(
            r#"UPDATE health_checks SET last_check = ?, last_status = ?, last_response_time_ms = ?,
               consecutive_failures = ?, last_error = ?, updated_at = ? WHERE id = ?"#
        )
        .bind(&last_check)
        .bind(&last_status)
        .bind(check.last_response_time_ms.map(|v| v as i32))
        .bind(check.consecutive_failures)
        .bind(&check.last_error)
        .bind(&updated_at)
        .bind(&id)
        .execute(pool).await?;
        Ok(check)
    }

    async fn create_health_result(&self, pool: &SqlitePool, result: HealthCheckResult) -> Result<HealthCheckResult> {
        let id = result.base.id.to_string();
        let check_id = result.check_id.to_string();
        let status = format!("{:?}", result.status);
        let checked_at = result.checked_at.to_rfc3339();
        let created_at = result.base.created_at.to_rfc3339();
        let updated_at = result.base.updated_at.to_rfc3339();
        sqlx::query(
            r#"INSERT INTO health_check_results (id, check_id, status, response_time_ms, message, details, checked_at, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(&id)
        .bind(&check_id)
        .bind(&status)
        .bind(result.response_time_ms as i32)
        .bind(&result.message)
        .bind(&result.details)
        .bind(&checked_at)
        .bind(&created_at)
        .bind(&updated_at)
        .execute(pool).await?;
        Ok(result)
    }

    async fn list_health_results(&self, pool: &SqlitePool, check_id: Uuid, limit: i32) -> Result<Vec<HealthCheckResult>> {
        let rows = sqlx::query(
            r#"SELECT id, check_id, status, response_time_ms, message, details, checked_at, created_at, updated_at
               FROM health_check_results WHERE check_id = ? ORDER BY checked_at DESC LIMIT ?"#
        )
        .bind(check_id.to_string())
        .bind(limit)
        .fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| HealthCheckResult {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.get::<String, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            check_id: Uuid::parse_str(&r.get::<String, _>("check_id")).unwrap(),
            status: HealthStatus::Healthy,
            response_time_ms: r.get::<i32, _>("response_time_ms") as i64,
            message: r.get::<Option<String>, _>("message"),
            details: r.get::<Option<String>, _>("details"),
            checked_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("checked_at")).unwrap().with_timezone(&chrono::Utc),
        }).collect())
    }

    async fn create_alert_rule(&self, pool: &SqlitePool, rule: AlertRule) -> Result<AlertRule> {
        let id = rule.base.id.to_string();
        let metric_type = format!("{:?}", rule.metric_type);
        let condition = format!("{:?}", rule.condition);
        let severity = format!("{:?}", rule.severity);
        let last_triggered = rule.last_triggered.map(|d| d.to_rfc3339());
        let created_at = rule.base.created_at.to_rfc3339();
        let updated_at = rule.base.updated_at.to_rfc3339();
        sqlx::query(
            r#"INSERT INTO alert_rules (id, name, metric_type, metric_name, condition, threshold, duration_minutes,
               severity, notification_channels, is_active, last_triggered, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(&id)
        .bind(&rule.name)
        .bind(&metric_type)
        .bind(&rule.metric_name)
        .bind(&condition)
        .bind(rule.threshold)
        .bind(rule.duration_minutes)
        .bind(&severity)
        .bind(&rule.notification_channels)
        .bind(rule.is_active as i32)
        .bind(&last_triggered)
        .bind(&created_at)
        .bind(&updated_at)
        .execute(pool).await?;
        Ok(rule)
    }

    async fn list_alert_rules(&self, pool: &SqlitePool) -> Result<Vec<AlertRule>> {
        let rows = sqlx::query(
            r#"SELECT id, name, metric_type, metric_name, condition, threshold, duration_minutes,
               severity, notification_channels, is_active, last_triggered, created_at, updated_at
               FROM alert_rules WHERE is_active = 1 ORDER BY name"#
        )
        .fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| AlertRule {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.get::<String, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: r.get::<String, _>("name"),
            metric_type: MetricType::Custom,
            metric_name: r.get::<String, _>("metric_name"),
            condition: AlertCondition::GreaterThan,
            threshold: r.get::<f64, _>("threshold"),
            duration_minutes: r.get::<i32, _>("duration_minutes"),
            severity: AlertSeverity::Warning,
            notification_channels: r.get::<Option<String>, _>("notification_channels"),
            is_active: r.get::<i32, _>("is_active") == 1,
            last_triggered: r.get::<Option<String>, _>("last_triggered")
                .and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
        }).collect())
    }

    async fn create_alert(&self, pool: &SqlitePool, alert: Alert) -> Result<Alert> {
        let id = alert.base.id.to_string();
        let rule_id = alert.rule_id.to_string();
        let status = format!("{:?}", alert.status);
        let severity = format!("{:?}", alert.severity);
        let triggered_at = alert.triggered_at.to_rfc3339();
        let acknowledged_at = alert.acknowledged_at.map(|d| d.to_rfc3339());
        let acknowledged_by = alert.acknowledged_by.map(|id| id.to_string());
        let resolved_at = alert.resolved_at.map(|d| d.to_rfc3339());
        let created_at = alert.base.created_at.to_rfc3339();
        let updated_at = alert.base.updated_at.to_rfc3339();
        sqlx::query(
            r#"INSERT INTO alerts (id, rule_id, status, severity, message, value, threshold, triggered_at,
               acknowledged_at, acknowledged_by, resolved_at, resolution_note, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(&id)
        .bind(&rule_id)
        .bind(&status)
        .bind(&severity)
        .bind(&alert.message)
        .bind(alert.value)
        .bind(alert.threshold)
        .bind(&triggered_at)
        .bind(&acknowledged_at)
        .bind(&acknowledged_by)
        .bind(&resolved_at)
        .bind(&alert.resolution_note)
        .bind(&created_at)
        .bind(&updated_at)
        .execute(pool).await?;
        Ok(alert)
    }

    async fn list_alerts(&self, pool: &SqlitePool, status: Option<AlertStatus>, limit: i32) -> Result<Vec<Alert>> {
        if let Some(s) = status {
            let s_str = format!("{:?}", s);
            let rows = sqlx::query(
                r#"SELECT id, rule_id, status, severity, message, value, threshold, triggered_at,
                   acknowledged_at, acknowledged_by, resolved_at, resolution_note, created_at, updated_at
                   FROM alerts WHERE status = ? ORDER BY triggered_at DESC LIMIT ?"#
            )
            .bind(&s_str)
            .bind(limit)
            .fetch_all(pool).await?;
            
            Ok(rows.into_iter().map(|r| Alert {
                base: erp_core::BaseEntity {
                    id: Uuid::parse_str(&r.get::<String, _>("id")).unwrap(),
                    created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                    created_by: None,
                    updated_by: None,
                },
                rule_id: Uuid::parse_str(&r.get::<String, _>("rule_id")).unwrap(),
                status: AlertStatus::Firing,
                severity: AlertSeverity::Warning,
                message: r.get::<String, _>("message"),
                value: r.get::<f64, _>("value"),
                threshold: r.get::<f64, _>("threshold"),
                triggered_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("triggered_at")).unwrap().with_timezone(&chrono::Utc),
                acknowledged_at: r.get::<Option<String>, _>("acknowledged_at")
                    .and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
                acknowledged_by: r.get::<Option<String>, _>("acknowledged_by")
                    .and_then(|id| Uuid::parse_str(&id).ok()),
                resolved_at: r.get::<Option<String>, _>("resolved_at")
                    .and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
                resolution_note: r.get::<Option<String>, _>("resolution_note"),
            }).collect())
        } else {
            let rows = sqlx::query(
                r#"SELECT id, rule_id, status, severity, message, value, threshold, triggered_at,
                   acknowledged_at, acknowledged_by, resolved_at, resolution_note, created_at, updated_at
                   FROM alerts ORDER BY triggered_at DESC LIMIT ?"#
            )
            .bind(limit)
            .fetch_all(pool).await?;
            
            Ok(rows.into_iter().map(|r| Alert {
                base: erp_core::BaseEntity {
                    id: Uuid::parse_str(&r.get::<String, _>("id")).unwrap(),
                    created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                    created_by: None,
                    updated_by: None,
                },
                rule_id: Uuid::parse_str(&r.get::<String, _>("rule_id")).unwrap(),
                status: AlertStatus::Firing,
                severity: AlertSeverity::Warning,
                message: r.get::<String, _>("message"),
                value: r.get::<f64, _>("value"),
                threshold: r.get::<f64, _>("threshold"),
                triggered_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("triggered_at")).unwrap().with_timezone(&chrono::Utc),
                acknowledged_at: r.get::<Option<String>, _>("acknowledged_at")
                    .and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
                acknowledged_by: r.get::<Option<String>, _>("acknowledged_by")
                    .and_then(|id| Uuid::parse_str(&id).ok()),
                resolved_at: r.get::<Option<String>, _>("resolved_at")
                    .and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|dt| dt.with_timezone(&chrono::Utc))),
                resolution_note: r.get::<Option<String>, _>("resolution_note"),
            }).collect())
        }
    }

    async fn update_alert(&self, pool: &SqlitePool, alert: Alert) -> Result<Alert> {
        let status = format!("{:?}", alert.status);
        let acknowledged_at = alert.acknowledged_at.map(|d| d.to_rfc3339());
        let acknowledged_by = alert.acknowledged_by.map(|id| id.to_string());
        let resolved_at = alert.resolved_at.map(|d| d.to_rfc3339());
        let updated_at = alert.base.updated_at.to_rfc3339();
        let id = alert.base.id.to_string();
        sqlx::query(
            r#"UPDATE alerts SET status = ?, acknowledged_at = ?, acknowledged_by = ?, resolved_at = ?,
               resolution_note = ?, updated_at = ? WHERE id = ?"#
        )
        .bind(&status)
        .bind(&acknowledged_at)
        .bind(&acknowledged_by)
        .bind(&resolved_at)
        .bind(&alert.resolution_note)
        .bind(&updated_at)
        .bind(&id)
        .execute(pool).await?;
        Ok(alert)
    }
}

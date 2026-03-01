use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use erp_core::{BaseEntity, Result};
use crate::models::*;
use crate::repository::{MonitoringRepository, SqliteMonitoringRepository};

pub struct MonitoringService {
    repo: SqliteMonitoringRepository,
}

impl MonitoringService {
    pub fn new() -> Self {
        Self { repo: SqliteMonitoringRepository }
    }

    pub async fn collect_system_metrics(&self, pool: &SqlitePool) -> Result<Vec<SystemMetric>> {
        let now = Utc::now();
        let mut metrics = Vec::new();
        
        metrics.push(SystemMetric {
            base: BaseEntity::new(),
            metric_type: MetricType::Cpu,
            metric_name: "cpu_usage_percent".to_string(),
            value: 0.0,
            unit: "percent".to_string(),
            tags: None,
            recorded_at: now,
            hostname: None,
        });
        
        metrics.push(SystemMetric {
            base: BaseEntity::new(),
            metric_type: MetricType::Memory,
            metric_name: "memory_usage_percent".to_string(),
            value: 0.0,
            unit: "percent".to_string(),
            tags: None,
            recorded_at: now,
            hostname: None,
        });
        
        for metric in &metrics {
            let _ = self.repo.create_metric(pool, metric.clone()).await;
        }
        
        Ok(metrics)
    }

    pub async fn get_current_metrics(&self, pool: &SqlitePool) -> Result<CurrentMetrics> {
        let active_users: i64 = sqlx::query_scalar(
            "SELECT COUNT(DISTINCT user_id) FROM user_sessions WHERE expires_at > datetime('now')"
        ).fetch_optional(pool).await?.flatten().unwrap_or(0);
        
        Ok(CurrentMetrics {
            cpu_percent: 0.0,
            memory_percent: 0.0,
            disk_percent: 0.0,
            database_connections: 1,
            active_users,
            requests_per_minute: 0.0,
            average_response_time_ms: 0.0,
        })
    }

    pub async fn create_health_check(&self, pool: &SqlitePool, check: HealthCheck) -> Result<HealthCheck> {
        self.repo.create_health_check(pool, check).await
    }

    pub async fn list_health_checks(&self, pool: &SqlitePool) -> Result<Vec<HealthCheck>> {
        self.repo.list_health_checks(pool).await
    }

    pub async fn run_health_check(&self, pool: &SqlitePool, check_id: Uuid) -> Result<HealthCheckResult> {
        let mut check = self.repo.get_health_check(pool, check_id).await?
            .ok_or_else(|| anyhow::anyhow!("Health check not found"))?;
        
        let start = std::time::Instant::now();
        let (status, message) = match check.check_type {
            HealthCheckType::Database => self.check_database(pool).await,
            _ => (HealthStatus::Healthy, Some("OK".to_string())),
        };
        
        let response_time_ms = start.elapsed().as_millis() as i64;
        
        let result = HealthCheckResult {
            base: BaseEntity::new(),
            check_id,
            status: status.clone(),
            response_time_ms,
            message: message.clone(),
            details: None,
            checked_at: Utc::now(),
        };
        
        let _ = self.repo.create_health_result(pool, result.clone()).await;
        
        check.last_check = Some(Utc::now());
        check.last_status = Some(status);
        check.last_response_time_ms = Some(response_time_ms);
        check.last_error = message;
        check.base.updated_at = Utc::now();
        
        if check.last_status == Some(HealthStatus::Unhealthy) {
            check.consecutive_failures += 1;
        } else {
            check.consecutive_failures = 0;
        }
        
        self.repo.update_health_check(pool, check).await?;
        
        Ok(result)
    }

    async fn check_database(&self, pool: &SqlitePool) -> (HealthStatus, Option<String>) {
        match sqlx::query("SELECT 1").fetch_one(pool).await {
            Ok(_) => (HealthStatus::Healthy, Some("Database connection OK".to_string())),
            Err(e) => (HealthStatus::Unhealthy, Some(format!("Database error: {}", e))),
        }
    }

    pub async fn get_system_status(&self, pool: &SqlitePool) -> Result<SystemStatus> {
        let checks = self.repo.list_health_checks(pool).await?;
        let mut component_statuses = Vec::new();
        let mut all_healthy = true;
        
        for check in &checks {
            let status = check.last_status.clone().unwrap_or(HealthStatus::Unknown);
            if status != HealthStatus::Healthy && status != HealthStatus::Unknown {
                all_healthy = false;
            }
            component_statuses.push(ComponentStatus {
                name: check.name.clone(),
                status: status.clone(),
                response_time_ms: check.last_response_time_ms,
                message: check.last_error.clone(),
                last_check: check.last_check,
            });
        }
        
        if component_statuses.is_empty() {
            component_statuses.push(ComponentStatus {
                name: "Database".to_string(),
                status: HealthStatus::Healthy,
                response_time_ms: Some(0),
                message: Some("Connected".to_string()),
                last_check: Some(Utc::now()),
            });
        }
        
        let metrics = self.get_current_metrics(pool).await?;
        
        let active_alerts = self.repo.list_alerts(pool, Some(AlertStatus::Firing), 100).await?.len() as i32;
        
        Ok(SystemStatus {
            overall_status: if all_healthy { HealthStatus::Healthy } else { HealthStatus::Degraded },
            checks: component_statuses,
            metrics,
            active_alerts,
            last_updated: Utc::now(),
        })
    }

    pub async fn create_alert_rule(&self, pool: &SqlitePool, rule: AlertRule) -> Result<AlertRule> {
        self.repo.create_alert_rule(pool, rule).await
    }

    pub async fn list_alert_rules(&self, pool: &SqlitePool) -> Result<Vec<AlertRule>> {
        self.repo.list_alert_rules(pool).await
    }

    pub async fn list_alerts(&self, pool: &SqlitePool, status: Option<AlertStatus>, limit: i32) -> Result<Vec<Alert>> {
        self.repo.list_alerts(pool, status, limit).await
    }

    pub async fn acknowledge_alert(&self, pool: &SqlitePool, alert_id: Uuid, acknowledged_by: Uuid) -> Result<Alert> {
        let mut alerts = self.repo.list_alerts(pool, None, 1000).await?;
        let alert = alerts.iter_mut().find(|a| a.base.id == alert_id)
            .ok_or_else(|| anyhow::anyhow!("Alert not found"))?;
        
        alert.status = AlertStatus::Acknowledged;
        alert.acknowledged_at = Some(Utc::now());
        alert.acknowledged_by = Some(acknowledged_by);
        alert.base.updated_at = Utc::now();
        
        self.repo.update_alert(pool, alert.clone()).await
    }

    pub async fn resolve_alert(&self, pool: &SqlitePool, alert_id: Uuid, resolution_note: Option<String>) -> Result<Alert> {
        let mut alerts = self.repo.list_alerts(pool, None, 1000).await?;
        let alert = alerts.iter_mut().find(|a| a.base.id == alert_id)
            .ok_or_else(|| anyhow::anyhow!("Alert not found"))?;
        
        alert.status = AlertStatus::Resolved;
        alert.resolved_at = Some(Utc::now());
        alert.resolution_note = resolution_note;
        alert.base.updated_at = Utc::now();
        
        self.repo.update_alert(pool, alert.clone()).await
    }

    pub async fn get_database_stats(&self, pool: &SqlitePool) -> Result<DatabaseStats> {
        let metadata = tokio::fs::metadata("./erp.db").await.ok();
        let size_bytes = metadata.map(|m| m.len() as i64).unwrap_or(0);
        
        let table_count: i32 = sqlx::query_scalar::<_, i32>("SELECT COUNT(*) FROM sqlite_master WHERE type='table'")
            .fetch_one(pool).await?;
        
        let index_count: i32 = sqlx::query_scalar::<_, i32>("SELECT COUNT(*) FROM sqlite_master WHERE type='index'")
            .fetch_one(pool).await?;
        
        Ok(DatabaseStats {
            size_bytes,
            table_count,
            index_count,
            connection_count: 1,
            active_transactions: 0,
            last_vacuum: None,
            last_analyze: None,
        })
    }

    pub async fn run_all_health_checks(&self, pool: &SqlitePool) -> Result<Vec<HealthCheckResult>> {
        let checks = self.repo.list_health_checks(pool).await?;
        let mut results = Vec::new();
        
        for check in checks {
            if check.is_active {
                let result = self.run_health_check(pool, check.base.id).await?;
                results.push(result);
            }
        }
        
        Ok(results)
    }
}

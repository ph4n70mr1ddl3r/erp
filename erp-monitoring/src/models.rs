use chrono::{DateTime, Utc};
use erp_core::BaseEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetric {
    pub base: BaseEntity,
    pub metric_type: MetricType,
    pub metric_name: String,
    pub value: f64,
    pub unit: String,
    pub tags: Option<String>,
    pub recorded_at: DateTime<Utc>,
    pub hostname: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MetricType {
    Cpu,
    Memory,
    Disk,
    Network,
    Database,
    Application,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub base: BaseEntity,
    pub name: String,
    pub check_type: HealthCheckType,
    pub endpoint: Option<String>,
    pub timeout_seconds: i32,
    pub interval_seconds: i32,
    pub is_active: bool,
    pub last_check: Option<DateTime<Utc>>,
    pub last_status: Option<HealthStatus>,
    pub last_response_time_ms: Option<i64>,
    pub consecutive_failures: i32,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum HealthCheckType {
    Database,
    Http,
    Tcp,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    pub base: BaseEntity,
    pub check_id: Uuid,
    pub status: HealthStatus,
    pub response_time_ms: i64,
    pub message: Option<String>,
    pub details: Option<String>,
    pub checked_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    pub base: BaseEntity,
    pub name: String,
    pub metric_type: MetricType,
    pub metric_name: String,
    pub condition: AlertCondition,
    pub threshold: f64,
    pub duration_minutes: i32,
    pub severity: AlertSeverity,
    pub notification_channels: Option<String>,
    pub is_active: bool,
    pub last_triggered: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AlertCondition {
    GreaterThan,
    LessThan,
    Equals,
    NotEquals,
    Missing,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
    Emergency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub base: BaseEntity,
    pub rule_id: Uuid,
    pub status: AlertStatus,
    pub severity: AlertSeverity,
    pub message: String,
    pub value: f64,
    pub threshold: f64,
    pub triggered_at: DateTime<Utc>,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub acknowledged_by: Option<Uuid>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolution_note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AlertStatus {
    Firing,
    Acknowledged,
    Resolved,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatus {
    pub overall_status: HealthStatus,
    pub checks: Vec<ComponentStatus>,
    pub metrics: CurrentMetrics,
    pub active_alerts: i32,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentStatus {
    pub name: String,
    pub status: HealthStatus,
    pub response_time_ms: Option<i64>,
    pub message: Option<String>,
    pub last_check: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentMetrics {
    pub cpu_percent: f64,
    pub memory_percent: f64,
    pub disk_percent: f64,
    pub database_connections: i64,
    pub active_users: i64,
    pub requests_per_minute: f64,
    pub average_response_time_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseStats {
    pub size_bytes: i64,
    pub table_count: i32,
    pub index_count: i32,
    pub connection_count: i32,
    pub active_transactions: i32,
    pub last_vacuum: Option<DateTime<Utc>>,
    pub last_analyze: Option<DateTime<Utc>>,
}

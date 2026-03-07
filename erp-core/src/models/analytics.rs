use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::common::Status;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DashboardType {
    Executive,
    Operational,
    Financial,
    Sales,
    Inventory,
    Manufacturing,
    HR,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum WidgetType {
    Chart,
    Table,
    KPI,
    Gauge,
    Map,
    Text,
    Image,
    Counter,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum KPICategory {
    Financial,
    Operational,
    Sales,
    Customer,
    HR,
    Quality,
    Efficiency,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RefreshFrequency {
    RealTime,
    Hourly,
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AlertType {
    System,
    Business,
    Threshold,
    Anomaly,
    Scheduled,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum Severity {
    Info,
    Warning,
    Critical,
    Emergency,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TrendDirection {
    Up,
    Down,
    Flat,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ForecastModelType {
    LinearRegression,
    MovingAverage,
    ExponentialSmoothing,
    ARIMA,
    Prophet,
    NeuralNetwork,
    Ensemble,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dashboard {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub dashboard_type: DashboardType,
    pub is_default: bool,
    pub layout: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardWidget {
    pub id: Uuid,
    pub dashboard_id: Uuid,
    pub widget_type: WidgetType,
    pub title: String,
    pub data_source: String,
    pub query_text: Option<String>,
    pub refresh_interval: i32,
    pub position_x: i32,
    pub position_y: i32,
    pub width: i32,
    pub height: i32,
    pub config: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KPIDefinition {
    pub id: Uuid,
    pub kpi_code: String,
    pub name: String,
    pub description: Option<String>,
    pub category: KPICategory,
    pub unit: Option<String>,
    pub target_value: Option<f64>,
    pub warning_threshold: Option<f64>,
    pub critical_threshold: Option<f64>,
    pub calculation_formula: Option<String>,
    pub data_source: Option<String>,
    pub refresh_frequency: RefreshFrequency,
    pub owner: Option<Uuid>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KPIValue {
    pub id: Uuid,
    pub kpi_id: Uuid,
    pub period: String,
    pub value: f64,
    pub target: Option<f64>,
    pub variance: Option<f64>,
    pub variance_percent: Option<f64>,
    pub trend: Option<TrendDirection>,
    pub calculated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: Uuid,
    pub alert_type: AlertType,
    pub severity: Severity,
    pub title: String,
    pub message: String,
    pub source_entity: Option<String>,
    pub source_id: Option<Uuid>,
    pub rule_id: Option<Uuid>,
    pub acknowledged: bool,
    pub acknowledged_by: Option<Uuid>,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub entity_type: String,
    pub condition_field: String,
    pub operator: String,
    pub threshold_value: String,
    pub severity: Severity,
    pub notification_channels: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastModel {
    pub id: Uuid,
    pub name: String,
    pub model_type: ForecastModelType,
    pub target_entity: String,
    pub features: Option<String>,
    pub parameters: Option<String>,
    pub accuracy_score: Option<f64>,
    pub last_trained: Option<DateTime<Utc>>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prediction {
    pub id: Uuid,
    pub model_id: Uuid,
    pub entity_id: Option<Uuid>,
    pub prediction_date: DateTime<Utc>,
    pub predicted_value: f64,
    pub confidence_lower: Option<f64>,
    pub confidence_upper: Option<f64>,
    pub actual_value: Option<f64>,
    pub created_at: DateTime<Utc>,
}

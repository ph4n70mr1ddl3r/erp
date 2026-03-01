use chrono::{DateTime, Utc};
use erp_core::BaseEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum KPIType {
    Counter,
    Gauge,
    Percentage,
    Currency,
    Ratio,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AggregationType {
    Sum,
    Average,
    Min,
    Max,
    Count,
    Last,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KPI {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub category: String,
    pub kpi_type: String,
    pub aggregation: String,
    pub data_source: String,
    pub query: Option<String>,
    pub target_value: Option<f64>,
    pub warning_threshold: Option<f64>,
    pub critical_threshold: Option<f64>,
    pub unit: Option<String>,
    pub refresh_interval_seconds: i32,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum WidgetType {
    LineChart,
    BarChart,
    PieChart,
    Gauge,
    Number,
    Table,
    Heatmap,
    TreeMap,
    ScatterPlot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dashboard {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: Uuid,
    pub is_default: bool,
    pub is_public: bool,
    pub layout_config: serde_json::Value,
    pub refresh_interval_seconds: i32,
    pub filters: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardWidget {
    pub id: Uuid,
    pub dashboard_id: Uuid,
    pub kpi_id: Option<Uuid>,
    pub widget_type: String,
    pub title: String,
    pub position_x: i32,
    pub position_y: i32,
    pub width: i32,
    pub height: i32,
    pub config: serde_json::Value,
    pub data_source: Option<String>,
    pub custom_query: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub category: String,
    pub query: String,
    pub parameters: Option<serde_json::Value>,
    pub columns: serde_json::Value,
    pub chart_config: Option<serde_json::Value>,
    pub is_scheduled: bool,
    pub schedule_cron: Option<String>,
    pub last_run_at: Option<DateTime<Utc>>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

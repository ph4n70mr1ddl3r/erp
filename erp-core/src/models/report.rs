use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportDefinition {
    pub id: Uuid,
    pub report_code: String,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub data_source: String,
    pub query_text: String,
    pub parameters: Option<String>,
    pub columns: Option<String>,
    pub filters: Option<String>,
    pub sorting: Option<String>,
    pub grouping: Option<String>,
    pub chart_type: Option<String>,
    pub is_scheduled: bool,
    pub schedule_cron: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReportExecutionStatus {
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportExecution {
    pub id: Uuid,
    pub report_id: Uuid,
    pub parameters: Option<String>,
    pub row_count: Option<i32>,
    pub file_path: Option<String>,
    pub file_format: Option<String>,
    pub file_size: Option<i32>,
    pub execution_time_ms: Option<i32>,
    pub status: ReportExecutionStatus,
    pub error_message: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

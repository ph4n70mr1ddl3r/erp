use chrono::{DateTime, NaiveDate, Utc};
use erp_core::BaseEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ScheduleFrequency {
    Once,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReportFormat {
    PDF,
    Excel,
    CSV,
    HTML,
    JSON,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DeliveryMethod {
    Email,
    Download,
    FTP,
    SFTP,
    S3,
    SharePoint,
    Webhook,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ScheduleStatus {
    Active,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ExecutionStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
    Timeout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSchedule {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub report_type: String,
    pub report_config: serde_json::Value,
    pub frequency: ScheduleFrequency,
    pub cron_expression: Option<String>,
    pub next_run_at: Option<DateTime<Utc>>,
    pub last_run_at: Option<DateTime<Utc>>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub timezone: String,
    pub format: ReportFormat,
    pub delivery_method: DeliveryMethod,
    pub delivery_config: Option<serde_json::Value>,
    pub recipients: Vec<String>,
    pub cc_recipients: Option<Vec<String>>,
    pub bcc_recipients: Option<Vec<String>>,
    pub email_subject: Option<String>,
    pub email_body: Option<String>,
    pub include_attachment: bool,
    pub compress_output: bool,
    pub compression_format: Option<String>,
    pub max_file_size_mb: Option<i32>,
    pub retry_on_failure: bool,
    pub max_retries: i32,
    pub retry_interval_minutes: i32,
    pub notify_on_success: bool,
    pub notify_on_failure: bool,
    pub notification_recipients: Option<Vec<String>>,
    pub status: ScheduleStatus,
    pub priority: i32,
    pub tags: Option<Vec<String>>,
    pub owner_id: Uuid,
    pub department_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleExecution {
    pub base: BaseEntity,
    pub schedule_id: Uuid,
    pub execution_number: i64,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i64>,
    pub status: ExecutionStatus,
    pub report_url: Option<String>,
    pub file_path: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub record_count: Option<i64>,
    pub error_message: Option<String>,
    pub error_stack: Option<String>,
    pub retry_count: i32,
    pub delivery_attempts: i32,
    pub delivery_status: Option<String>,
    pub delivery_error: Option<String>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub delivery_details: Option<serde_json::Value>,
    pub parameters: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSubscription {
    pub base: BaseEntity,
    pub schedule_id: Uuid,
    pub user_id: Uuid,
    pub email: String,
    pub delivery_method: DeliveryMethod,
    pub format: Option<ReportFormat>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportDistribution {
    pub base: BaseEntity,
    pub execution_id: Uuid,
    pub recipient_email: String,
    pub recipient_name: Option<String>,
    pub delivery_method: DeliveryMethod,
    pub delivered_at: Option<DateTime<Utc>>,
    pub opened_at: Option<DateTime<Utc>>,
    pub downloaded_at: Option<DateTime<Utc>>,
    pub status: String,
    pub error_message: Option<String>,
    pub external_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleAudit {
    pub id: Uuid,
    pub schedule_id: Uuid,
    pub execution_id: Option<Uuid>,
    pub action: String,
    pub old_values: Option<serde_json::Value>,
    pub new_values: Option<serde_json::Value>,
    pub performed_by: Uuid,
    pub performed_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub notes: Option<String>,
}

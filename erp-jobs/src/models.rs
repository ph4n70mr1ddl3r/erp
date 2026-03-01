use chrono::{DateTime, NaiveTime, Utc};
use erp_core::BaseEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum JobStatus {
    Pending,
    Scheduled,
    Running,
    Completed,
    Failed,
    Cancelled,
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum JobType {
    OneTime,
    Recurring,
    Cron,
    EventTriggered,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum JobPriority {
    Low,
    Normal,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ScheduledJob {
    #[sqlx(flatten)]
    pub base: BaseEntity,
    pub name: String,
    pub job_type: JobType,
    pub handler: String,
    pub payload: Option<serde_json::Value>,
    pub priority: JobPriority,
    pub cron_expression: Option<String>,
    pub interval_seconds: Option<i64>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub next_run_at: Option<DateTime<Utc>>,
    pub last_run_at: Option<DateTime<Utc>>,
    pub last_success_at: Option<DateTime<Utc>>,
    pub last_failure_at: Option<DateTime<Utc>>,
    pub status: JobStatus,
    pub run_count: i64,
    pub success_count: i64,
    pub failure_count: i64,
    pub max_retries: i32,
    pub retry_count: i32,
    pub retry_delay_seconds: i32,
    pub timeout_seconds: i32,
    pub last_error: Option<String>,
    pub last_duration_ms: Option<i64>,
    pub avg_duration_ms: Option<i64>,
    pub tags: Option<String>,
    pub created_by: Option<Uuid>,
    pub locked_by: Option<String>,
    pub locked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct JobExecution {
    #[sqlx(flatten)]
    pub base: BaseEntity,
    pub job_id: Uuid,
    pub execution_number: i64,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration_ms: Option<i64>,
    pub status: ExecutionStatus,
    pub result: Option<serde_json::Value>,
    pub error_message: Option<String>,
    pub error_stack_trace: Option<String>,
    pub retry_of_id: Option<Uuid>,
    pub retry_number: i32,
    pub worker_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ExecutionStatus {
    Running,
    Completed,
    Failed,
    Cancelled,
    Timeout,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobQueue {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub max_concurrent_jobs: i32,
    pub current_jobs: i32,
    pub total_processed: i64,
    pub total_failed: i64,
    pub avg_wait_time_ms: Option<i64>,
    pub avg_process_time_ms: Option<i64>,
    pub status: QueueStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum QueueStatus {
    Active,
    Paused,
    Stopped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobWorker {
    pub id: Uuid,
    pub worker_id: String,
    pub queue_name: String,
    pub hostname: String,
    pub pid: i32,
    pub status: WorkerStatus,
    pub current_job_id: Option<Uuid>,
    pub jobs_processed: i64,
    pub jobs_failed: i64,
    pub started_at: DateTime<Utc>,
    pub last_heartbeat: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum WorkerStatus {
    Idle,
    Busy,
    Stopped,
    Crashed,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct JobSchedule {
    #[sqlx(flatten)]
    pub base: BaseEntity,
    pub name: String,
    pub job_template_id: Option<Uuid>,
    pub job_name: String,
    pub handler: String,
    pub default_payload: Option<serde_json::Value>,
    pub schedule_type: ScheduleType,
    pub cron_expression: Option<String>,
    pub interval_minutes: Option<i32>,
    pub specific_times: Option<String>,
    pub run_on_days: Option<String>,
    pub timezone: String,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub next_scheduled_run: Option<DateTime<Utc>>,
    pub last_run: Option<DateTime<Utc>>,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ScheduleType {
    Cron,
    Interval,
    Daily,
    Weekly,
    Monthly,
    SpecificTimes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobDependency {
    pub id: Uuid,
    pub job_id: Uuid,
    pub depends_on_job_id: Uuid,
    pub dependency_type: DependencyType,
    pub satisfied: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DependencyType {
    OnSuccess,
    OnFailure,
    OnCompletion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobLock {
    pub id: Uuid,
    pub resource_key: String,
    pub job_id: Uuid,
    pub locked_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobMetric {
    pub id: Uuid,
    pub date: chrono::NaiveDate,
    pub hour: i32,
    pub queue_name: String,
    pub jobs_submitted: i64,
    pub jobs_completed: i64,
    pub jobs_failed: i64,
    pub avg_wait_time_ms: i64,
    pub avg_process_time_ms: i64,
    pub max_concurrent: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobTemplate {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub handler: String,
    pub default_payload: Option<serde_json::Value>,
    pub default_priority: JobPriority,
    pub default_timeout_seconds: i32,
    pub default_max_retries: i32,
    pub tags: Option<String>,
    pub status: erp_core::Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkJobRequest {
    pub base: BaseEntity,
    pub request_name: String,
    pub job_handler: String,
    pub payloads: Vec<serde_json::Value>,
    pub priority: JobPriority,
    pub status: BulkRequestStatus,
    pub total_jobs: i32,
    pub created_jobs: i32,
    pub completed_jobs: i32,
    pub failed_jobs: i32,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BulkRequestStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

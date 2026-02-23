use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum AutomationType {
    Scheduled,
    EventDriven,
    Trigger,
    Webhook,
    API,
    Manual,
    Recurring,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum AutomationStatus {
    Draft,
    Active,
    Paused,
    Disabled,
    Archived,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationWorkflow {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub category: String,
    pub automation_type: AutomationType,
    pub trigger_config: String,
    pub conditions: Option<String>,
    pub actions: String,
    pub error_handling: Option<String>,
    pub retry_policy: Option<String>,
    pub timeout_seconds: i32,
    pub max_concurrent_runs: i32,
    pub priority: i32,
    pub status: AutomationStatus,
    pub version: i32,
    pub last_modified_by: Option<Uuid>,
    pub last_modified_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    pub published_by: Option<Uuid>,
    pub schedule_cron: Option<String>,
    pub schedule_timezone: Option<String>,
    pub next_run_at: Option<DateTime<Utc>>,
    pub last_run_at: Option<DateTime<Utc>>,
    pub last_run_status: Option<String>,
    pub total_runs: i64,
    pub successful_runs: i64,
    pub failed_runs: i64,
    pub avg_duration_ms: Option<i64>,
    pub tags: Option<String>,
    pub owner_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub step_number: i32,
    pub step_name: String,
    pub step_type: WorkflowStepType,
    pub config: String,
    pub input_mapping: Option<String>,
    pub output_mapping: Option<String>,
    pub condition: Option<String>,
    pub error_handler: Option<String>,
    pub retry_count: i32,
    pub timeout_seconds: i32,
    pub on_success: Option<String>,
    pub on_failure: Option<String>,
    pub parallel_group: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum WorkflowStepType {
    Action,
    Condition,
    Loop,
    Parallel,
    SubWorkflow,
    Delay,
    Webhook,
    APICall,
    DatabaseQuery,
    Email,
    Notification,
    Transform,
    Script,
    AIInference,
    Approval,
    WaitForEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecution {
    pub base: BaseEntity,
    pub workflow_id: Uuid,
    pub execution_number: String,
    pub trigger_type: TriggerType,
    pub trigger_data: Option<String>,
    pub input_data: Option<String>,
    pub output_data: Option<String>,
    pub status: ExecutionStatus,
    pub current_step: Option<i32>,
    pub total_steps: i32,
    pub completed_steps: i32,
    pub progress_percent: i32,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration_ms: Option<i64>,
    pub error_step: Option<i32>,
    pub error_message: Option<String>,
    pub error_stack: Option<String>,
    pub retry_count: i32,
    pub parent_execution_id: Option<Uuid>,
    pub correlation_id: Option<String>,
    pub variables: Option<String>,
    pub checkpoint_data: Option<String>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum TriggerType {
    Manual,
    Schedule,
    Webhook,
    API,
    Event,
    Condition,
    Parent,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum ExecutionStatus {
    Pending,
    Running,
    Paused,
    Waiting,
    Completed,
    Failed,
    Cancelled,
    Timeout,
    Retrying,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepExecution {
    pub id: Uuid,
    pub execution_id: Uuid,
    pub step_id: Uuid,
    pub step_number: i32,
    pub status: ExecutionStatus,
    pub input_data: Option<String>,
    pub output_data: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration_ms: Option<i64>,
    pub error_message: Option<String>,
    pub retry_count: i32,
    pub attempts: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationTrigger {
    pub base: BaseEntity,
    pub name: String,
    pub trigger_type: TriggerType,
    pub entity_type: Option<String>,
    pub event_name: Option<String>,
    pub condition: Option<String>,
    pub workflow_id: Uuid,
    pub is_active: bool,
    pub priority: i32,
    pub cooldown_seconds: i32,
    pub last_triggered_at: Option<DateTime<Utc>>,
    pub trigger_count: i64,
    pub config: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEndpoint {
    pub base: BaseEntity,
    pub name: String,
    pub endpoint_path: String,
    pub workflow_id: Uuid,
    pub authentication_type: WebhookAuthType,
    pub authentication_config: Option<String>,
    pub allowed_ips: Option<String>,
    pub rate_limit_per_minute: i32,
    pub timeout_seconds: i32,
    pub retry_policy: Option<String>,
    pub is_active: bool,
    pub verify_ssl: bool,
    pub secret_key: Option<String>,
    pub total_requests: i64,
    pub successful_requests: i64,
    pub failed_requests: i64,
    pub last_request_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum WebhookAuthType {
    None,
    APIKey,
    BasicAuth,
    HMAC,
    OAuth2,
    JWT,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookRequest {
    pub id: Uuid,
    pub endpoint_id: Uuid,
    pub request_id: String,
    pub method: String,
    pub headers: Option<String>,
    pub query_params: Option<String>,
    pub body: Option<String>,
    pub content_type: Option<String>,
    pub source_ip: Option<String>,
    pub user_agent: Option<String>,
    pub execution_id: Option<Uuid>,
    pub response_status: Option<i32>,
    pub response_body: Option<String>,
    pub processing_time_ms: Option<i64>,
    pub error_message: Option<String>,
    pub received_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledJob {
    pub base: BaseEntity,
    pub name: String,
    pub job_type: String,
    pub description: Option<String>,
    pub schedule_cron: String,
    pub timezone: String,
    pub workflow_id: Option<Uuid>,
    pub job_config: Option<String>,
    pub parameters: Option<String>,
    pub is_active: bool,
    pub misfire_policy: MisfirePolicy,
    pub last_run_at: Option<DateTime<Utc>>,
    pub last_run_status: Option<String>,
    pub last_duration_ms: Option<i64>,
    pub next_run_at: Option<DateTime<Utc>>,
    pub run_count: i64,
    pub failure_count: i64,
    pub consecutive_failures: i32,
    pub max_consecutive_failures: i32,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum MisfirePolicy {
    RunImmediately,
    RunOnce,
    Ignore,
    DisableJob,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobExecution {
    pub base: BaseEntity,
    pub scheduled_job_id: Uuid,
    pub execution_number: String,
    pub scheduled_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration_ms: Option<i64>,
    pub status: ExecutionStatus,
    pub trigger_type: TriggerType,
    pub input_parameters: Option<String>,
    pub output_data: Option<String>,
    pub error_message: Option<String>,
    pub error_stack: Option<String>,
    pub retry_count: i32,
    pub execution_node: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionTemplate {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub category: String,
    pub description: Option<String>,
    pub action_type: ActionType,
    pub icon: Option<String>,
    pub input_schema: Option<String>,
    pub output_schema: Option<String>,
    pub config_schema: Option<String>,
    pub default_config: Option<String>,
    pub documentation_url: Option<String>,
    pub is_builtin: bool,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum ActionType {
    Database,
    HTTP,
    Email,
    Notification,
    File,
    Transform,
    Script,
    AI,
    Integration,
    Approval,
    Delay,
    Condition,
    Loop,
    Variable,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationQueue {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub max_workers: i32,
    pub current_workers: i32,
    pub pending_count: i32,
    pub processing_count: i32,
    pub completed_count: i64,
    pub failed_count: i64,
    pub avg_wait_time_ms: Option<i64>,
    pub avg_process_time_ms: Option<i64>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueItem {
    pub id: Uuid,
    pub queue_id: Uuid,
    pub execution_id: Uuid,
    pub priority: i32,
    pub status: QueueItemStatus,
    pub enqueued_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub wait_time_ms: Option<i64>,
    pub process_time_ms: Option<i64>,
    pub retry_count: i32,
    pub max_retries: i32,
    pub error_message: Option<String>,
    pub worker_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum QueueItemStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
    DeadLetter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationVariable {
    pub id: Uuid,
    pub name: String,
    pub variable_type: VariableType,
    pub scope: VariableScope,
    pub value: Option<String>,
    pub default_value: Option<String>,
    pub description: Option<String>,
    pub is_encrypted: bool,
    pub is_required: bool,
    pub validation_regex: Option<String>,
    pub workflow_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum VariableType {
    String,
    Number,
    Boolean,
    Date,
    JSON,
    Array,
    Secret,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum VariableScope {
    Global,
    Workflow,
    Execution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationMetric {
    pub id: Uuid,
    pub metric_name: String,
    pub metric_type: String,
    pub workflow_id: Option<Uuid>,
    pub value: f64,
    pub unit: Option<String>,
    pub tags: Option<String>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub sample_count: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RPARecorder {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub script_type: RPAScriptType,
    pub script_content: String,
    pub selector_strategy: Option<String>,
    pub variables: Option<String>,
    pub recorded_at: DateTime<Utc>,
    pub recorded_by: Option<Uuid>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum RPAScriptType {
    UIPath,
    Selenium,
    Playwright,
    Puppeteer,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RPAExecution {
    pub id: Uuid,
    pub recorder_id: Uuid,
    pub workflow_execution_id: Option<Uuid>,
    pub status: ExecutionStatus,
    pub screenshots: Option<String>,
    pub logs: Option<String>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration_ms: Option<i64>,
    pub error_message: Option<String>,
}

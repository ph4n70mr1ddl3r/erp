use async_trait::async_trait;
use sqlx::SqlitePool;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity, Status};
use crate::models::*;
use uuid::Uuid;
use chrono::{Utc, DateTime};

#[async_trait]
pub trait WorkflowRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<AutomationWorkflow>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<AutomationWorkflow>>;
    async fn find_active(&self, pool: &SqlitePool) -> Result<Vec<AutomationWorkflow>>;
    async fn create(&self, pool: &SqlitePool, workflow: AutomationWorkflow) -> Result<AutomationWorkflow>;
    async fn update(&self, pool: &SqlitePool, workflow: &AutomationWorkflow) -> Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

pub struct SqliteWorkflowRepository;

#[async_trait]
impl WorkflowRepository for SqliteWorkflowRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<AutomationWorkflow> {
        let row = sqlx::query_as::<_, WorkflowRow>(
            "SELECT * FROM automation_workflows WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("AutomationWorkflow", &id.to_string()))?;

        Ok(row.into())
    }

    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<AutomationWorkflow>> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM automation_workflows")
            .fetch_one(pool)
            .await
            .map_err(|e| Error::Database(e))?;

        let rows = sqlx::query_as::<_, WorkflowRow>(
            "SELECT * FROM automation_workflows ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(pagination.limit())
        .bind(pagination.offset())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(Paginated::new(rows.into_iter().map(|r| r.into()).collect(), count as u64, pagination))
    }

    async fn find_active(&self, pool: &SqlitePool) -> Result<Vec<AutomationWorkflow>> {
        let rows = sqlx::query_as::<_, WorkflowRow>(
            "SELECT * FROM automation_workflows WHERE status = 'Active'"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn create(&self, pool: &SqlitePool, workflow: AutomationWorkflow) -> Result<AutomationWorkflow> {
        sqlx::query(
            r#"INSERT INTO automation_workflows (id, name, code, description, category,
               automation_type, trigger_config, actions, timeout_seconds, max_concurrent_runs,
               priority, status, version, schedule_cron, owner_id, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(workflow.base.id.to_string())
        .bind(&workflow.name)
        .bind(&workflow.code)
        .bind(&workflow.description)
        .bind(&workflow.category)
        .bind(format!("{:?}", workflow.automation_type))
        .bind(&workflow.trigger_config)
        .bind(&workflow.actions)
        .bind(workflow.timeout_seconds)
        .bind(workflow.max_concurrent_runs)
        .bind(workflow.priority)
        .bind(format!("{:?}", workflow.status))
        .bind(workflow.version)
        .bind(&workflow.schedule_cron)
        .bind(workflow.owner_id.map(|id| id.to_string()))
        .bind(workflow.base.created_at.to_rfc3339())
        .bind(workflow.last_modified_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(workflow)
    }

    async fn update(&self, pool: &SqlitePool, workflow: &AutomationWorkflow) -> Result<()> {
        sqlx::query(
            r#"UPDATE automation_workflows SET name = ?, description = ?, trigger_config = ?,
               actions = ?, status = ?, version = ?, last_modified_at = ? WHERE id = ?"#
        )
        .bind(&workflow.name)
        .bind(&workflow.description)
        .bind(&workflow.trigger_config)
        .bind(&workflow.actions)
        .bind(format!("{:?}", workflow.status))
        .bind(workflow.version)
        .bind(workflow.last_modified_at.to_rfc3339())
        .bind(workflow.base.id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("UPDATE automation_workflows SET status = 'Archived' WHERE id = ?")
            .bind(id.to_string())
            .execute(pool)
            .await
            .map_err(|e| Error::Database(e))?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct WorkflowRow {
    id: String,
    name: String,
    code: String,
    description: Option<String>,
    category: String,
    automation_type: String,
    trigger_config: String,
    actions: String,
    timeout_seconds: i32,
    max_concurrent_runs: i32,
    priority: i32,
    status: String,
    version: i32,
    schedule_cron: Option<String>,
    owner_id: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<WorkflowRow> for AutomationWorkflow {
    fn from(r: WorkflowRow) -> Self {
        use chrono::{DateTime, Utc};
        Self {
            base: BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap_or_default(),
                created_at: DateTime::parse_from_rfc3339(&r.created_at)
                    .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&r.updated_at)
                    .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                created_by: None,
                updated_by: None,
            },
            name: r.name,
            code: r.code,
            description: r.description,
            category: r.category,
            automation_type: match r.automation_type.as_str() {
                "EventDriven" => AutomationType::EventDriven,
                "Trigger" => AutomationType::Trigger,
                "Webhook" => AutomationType::Webhook,
                "API" => AutomationType::API,
                "Manual" => AutomationType::Manual,
                "Recurring" => AutomationType::Recurring,
                _ => AutomationType::Scheduled,
            },
            trigger_config: r.trigger_config,
            conditions: None,
            actions: r.actions,
            error_handling: None,
            retry_policy: None,
            timeout_seconds: r.timeout_seconds,
            max_concurrent_runs: r.max_concurrent_runs,
            priority: r.priority,
            status: match r.status.as_str() {
                "Draft" => AutomationStatus::Draft,
                "Active" => AutomationStatus::Active,
                "Paused" => AutomationStatus::Paused,
                "Disabled" => AutomationStatus::Disabled,
                "Archived" => AutomationStatus::Archived,
                "Error" => AutomationStatus::Error,
                _ => AutomationStatus::Draft,
            },
            version: r.version,
            last_modified_by: None,
            last_modified_at: DateTime::parse_from_rfc3339(&r.updated_at)
                .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            published_at: None,
            published_by: None,
            schedule_cron: r.schedule_cron,
            schedule_timezone: None,
            next_run_at: None,
            last_run_at: None,
            last_run_status: None,
            total_runs: 0,
            successful_runs: 0,
            failed_runs: 0,
            avg_duration_ms: None,
            tags: None,
            owner_id: r.owner_id.and_then(|id| Uuid::parse_str(&id).ok()),
            created_at: DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
        }
    }
}

#[async_trait]
pub trait WorkflowExecutionRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, execution: WorkflowExecution) -> Result<WorkflowExecution>;
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<WorkflowExecution>;
    async fn find_by_workflow(&self, pool: &SqlitePool, workflow_id: Uuid, limit: i64) -> Result<Vec<WorkflowExecution>>;
    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: ExecutionStatus, output: Option<String>) -> Result<()>;
}

pub struct SqliteWorkflowExecutionRepository;

#[async_trait]
impl WorkflowExecutionRepository for SqliteWorkflowExecutionRepository {
    async fn create(&self, pool: &SqlitePool, execution: WorkflowExecution) -> Result<WorkflowExecution> {
        sqlx::query(
            r#"INSERT INTO workflow_executions (id, workflow_id, execution_number, trigger_type,
               trigger_data, input_data, status, current_step, total_steps, completed_steps,
               started_at, created_by, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(execution.base.id.to_string())
        .bind(execution.workflow_id.to_string())
        .bind(&execution.execution_number)
        .bind(format!("{:?}", execution.trigger_type))
        .bind(&execution.trigger_data)
        .bind(&execution.input_data)
        .bind(format!("{:?}", execution.status))
        .bind(execution.current_step)
        .bind(execution.total_steps)
        .bind(execution.completed_steps)
        .bind(execution.started_at.to_rfc3339())
        .bind(execution.created_by.map(|id| id.to_string()))
        .bind(execution.base.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(execution)
    }

    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<WorkflowExecution> {
        let row = sqlx::query_as::<_, ExecutionRow>(
            "SELECT * FROM workflow_executions WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("WorkflowExecution", &id.to_string()))?;

        Ok(row.into())
    }

    async fn find_by_workflow(&self, pool: &SqlitePool, workflow_id: Uuid, limit: i64) -> Result<Vec<WorkflowExecution>> {
        let rows = sqlx::query_as::<_, ExecutionRow>(
            "SELECT * FROM workflow_executions WHERE workflow_id = ? ORDER BY started_at DESC LIMIT ?"
        )
        .bind(workflow_id.to_string())
        .bind(limit)
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: ExecutionStatus, output: Option<String>) -> Result<()> {
        let now = Utc::now();
        let completed_at = matches!(status, ExecutionStatus::Completed | ExecutionStatus::Failed | ExecutionStatus::Cancelled).then_some(now);

        sqlx::query(
            "UPDATE workflow_executions SET status = ?, output_data = ?, completed_at = COALESCE(?, completed_at) WHERE id = ?"
        )
        .bind(format!("{:?}", status))
        .bind(&output)
        .bind(completed_at.map(|d| d.to_rfc3339()))
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct ExecutionRow {
    id: String,
    workflow_id: String,
    execution_number: String,
    trigger_type: String,
    trigger_data: Option<String>,
    input_data: Option<String>,
    output_data: Option<String>,
    status: String,
    current_step: Option<i32>,
    total_steps: i32,
    completed_steps: i32,
    started_at: String,
    completed_at: Option<String>,
    error_message: Option<String>,
    created_by: Option<String>,
    created_at: String,
}

impl From<ExecutionRow> for WorkflowExecution {
    fn from(r: ExecutionRow) -> Self {
        use chrono::{DateTime, Utc};
        Self {
            base: BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap_or_default(),
                created_at: DateTime::parse_from_rfc3339(&r.created_at)
                    .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: Utc::now(),
                created_by: None,
                updated_by: None,
            },
            workflow_id: Uuid::parse_str(&r.workflow_id).unwrap_or_default(),
            execution_number: r.execution_number,
            trigger_type: match r.trigger_type.as_str() {
                "Schedule" => TriggerType::Schedule,
                "Webhook" => TriggerType::Webhook,
                "API" => TriggerType::API,
                "Event" => TriggerType::Event,
                "Condition" => TriggerType::Condition,
                "Parent" => TriggerType::Parent,
                _ => TriggerType::Manual,
            },
            trigger_data: r.trigger_data,
            input_data: r.input_data,
            output_data: r.output_data,
            status: match r.status.as_str() {
                "Running" => ExecutionStatus::Running,
                "Paused" => ExecutionStatus::Paused,
                "Waiting" => ExecutionStatus::Waiting,
                "Completed" => ExecutionStatus::Completed,
                "Failed" => ExecutionStatus::Failed,
                "Cancelled" => ExecutionStatus::Cancelled,
                "Timeout" => ExecutionStatus::Timeout,
                "Retrying" => ExecutionStatus::Retrying,
                _ => ExecutionStatus::Pending,
            },
            current_step: r.current_step,
            total_steps: r.total_steps,
            completed_steps: r.completed_steps,
            progress_percent: if r.total_steps > 0 { (r.completed_steps * 100 / r.total_steps) } else { 0 },
            started_at: DateTime::parse_from_rfc3339(&r.started_at)
                .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            completed_at: r.completed_at.and_then(|d| DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            duration_ms: None,
            error_step: None,
            error_message: r.error_message,
            error_stack: None,
            retry_count: 0,
            parent_execution_id: None,
            correlation_id: None,
            variables: None,
            checkpoint_data: None,
            created_by: r.created_by.and_then(|id| Uuid::parse_str(&id).ok()),
        }
    }
}

#[async_trait]
pub trait ScheduledJobRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ScheduledJob>;
    async fn find_due(&self, pool: &SqlitePool) -> Result<Vec<ScheduledJob>>;
    async fn create(&self, pool: &SqlitePool, job: ScheduledJob) -> Result<ScheduledJob>;
    async fn update_next_run(&self, pool: &SqlitePool, id: Uuid, next_run: Option<DateTime<Utc>>) -> Result<()>;
}

pub struct SqliteScheduledJobRepository;

#[async_trait]
impl ScheduledJobRepository for SqliteScheduledJobRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ScheduledJob> {
        let row = sqlx::query_as::<_, ScheduledJobRow>(
            "SELECT * FROM scheduled_jobs WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("ScheduledJob", &id.to_string()))?;

        Ok(row.into())
    }

    async fn find_due(&self, pool: &SqlitePool) -> Result<Vec<ScheduledJob>> {
        let rows = sqlx::query_as::<_, ScheduledJobRow>(
            "SELECT * FROM scheduled_jobs WHERE is_active = 1 AND next_run_at <= ?"
        )
        .bind(Utc::now().to_rfc3339())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn create(&self, pool: &SqlitePool, job: ScheduledJob) -> Result<ScheduledJob> {
        sqlx::query(
            r#"INSERT INTO scheduled_jobs (id, name, job_type, description, schedule_cron,
               timezone, workflow_id, job_config, parameters, is_active, next_run_at, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(job.base.id.to_string())
        .bind(&job.name)
        .bind(&job.job_type)
        .bind(&job.description)
        .bind(&job.schedule_cron)
        .bind(&job.timezone)
        .bind(job.workflow_id.map(|id| id.to_string()))
        .bind(&job.job_config)
        .bind(&job.parameters)
        .bind(job.is_active as i32)
        .bind(job.next_run_at.map(|d| d.to_rfc3339()))
        .bind(job.base.created_at.to_rfc3339())
        .bind(job.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(job)
    }

    async fn update_next_run(&self, pool: &SqlitePool, id: Uuid, next_run: Option<DateTime<Utc>>) -> Result<()> {
        let now = Utc::now();
        sqlx::query(
            "UPDATE scheduled_jobs SET next_run_at = ?, last_run_at = ?, updated_at = ? WHERE id = ?"
        )
        .bind(next_run.map(|d| d.to_rfc3339()))
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct ScheduledJobRow {
    id: String,
    name: String,
    job_type: String,
    description: Option<String>,
    schedule_cron: String,
    timezone: String,
    workflow_id: Option<String>,
    job_config: Option<String>,
    parameters: Option<String>,
    is_active: i32,
    last_run_at: Option<String>,
    next_run_at: Option<String>,
    run_count: i64,
    failure_count: i64,
    created_at: String,
    updated_at: String,
}

impl From<ScheduledJobRow> for ScheduledJob {
    fn from(r: ScheduledJobRow) -> Self {
        use chrono::{DateTime, Utc};
        Self {
            base: BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap_or_default(),
                created_at: DateTime::parse_from_rfc3339(&r.created_at)
                    .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&r.updated_at)
                    .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                created_by: None,
                updated_by: None,
            },
            name: r.name,
            job_type: r.job_type,
            description: r.description,
            schedule_cron: r.schedule_cron,
            timezone: r.timezone,
            workflow_id: r.workflow_id.and_then(|id| Uuid::parse_str(&id).ok()),
            job_config: r.job_config,
            parameters: r.parameters,
            is_active: r.is_active != 0,
            misfire_policy: MisfirePolicy::RunImmediately,
            last_run_at: r.last_run_at.and_then(|d| DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            last_run_status: None,
            last_duration_ms: None,
            next_run_at: r.next_run_at.and_then(|d| DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            run_count: r.run_count,
            failure_count: r.failure_count,
            consecutive_failures: 0,
            max_consecutive_failures: 3,
            created_by: None,
            created_at: DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            updated_at: DateTime::parse_from_rfc3339(&r.updated_at)
                .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
        }
    }
}

#[async_trait]
pub trait WebhookRequestRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, request: WebhookRequest) -> Result<WebhookRequest>;
    async fn find_by_endpoint(&self, pool: &SqlitePool, endpoint_id: Uuid, limit: i64) -> Result<Vec<WebhookRequest>>;
}

pub struct SqliteWebhookRequestRepository;

#[async_trait]
impl WebhookRequestRepository for SqliteWebhookRequestRepository {
    async fn create(&self, pool: &SqlitePool, request: WebhookRequest) -> Result<WebhookRequest> {
        sqlx::query(
            r#"INSERT INTO webhook_requests (id, endpoint_id, request_id, method, headers,
               query_params, body, content_type, source_ip, received_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(request.id.to_string())
        .bind(request.endpoint_id.to_string())
        .bind(&request.request_id)
        .bind(&request.method)
        .bind(&request.headers)
        .bind(&request.query_params)
        .bind(&request.body)
        .bind(&request.content_type)
        .bind(&request.source_ip)
        .bind(request.received_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(request)
    }

    async fn find_by_endpoint(&self, pool: &SqlitePool, endpoint_id: Uuid, limit: i64) -> Result<Vec<WebhookRequest>> {
        let rows = sqlx::query_as::<_, WebhookRequestRow>(
            "SELECT * FROM webhook_requests WHERE endpoint_id = ? ORDER BY received_at DESC LIMIT ?"
        )
        .bind(endpoint_id.to_string())
        .bind(limit)
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

#[derive(sqlx::FromRow)]
struct WebhookRequestRow {
    id: String,
    endpoint_id: String,
    request_id: String,
    method: String,
    headers: Option<String>,
    query_params: Option<String>,
    body: Option<String>,
    content_type: Option<String>,
    source_ip: Option<String>,
    user_agent: Option<String>,
    execution_id: Option<String>,
    response_status: Option<i32>,
    response_body: Option<String>,
    processing_time_ms: Option<i64>,
    received_at: String,
    processed_at: Option<String>,
}

impl From<WebhookRequestRow> for WebhookRequest {
    fn from(r: WebhookRequestRow) -> Self {
        use chrono::{DateTime, Utc};
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            endpoint_id: Uuid::parse_str(&r.endpoint_id).unwrap_or_default(),
            request_id: r.request_id,
            method: r.method,
            headers: r.headers,
            query_params: r.query_params,
            body: r.body,
            content_type: r.content_type,
            source_ip: r.source_ip,
            user_agent: r.user_agent,
            execution_id: r.execution_id.and_then(|id| Uuid::parse_str(&id).ok()),
            response_status: r.response_status,
            response_body: r.response_body,
            processing_time_ms: r.processing_time_ms,
            error_message: None,
            received_at: DateTime::parse_from_rfc3339(&r.received_at)
                .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            processed_at: r.processed_at.and_then(|d| DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
        }
    }
}

use async_trait::async_trait;
use sqlx::SqlitePool;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity};
use crate::models::*;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[async_trait]
pub trait ReportDefinitionRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ReportDefinition>;
    async fn find_by_code(&self, pool: &SqlitePool, code: &str) -> Result<ReportDefinition>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<ReportDefinition>>;
    async fn create(&self, pool: &SqlitePool, report: ReportDefinition) -> Result<ReportDefinition>;
    async fn update(&self, pool: &SqlitePool, report: ReportDefinition) -> Result<ReportDefinition>;
}

pub struct SqliteReportDefinitionRepository;

#[async_trait]
impl ReportDefinitionRepository for SqliteReportDefinitionRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ReportDefinition> {
        let row = sqlx::query_as::<_, ReportDefinitionRow>(
            "SELECT * FROM report_definitions WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("ReportDefinition", &id.to_string()))?;
        
        Ok(row.into())
    }
    
    async fn find_by_code(&self, pool: &SqlitePool, code: &str) -> Result<ReportDefinition> {
        let row = sqlx::query_as::<_, ReportDefinitionRow>(
            "SELECT * FROM report_definitions WHERE code = ?"
        )
        .bind(code)
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("ReportDefinition", code))?;
        
        Ok(row.into())
    }
    
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<ReportDefinition>> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM report_definitions")
            .fetch_one(pool)
            .await
            .map_err(|e| Error::Database(e))?;
        
        let offset = (pagination.page.saturating_sub(1)) * pagination.per_page;
        let rows = sqlx::query_as::<_, ReportDefinitionRow>(
            "SELECT * FROM report_definitions ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(pagination.per_page as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(Paginated::new(rows.into_iter().map(|r| r.into()).collect(), count as u64, pagination))
    }
    
    async fn create(&self, pool: &SqlitePool, report: ReportDefinition) -> Result<ReportDefinition> {
        sqlx::query(
            "INSERT INTO report_definitions (id, name, code, category, description, data_source, query_template, parameters, columns, default_format, allowed_formats, is_scheduled, status, created_by, version, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(report.base.id.to_string())
        .bind(&report.name)
        .bind(&report.code)
        .bind(format!("{:?}", report.category))
        .bind(&report.description)
        .bind(&report.data_source)
        .bind(&report.query_template)
        .bind(serde_json::to_string(&report.parameters).unwrap_or_default())
        .bind(serde_json::to_string(&report.columns).unwrap_or_default())
        .bind(format!("{:?}", report.default_format))
        .bind(serde_json::to_string(&report.allowed_formats).unwrap_or_default())
        .bind(report.is_scheduled as i32)
        .bind(format!("{:?}", report.status))
        .bind(report.created_by.map(|id| id.to_string()))
        .bind(report.version)
        .bind(report.base.created_at.to_rfc3339())
        .bind(report.base.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(report)
    }
    
    async fn update(&self, pool: &SqlitePool, report: ReportDefinition) -> Result<ReportDefinition> {
        sqlx::query(
            "UPDATE report_definitions SET name = ?, query_template = ?, parameters = ?, columns = ?, status = ?, version = version + 1, updated_at = ? WHERE id = ?"
        )
        .bind(&report.name)
        .bind(&report.query_template)
        .bind(serde_json::to_string(&report.parameters).unwrap_or_default())
        .bind(serde_json::to_string(&report.columns).unwrap_or_default())
        .bind(format!("{:?}", report.status))
        .bind(report.base.updated_at.to_rfc3339())
        .bind(report.base.id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(report)
    }
}

#[derive(sqlx::FromRow)]
struct ReportDefinitionRow {
    id: String,
    name: String,
    code: String,
    category: String,
    description: Option<String>,
    data_source: String,
    query_template: String,
    parameters: String,
    columns: String,
    default_format: String,
    allowed_formats: String,
    is_scheduled: i32,
    status: String,
    created_by: Option<String>,
    version: i32,
    created_at: String,
    updated_at: String,
}

impl From<ReportDefinitionRow> for ReportDefinition {
    fn from(r: ReportDefinitionRow) -> Self {
        Self {
            base: BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                created_by: None,
                updated_by: None,
            },
            name: r.name,
            code: r.code,
            category: match r.category.as_str() {
                "Sales" => ReportCategory::Sales,
                "Inventory" => ReportCategory::Inventory,
                "Purchasing" => ReportCategory::Purchasing,
                "Manufacturing" => ReportCategory::Manufacturing,
                "HR" => ReportCategory::HR,
                "CRM" => ReportCategory::CRM,
                "Operations" => ReportCategory::Operations,
                "Compliance" => ReportCategory::Compliance,
                "Executive" => ReportCategory::Executive,
                "Custom" => ReportCategory::Custom,
                _ => ReportCategory::Financial,
            },
            description: r.description,
            data_source: r.data_source,
            query_template: r.query_template,
            parameters: serde_json::from_str(&r.parameters).unwrap_or_default(),
            columns: serde_json::from_str(&r.columns).unwrap_or_default(),
            default_format: match r.default_format.as_str() {
                "Excel" => ReportFormat::Excel,
                "CSV" => ReportFormat::CSV,
                "HTML" => ReportFormat::HTML,
                "JSON" => ReportFormat::JSON,
                "Word" => ReportFormat::Word,
                _ => ReportFormat::PDF,
            },
            allowed_formats: serde_json::from_str(&r.allowed_formats).unwrap_or_default(),
            is_scheduled: r.is_scheduled != 0,
            status: match r.status.as_str() {
                "Inactive" => erp_core::Status::Inactive,
                _ => erp_core::Status::Active,
            },
            created_by: r.created_by.and_then(|id| Uuid::parse_str(&id).ok()),
            version: r.version,
        }
    }
}

#[async_trait]
pub trait ReportScheduleRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ReportSchedule>;
    async fn find_due(&self, pool: &SqlitePool) -> Result<Vec<ReportSchedule>>;
    async fn create(&self, pool: &SqlitePool, schedule: ReportSchedule) -> Result<ReportSchedule>;
    async fn update_next_run(&self, pool: &SqlitePool, id: Uuid, next_run: DateTime<chrono::Utc>) -> Result<()>;
}

pub struct SqliteReportScheduleRepository;

#[async_trait]
impl ReportScheduleRepository for SqliteReportScheduleRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ReportSchedule> {
        let row = sqlx::query_as::<_, ReportScheduleRow>(
            "SELECT * FROM report_schedules WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("ReportSchedule", &id.to_string()))?;
        
        Ok(row.into())
    }
    
    async fn find_due(&self, pool: &SqlitePool) -> Result<Vec<ReportSchedule>> {
        let now = chrono::Utc::now();
        let rows = sqlx::query_as::<_, ReportScheduleRow>(
            "SELECT * FROM report_schedules WHERE is_active = 1 AND next_run_at <= ? AND status = 'Active'"
        )
        .bind(now.to_rfc3339())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
    
    async fn create(&self, pool: &SqlitePool, schedule: ReportSchedule) -> Result<ReportSchedule> {
        sqlx::query(
            "INSERT INTO report_schedules (id, report_definition_id, name, frequency, cron_expression, start_date, end_date, next_run_at, last_run_at, parameters, output_format, delivery_methods, recipients, email_subject, email_body, include_attachments, ftp_host, ftp_path, webhook_url, is_active, status, created_by, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(schedule.base.id.to_string())
        .bind(schedule.report_definition_id.to_string())
        .bind(&schedule.name)
        .bind(format!("{:?}", schedule.frequency))
        .bind(&schedule.cron_expression)
        .bind(schedule.start_date.to_rfc3339())
        .bind(schedule.end_date.map(|d| d.to_rfc3339()))
        .bind(schedule.next_run_at.map(|d| d.to_rfc3339()))
        .bind(schedule.last_run_at.map(|d| d.to_rfc3339()))
        .bind(&schedule.parameters)
        .bind(format!("{:?}", schedule.output_format))
        .bind(serde_json::to_string(&schedule.delivery_methods).unwrap_or_default())
        .bind(serde_json::to_string(&schedule.recipients).unwrap_or_default())
        .bind(&schedule.email_subject)
        .bind(&schedule.email_body)
        .bind(schedule.include_attachments as i32)
        .bind(&schedule.ftp_host)
        .bind(&schedule.ftp_path)
        .bind(&schedule.webhook_url)
        .bind(schedule.is_active as i32)
        .bind(format!("{:?}", schedule.status))
        .bind(schedule.created_by.map(|id| id.to_string()))
        .bind(schedule.base.created_at.to_rfc3339())
        .bind(schedule.base.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(schedule)
    }
    
    async fn update_next_run(&self, pool: &SqlitePool, id: Uuid, next_run: DateTime<chrono::Utc>) -> Result<()> {
        let now = chrono::Utc::now();
        sqlx::query(
            "UPDATE report_schedules SET next_run_at = ?, last_run_at = ?, updated_at = ? WHERE id = ?"
        )
        .bind(next_run.to_rfc3339())
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
struct ReportScheduleRow {
    id: String,
    report_definition_id: String,
    name: String,
    frequency: String,
    cron_expression: Option<String>,
    start_date: String,
    end_date: Option<String>,
    next_run_at: Option<String>,
    last_run_at: Option<String>,
    parameters: String,
    output_format: String,
    delivery_methods: String,
    recipients: String,
    email_subject: Option<String>,
    email_body: Option<String>,
    include_attachments: i32,
    ftp_host: Option<String>,
    ftp_path: Option<String>,
    webhook_url: Option<String>,
    is_active: i32,
    status: String,
    created_by: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<ReportScheduleRow> for ReportSchedule {
    fn from(r: ReportScheduleRow) -> Self {
        Self {
            base: BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                created_by: None,
                updated_by: None,
            },
            report_definition_id: Uuid::parse_str(&r.report_definition_id).unwrap_or_default(),
            name: r.name,
            frequency: match r.frequency.as_str() {
                "Once" => ScheduleFrequency::Once,
                "Hourly" => ScheduleFrequency::Hourly,
                "Daily" => ScheduleFrequency::Daily,
                "Weekly" => ScheduleFrequency::Weekly,
                "Monthly" => ScheduleFrequency::Monthly,
                "Quarterly" => ScheduleFrequency::Quarterly,
                "Yearly" => ScheduleFrequency::Yearly,
                "Custom" => ScheduleFrequency::Custom,
                _ => ScheduleFrequency::Daily,
            },
            cron_expression: r.cron_expression,
            start_date: chrono::DateTime::parse_from_rfc3339(&r.start_date)
                .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            end_date: r.end_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            next_run_at: r.next_run_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            last_run_at: r.last_run_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            parameters: r.parameters,
            output_format: match r.output_format.as_str() {
                "Excel" => ReportFormat::Excel,
                "CSV" => ReportFormat::CSV,
                "HTML" => ReportFormat::HTML,
                "JSON" => ReportFormat::JSON,
                "Word" => ReportFormat::Word,
                _ => ReportFormat::PDF,
            },
            delivery_methods: serde_json::from_str(&r.delivery_methods).unwrap_or_default(),
            recipients: serde_json::from_str(&r.recipients).unwrap_or_default(),
            email_subject: r.email_subject,
            email_body: r.email_body,
            include_attachments: r.include_attachments != 0,
            ftp_host: r.ftp_host,
            ftp_path: r.ftp_path,
            webhook_url: r.webhook_url,
            is_active: r.is_active != 0,
            status: match r.status.as_str() {
                "Inactive" => erp_core::Status::Inactive,
                _ => erp_core::Status::Active,
            },
            created_by: r.created_by.and_then(|id| Uuid::parse_str(&id).ok()),
        }
    }
}

#[async_trait]
pub trait ReportExecutionRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ReportExecution>;
    async fn create(&self, pool: &SqlitePool, execution: ReportExecution) -> Result<ReportExecution>;
    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: ReportStatus, file_path: Option<&str>, row_count: i64, error: Option<&str>) -> Result<()>;
}

pub struct SqliteReportExecutionRepository;

#[async_trait]
impl ReportExecutionRepository for SqliteReportExecutionRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ReportExecution> {
        let row = sqlx::query_as::<_, ReportExecutionRow>(
            "SELECT * FROM report_executions WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("ReportExecution", &id.to_string()))?;
        
        Ok(row.into())
    }
    
    async fn create(&self, pool: &SqlitePool, execution: ReportExecution) -> Result<ReportExecution> {
        sqlx::query(
            "INSERT INTO report_executions (id, report_definition_id, schedule_id, parameters, format, status, started_at, completed_at, duration_ms, row_count, file_path, file_size_bytes, error_message, delivery_status, delivered_at, executed_by, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(execution.base.id.to_string())
        .bind(execution.report_definition_id.to_string())
        .bind(execution.schedule_id.map(|id| id.to_string()))
        .bind(&execution.parameters)
        .bind(format!("{:?}", execution.format))
        .bind(format!("{:?}", execution.status))
        .bind(execution.started_at.map(|d| d.to_rfc3339()))
        .bind(execution.completed_at.map(|d| d.to_rfc3339()))
        .bind(execution.duration_ms)
        .bind(execution.row_count)
        .bind(&execution.file_path)
        .bind(execution.file_size_bytes)
        .bind(&execution.error_message)
        .bind(&execution.delivery_status)
        .bind(execution.delivered_at.map(|d| d.to_rfc3339()))
        .bind(execution.executed_by.map(|id| id.to_string()))
        .bind(execution.base.created_at.to_rfc3339())
        .bind(execution.base.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(execution)
    }
    
    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: ReportStatus, file_path: Option<&str>, row_count: i64, error: Option<&str>) -> Result<()> {
        let now = chrono::Utc::now();
        let completed_at = matches!(status, ReportStatus::Completed | ReportStatus::Failed).then_some(now);
        
        sqlx::query(
            "UPDATE report_executions SET status = ?, file_path = ?, row_count = ?, error_message = ?, completed_at = COALESCE(?, completed_at), updated_at = ? WHERE id = ?"
        )
        .bind(format!("{:?}", status))
        .bind(file_path)
        .bind(row_count)
        .bind(error)
        .bind(completed_at.map(|d| d.to_rfc3339()))
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct ReportExecutionRow {
    id: String,
    report_definition_id: String,
    schedule_id: Option<String>,
    parameters: String,
    format: String,
    status: String,
    started_at: Option<String>,
    completed_at: Option<String>,
    duration_ms: Option<i64>,
    row_count: i64,
    file_path: Option<String>,
    file_size_bytes: Option<i64>,
    error_message: Option<String>,
    delivery_status: Option<String>,
    delivered_at: Option<String>,
    executed_by: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<ReportExecutionRow> for ReportExecution {
    fn from(r: ReportExecutionRow) -> Self {
        Self {
            base: BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                created_by: None,
                updated_by: None,
            },
            report_definition_id: Uuid::parse_str(&r.report_definition_id).unwrap_or_default(),
            schedule_id: r.schedule_id.and_then(|id| Uuid::parse_str(&id).ok()),
            parameters: r.parameters,
            format: match r.format.as_str() {
                "Excel" => ReportFormat::Excel,
                "CSV" => ReportFormat::CSV,
                "HTML" => ReportFormat::HTML,
                "JSON" => ReportFormat::JSON,
                "Word" => ReportFormat::Word,
                _ => ReportFormat::PDF,
            },
            status: match r.status.as_str() {
                "Pending" => ReportStatus::Pending,
                "Running" => ReportStatus::Running,
                "Completed" => ReportStatus::Completed,
                "Failed" => ReportStatus::Failed,
                "Cancelled" => ReportStatus::Cancelled,
                _ => ReportStatus::Draft,
            },
            started_at: r.started_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            completed_at: r.completed_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            duration_ms: r.duration_ms,
            row_count: r.row_count,
            file_path: r.file_path,
            file_size_bytes: r.file_size_bytes,
            error_message: r.error_message,
            delivery_status: r.delivery_status,
            delivered_at: r.delivered_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            executed_by: r.executed_by.and_then(|id| Uuid::parse_str(&id).ok()),
        }
    }
}

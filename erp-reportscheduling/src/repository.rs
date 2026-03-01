use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;

#[async_trait]
pub trait ReportScheduleRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, schedule: &ReportSchedule) -> anyhow::Result<ReportSchedule>;
    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<ReportSchedule>>;
    async fn list(&self, pool: &SqlitePool, owner_id: Option<Uuid>, status: Option<ScheduleStatus>) -> anyhow::Result<Vec<ReportSchedule>>;
    async fn list_due(&self, pool: &SqlitePool, before: DateTime<Utc>) -> anyhow::Result<Vec<ReportSchedule>>;
    async fn update(&self, pool: &SqlitePool, schedule: &ReportSchedule) -> anyhow::Result<()>;
    async fn update_next_run(&self, pool: &SqlitePool, id: Uuid, next_run: Option<DateTime<Utc>>, last_run: DateTime<Utc>) -> anyhow::Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()>;
}

pub struct SqliteReportScheduleRepository;

#[async_trait]
impl ReportScheduleRepository for SqliteReportScheduleRepository {
    async fn create(&self, pool: &SqlitePool, schedule: &ReportSchedule) -> anyhow::Result<ReportSchedule> {
        let now = Utc::now();
        sqlx::query_as::<_, ReportSchedule>(
            r#"INSERT INTO report_schedules (
                id, name, description, report_type, report_config, frequency, cron_expression,
                next_run_at, last_run_at, start_date, end_date, timezone, format, delivery_method,
                delivery_config, recipients, cc_recipients, bcc_recipients, email_subject, email_body,
                include_attachment, compress_output, compression_format, max_file_size_mb,
                retry_on_failure, max_retries, retry_interval_minutes, notify_on_success, notify_on_failure,
                notification_recipients, status, priority, tags, owner_id, department_id, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"#,
        )
        .bind(schedule.base.id)
        .bind(&schedule.name)
        .bind(&schedule.description)
        .bind(&schedule.report_type)
        .bind(&schedule.report_config)
        .bind(&schedule.frequency)
        .bind(&schedule.cron_expression)
        .bind(schedule.next_run_at)
        .bind(schedule.last_run_at)
        .bind(schedule.start_date)
        .bind(schedule.end_date)
        .bind(&schedule.timezone)
        .bind(&schedule.format)
        .bind(&schedule.delivery_method)
        .bind(&schedule.delivery_config)
        .bind(serde_json::to_string(&schedule.recipients)?)
        .bind(schedule.cc_recipients.as_ref().and_then(|v| serde_json::to_string(v).ok()))
        .bind(schedule.bcc_recipients.as_ref().and_then(|v| serde_json::to_string(v).ok()))
        .bind(&schedule.email_subject)
        .bind(&schedule.email_body)
        .bind(schedule.include_attachment)
        .bind(schedule.compress_output)
        .bind(&schedule.compression_format)
        .bind(schedule.max_file_size_mb)
        .bind(schedule.retry_on_failure)
        .bind(schedule.max_retries)
        .bind(schedule.retry_interval_minutes)
        .bind(schedule.notify_on_success)
        .bind(schedule.notify_on_failure)
        .bind(schedule.notification_recipients.as_ref().and_then(|v| serde_json::to_string(v).ok()))
        .bind(&schedule.status)
        .bind(schedule.priority)
        .bind(schedule.tags.as_ref().and_then(|v| serde_json::to_string(v).ok()))
        .bind(schedule.owner_id)
        .bind(schedule.department_id)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<ReportSchedule>> {
        sqlx::query_as::<_, ReportSchedule>("SELECT * FROM report_schedules WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list(&self, pool: &SqlitePool, owner_id: Option<Uuid>, status: Option<ScheduleStatus>) -> anyhow::Result<Vec<ReportSchedule>> {
        let mut query = "SELECT * FROM report_schedules WHERE 1=1".to_string();
        if owner_id.is_some() { query.push_str(" AND owner_id = ?"); }
        if status.is_some() { query.push_str(" AND status = ?"); }
        query.push_str(" ORDER BY next_run_at ASC");
        
        let mut q = sqlx::query_as::<_, ReportSchedule>(&query);
        if let Some(oid) = owner_id { q = q.bind(oid); }
        if let Some(s) = status { q = q.bind(s); }
        q.fetch_all(pool).await.map_err(Into::into)
    }

    async fn list_due(&self, pool: &SqlitePool, before: DateTime<Utc>) -> anyhow::Result<Vec<ReportSchedule>> {
        sqlx::query_as::<_, ReportSchedule>(
            "SELECT * FROM report_schedules WHERE status = 'Active' AND next_run_at <= ? ORDER BY next_run_at ASC"
        )
        .bind(before)
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    }

    async fn update(&self, pool: &SqlitePool, schedule: &ReportSchedule) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query(r#"UPDATE report_schedules SET name=?, description=?, status=?, updated_at=? WHERE id=?"#)
            .bind(&schedule.name)
            .bind(&schedule.description)
            .bind(&schedule.status)
            .bind(now)
            .bind(schedule.base.id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn update_next_run(&self, pool: &SqlitePool, id: Uuid, next_run: Option<DateTime<Utc>>, last_run: DateTime<Utc>) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE report_schedules SET next_run_at=?, last_run_at=?, updated_at=? WHERE id=?")
            .bind(next_run)
            .bind(last_run)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM report_schedules WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

#[async_trait]
pub trait ScheduleExecutionRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, execution: &ScheduleExecution) -> anyhow::Result<ScheduleExecution>;
    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<ScheduleExecution>>;
    async fn list_by_schedule(&self, pool: &SqlitePool, schedule_id: Uuid, limit: i32) -> anyhow::Result<Vec<ScheduleExecution>>;
    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: ExecutionStatus, error: Option<String>) -> anyhow::Result<()>;
    async fn complete(&self, pool: &SqlitePool, id: Uuid, report_url: String, file_size: i64, record_count: i64) -> anyhow::Result<()>;
}

pub struct SqliteScheduleExecutionRepository;

#[async_trait]
impl ScheduleExecutionRepository for SqliteScheduleExecutionRepository {
    async fn create(&self, pool: &SqlitePool, execution: &ScheduleExecution) -> anyhow::Result<ScheduleExecution> {
        let now = Utc::now();
        sqlx::query_as::<_, ScheduleExecution>(
            r#"INSERT INTO schedule_executions (
                id, schedule_id, execution_number, started_at, completed_at, duration_seconds,
                status, report_url, file_path, file_size_bytes, record_count, error_message,
                error_stack, retry_count, delivery_attempts, delivery_status, delivery_error,
                delivered_at, delivery_details, parameters, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"#,
        )
        .bind(execution.base.id)
        .bind(execution.schedule_id)
        .bind(execution.execution_number)
        .bind(execution.started_at)
        .bind(execution.completed_at)
        .bind(execution.duration_seconds)
        .bind(&execution.status)
        .bind(&execution.report_url)
        .bind(&execution.file_path)
        .bind(execution.file_size_bytes)
        .bind(execution.record_count)
        .bind(&execution.error_message)
        .bind(&execution.error_stack)
        .bind(execution.retry_count)
        .bind(execution.delivery_attempts)
        .bind(&execution.delivery_status)
        .bind(&execution.delivery_error)
        .bind(execution.delivered_at)
        .bind(&execution.delivery_details)
        .bind(&execution.parameters)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<ScheduleExecution>> {
        sqlx::query_as::<_, ScheduleExecution>("SELECT * FROM schedule_executions WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list_by_schedule(&self, pool: &SqlitePool, schedule_id: Uuid, limit: i32) -> anyhow::Result<Vec<ScheduleExecution>> {
        sqlx::query_as::<_, ScheduleExecution>("SELECT * FROM schedule_executions WHERE schedule_id = ? ORDER BY created_at DESC LIMIT ?")
            .bind(schedule_id)
            .bind(limit)
            .fetch_all(pool)
            .await
            .map_err(Into::into)
    }

    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: ExecutionStatus, error: Option<String>) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE schedule_executions SET status=?, error_message=?, updated_at=? WHERE id=?")
            .bind(&status)
            .bind(&error)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn complete(&self, pool: &SqlitePool, id: Uuid, report_url: String, file_size: i64, record_count: i64) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE schedule_executions SET status='Completed', completed_at=?, report_url=?, file_size_bytes=?, record_count=?, updated_at=? WHERE id=?")
            .bind(now)
            .bind(&report_url)
            .bind(file_size)
            .bind(record_count)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

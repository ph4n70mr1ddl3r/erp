use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;

#[async_trait]
pub trait JobRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, job: &ScheduledJob) -> anyhow::Result<ScheduledJob>;
    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<ScheduledJob>>;
    async fn list(&self, pool: &SqlitePool, status: Option<JobStatus>, limit: i32, offset: i32) -> anyhow::Result<Vec<ScheduledJob>>;
    async fn list_pending(&self, pool: &SqlitePool, limit: i32) -> anyhow::Result<Vec<ScheduledJob>>;
    async fn list_scheduled(&self, pool: &SqlitePool, before: DateTime<Utc>) -> anyhow::Result<Vec<ScheduledJob>>;
    async fn list_recurring(&self, pool: &SqlitePool) -> anyhow::Result<Vec<ScheduledJob>>;
    async fn update(&self, pool: &SqlitePool, job: &ScheduledJob) -> anyhow::Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()>;
    async fn acquire_lock(&self, pool: &SqlitePool, id: Uuid, worker_id: &str) -> anyhow::Result<bool>;
    async fn release_lock(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()>;
}

pub struct SqliteJobRepository;

#[async_trait]
impl JobRepository for SqliteJobRepository {
    async fn create(&self, pool: &SqlitePool, job: &ScheduledJob) -> anyhow::Result<ScheduledJob> {
        let now = Utc::now();
        sqlx::query_as::<_, ScheduledJob>(
            r#"
            INSERT INTO scheduled_jobs (
                id, name, job_type, handler, payload, priority, cron_expression,
                interval_seconds, scheduled_at, started_at, completed_at, next_run_at,
                last_run_at, last_success_at, last_failure_at, status, run_count,
                success_count, failure_count, max_retries, retry_count, retry_delay_seconds,
                timeout_seconds, last_error, last_duration_ms, avg_duration_ms, tags,
                created_by, locked_by, locked_at, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(job.base.id)
        .bind(&job.name)
        .bind(&job.job_type)
        .bind(&job.handler)
        .bind(&job.payload)
        .bind(&job.priority)
        .bind(&job.cron_expression)
        .bind(job.interval_seconds)
        .bind(job.scheduled_at)
        .bind(job.started_at)
        .bind(job.completed_at)
        .bind(job.next_run_at)
        .bind(job.last_run_at)
        .bind(job.last_success_at)
        .bind(job.last_failure_at)
        .bind(&job.status)
        .bind(job.run_count)
        .bind(job.success_count)
        .bind(job.failure_count)
        .bind(job.max_retries)
        .bind(job.retry_count)
        .bind(job.retry_delay_seconds)
        .bind(job.timeout_seconds)
        .bind(&job.last_error)
        .bind(job.last_duration_ms)
        .bind(job.avg_duration_ms)
        .bind(&job.tags)
        .bind(job.created_by)
        .bind(&job.locked_by)
        .bind(job.locked_at)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<ScheduledJob>> {
        sqlx::query_as::<_, ScheduledJob>("SELECT * FROM scheduled_jobs WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list(&self, pool: &SqlitePool, status: Option<JobStatus>, limit: i32, offset: i32) -> anyhow::Result<Vec<ScheduledJob>> {
        match status {
            Some(s) => sqlx::query_as::<_, ScheduledJob>(
                "SELECT * FROM scheduled_jobs WHERE status = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
            )
            .bind(&s)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
            .map_err(Into::into),
            None => sqlx::query_as::<_, ScheduledJob>(
                "SELECT * FROM scheduled_jobs ORDER BY created_at DESC LIMIT ? OFFSET ?"
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
            .map_err(Into::into),
        }
    }

    async fn list_pending(&self, pool: &SqlitePool, limit: i32) -> anyhow::Result<Vec<ScheduledJob>> {
        let now = Utc::now();
        sqlx::query_as::<_, ScheduledJob>(
            r#"SELECT * FROM scheduled_jobs 
               WHERE status IN ('Pending', 'Scheduled') 
               AND (scheduled_at IS NULL OR scheduled_at <= ?)
               AND (locked_by IS NULL OR locked_at < ?)
               ORDER BY 
                 CASE priority WHEN 'Critical' THEN 1 WHEN 'High' THEN 2 WHEN 'Normal' THEN 3 ELSE 4 END,
                 created_at ASC
               LIMIT ?"#
        )
        .bind(now)
        .bind(now - chrono::Duration::minutes(10))
        .bind(limit)
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    }

    async fn list_scheduled(&self, pool: &SqlitePool, before: DateTime<Utc>) -> anyhow::Result<Vec<ScheduledJob>> {
        sqlx::query_as::<_, ScheduledJob>(
            r#"SELECT * FROM scheduled_jobs 
               WHERE status = 'Scheduled' 
               AND next_run_at IS NOT NULL 
               AND next_run_at <= ?
               ORDER BY next_run_at ASC"#
        )
        .bind(before)
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    }

    async fn list_recurring(&self, pool: &SqlitePool) -> anyhow::Result<Vec<ScheduledJob>> {
        sqlx::query_as::<_, ScheduledJob>(
            "SELECT * FROM scheduled_jobs WHERE job_type IN ('Recurring', 'Cron') AND status = 'Scheduled'"
        )
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    }

    async fn update(&self, pool: &SqlitePool, job: &ScheduledJob) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query(
            r#"
            UPDATE scheduled_jobs SET
                name = ?, handler = ?, payload = ?, priority = ?, cron_expression = ?,
                interval_seconds = ?, scheduled_at = ?, started_at = ?, completed_at = ?,
                next_run_at = ?, last_run_at = ?, last_success_at = ?, last_failure_at = ?,
                status = ?, run_count = ?, success_count = ?, failure_count = ?,
                max_retries = ?, retry_count = ?, last_error = ?, last_duration_ms = ?,
                avg_duration_ms = ?, locked_by = ?, locked_at = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&job.name)
        .bind(&job.handler)
        .bind(&job.payload)
        .bind(&job.priority)
        .bind(&job.cron_expression)
        .bind(job.interval_seconds)
        .bind(job.scheduled_at)
        .bind(job.started_at)
        .bind(job.completed_at)
        .bind(job.next_run_at)
        .bind(job.last_run_at)
        .bind(job.last_success_at)
        .bind(job.last_failure_at)
        .bind(&job.status)
        .bind(job.run_count)
        .bind(job.success_count)
        .bind(job.failure_count)
        .bind(job.max_retries)
        .bind(job.retry_count)
        .bind(&job.last_error)
        .bind(job.last_duration_ms)
        .bind(job.avg_duration_ms)
        .bind(&job.locked_by)
        .bind(job.locked_at)
        .bind(now)
        .bind(job.base.id)
        .execute(pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM scheduled_jobs WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn acquire_lock(&self, pool: &SqlitePool, id: Uuid, worker_id: &str) -> anyhow::Result<bool> {
        let now = Utc::now();
        let result = sqlx::query(
            r#"UPDATE scheduled_jobs SET locked_by = ?, locked_at = ? 
               WHERE id = ? AND (locked_by IS NULL OR locked_at < ?)"#
        )
        .bind(worker_id)
        .bind(now)
        .bind(id)
        .bind(now - chrono::Duration::minutes(10))
        .execute(pool)
        .await?;
        
        Ok(result.rows_affected() > 0)
    }

    async fn release_lock(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("UPDATE scheduled_jobs SET locked_by = NULL, locked_at = NULL WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

#[async_trait]
pub trait JobExecutionRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, execution: &JobExecution) -> anyhow::Result<JobExecution>;
    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<JobExecution>>;
    async fn list_by_job(&self, pool: &SqlitePool, job_id: Uuid, limit: i32) -> anyhow::Result<Vec<JobExecution>>;
    async fn update(&self, pool: &SqlitePool, execution: &JobExecution) -> anyhow::Result<()>;
}

pub struct SqliteJobExecutionRepository;

#[async_trait]
impl JobExecutionRepository for SqliteJobExecutionRepository {
    async fn create(&self, pool: &SqlitePool, execution: &JobExecution) -> anyhow::Result<JobExecution> {
        let now = Utc::now();
        sqlx::query_as::<_, JobExecution>(
            r#"
            INSERT INTO job_executions (
                id, job_id, execution_number, started_at, completed_at, duration_ms,
                status, result, error_message, error_stack_trace, retry_of_id,
                retry_number, worker_id, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(execution.base.id)
        .bind(execution.job_id)
        .bind(execution.execution_number)
        .bind(execution.started_at)
        .bind(execution.completed_at)
        .bind(execution.duration_ms)
        .bind(&execution.status)
        .bind(&execution.result)
        .bind(&execution.error_message)
        .bind(&execution.error_stack_trace)
        .bind(execution.retry_of_id)
        .bind(execution.retry_number)
        .bind(&execution.worker_id)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<JobExecution>> {
        sqlx::query_as::<_, JobExecution>("SELECT * FROM job_executions WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list_by_job(&self, pool: &SqlitePool, job_id: Uuid, limit: i32) -> anyhow::Result<Vec<JobExecution>> {
        sqlx::query_as::<_, JobExecution>(
            "SELECT * FROM job_executions WHERE job_id = ? ORDER BY started_at DESC LIMIT ?"
        )
        .bind(job_id)
        .bind(limit)
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    }

    async fn update(&self, pool: &SqlitePool, execution: &JobExecution) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            UPDATE job_executions SET
                completed_at = ?, duration_ms = ?, status = ?, result = ?,
                error_message = ?, error_stack_trace = ?
            WHERE id = ?
            "#,
        )
        .bind(execution.completed_at)
        .bind(execution.duration_ms)
        .bind(&execution.status)
        .bind(&execution.result)
        .bind(&execution.error_message)
        .bind(&execution.error_stack_trace)
        .bind(execution.base.id)
        .execute(pool)
        .await?;
        Ok(())
    }
}

#[async_trait]
pub trait JobScheduleRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, schedule: &JobSchedule) -> anyhow::Result<JobSchedule>;
    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<JobSchedule>>;
    async fn list(&self, pool: &SqlitePool) -> anyhow::Result<Vec<JobSchedule>>;
    async fn list_enabled(&self, pool: &SqlitePool) -> anyhow::Result<Vec<JobSchedule>>;
    async fn update(&self, pool: &SqlitePool, schedule: &JobSchedule) -> anyhow::Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()>;
}

pub struct SqliteJobScheduleRepository;

#[async_trait]
impl JobScheduleRepository for SqliteJobScheduleRepository {
    async fn create(&self, pool: &SqlitePool, schedule: &JobSchedule) -> anyhow::Result<JobSchedule> {
        let now = Utc::now();
        sqlx::query_as::<_, JobSchedule>(
            r#"
            INSERT INTO job_schedules (
                id, name, job_template_id, job_name, handler, default_payload,
                schedule_type, cron_expression, interval_minutes, specific_times,
                run_on_days, timezone, start_date, end_date, next_scheduled_run,
                last_run, enabled, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(schedule.base.id)
        .bind(&schedule.name)
        .bind(schedule.job_template_id)
        .bind(&schedule.job_name)
        .bind(&schedule.handler)
        .bind(&schedule.default_payload)
        .bind(&schedule.schedule_type)
        .bind(&schedule.cron_expression)
        .bind(schedule.interval_minutes)
        .bind(&schedule.specific_times)
        .bind(&schedule.run_on_days)
        .bind(&schedule.timezone)
        .bind(schedule.start_date)
        .bind(schedule.end_date)
        .bind(schedule.next_scheduled_run)
        .bind(schedule.last_run)
        .bind(schedule.enabled)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<JobSchedule>> {
        sqlx::query_as::<_, JobSchedule>("SELECT * FROM job_schedules WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list(&self, pool: &SqlitePool) -> anyhow::Result<Vec<JobSchedule>> {
        sqlx::query_as::<_, JobSchedule>("SELECT * FROM job_schedules ORDER BY name")
            .fetch_all(pool)
            .await
            .map_err(Into::into)
    }

    async fn list_enabled(&self, pool: &SqlitePool) -> anyhow::Result<Vec<JobSchedule>> {
        sqlx::query_as::<_, JobSchedule>("SELECT * FROM job_schedules WHERE enabled = TRUE ORDER BY name")
            .fetch_all(pool)
            .await
            .map_err(Into::into)
    }

    async fn update(&self, pool: &SqlitePool, schedule: &JobSchedule) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query(
            r#"
            UPDATE job_schedules SET
                name = ?, job_name = ?, handler = ?, default_payload = ?,
                schedule_type = ?, cron_expression = ?, interval_minutes = ?,
                specific_times = ?, run_on_days = ?, timezone = ?, start_date = ?,
                end_date = ?, next_scheduled_run = ?, last_run = ?, enabled = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&schedule.name)
        .bind(&schedule.job_name)
        .bind(&schedule.handler)
        .bind(&schedule.default_payload)
        .bind(&schedule.schedule_type)
        .bind(&schedule.cron_expression)
        .bind(schedule.interval_minutes)
        .bind(&schedule.specific_times)
        .bind(&schedule.run_on_days)
        .bind(&schedule.timezone)
        .bind(schedule.start_date)
        .bind(schedule.end_date)
        .bind(schedule.next_scheduled_run)
        .bind(schedule.last_run)
        .bind(schedule.enabled)
        .bind(now)
        .bind(schedule.base.id)
        .execute(pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM job_schedules WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

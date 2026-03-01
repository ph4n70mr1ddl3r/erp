use chrono::{DateTime, Utc};
use cron::Schedule as CronSchedule;
use erp_core::BaseEntity;
use sqlx::SqlitePool;
use std::str::FromStr;
use uuid::Uuid;

use crate::models::*;
use crate::repository::*;

pub struct JobService {
    job_repo: SqliteJobRepository,
    execution_repo: SqliteJobExecutionRepository,
    schedule_repo: SqliteJobScheduleRepository,
}

impl Default for JobService {
    fn default() -> Self {
        Self::new()
    }
}

impl JobService {
    pub fn new() -> Self {
        Self {
            job_repo: SqliteJobRepository,
            execution_repo: SqliteJobExecutionRepository,
            schedule_repo: SqliteJobScheduleRepository,
        }
    }

    pub async fn submit(
        &self,
        pool: &SqlitePool,
        name: String,
        handler: String,
        payload: Option<serde_json::Value>,
        priority: Option<JobPriority>,
        scheduled_at: Option<DateTime<Utc>>,
        created_by: Option<Uuid>,
    ) -> anyhow::Result<ScheduledJob> {
        let job = ScheduledJob {
            base: BaseEntity::new(),
            name,
            job_type: JobType::OneTime,
            handler,
            payload,
            priority: priority.unwrap_or(JobPriority::Normal),
            cron_expression: None,
            interval_seconds: None,
            scheduled_at,
            started_at: None,
            completed_at: None,
            next_run_at: scheduled_at,
            last_run_at: None,
            last_success_at: None,
            last_failure_at: None,
            status: if scheduled_at.is_some() { JobStatus::Scheduled } else { JobStatus::Pending },
            run_count: 0,
            success_count: 0,
            failure_count: 0,
            max_retries: 3,
            retry_count: 0,
            retry_delay_seconds: 60,
            timeout_seconds: 300,
            last_error: None,
            last_duration_ms: None,
            avg_duration_ms: None,
            tags: None,
            created_by,
            locked_by: None,
            locked_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.job_repo.create(pool, &job).await
    }

    pub async fn schedule_cron(
        &self,
        pool: &SqlitePool,
        name: String,
        handler: String,
        cron_expression: String,
        payload: Option<serde_json::Value>,
    ) -> anyhow::Result<ScheduledJob> {
        let next_run = calculate_next_cron_run(&cron_expression)?;
        
        let job = ScheduledJob {
            base: BaseEntity::new(),
            name,
            job_type: JobType::Cron,
            handler,
            payload,
            priority: JobPriority::Normal,
            cron_expression: Some(cron_expression),
            interval_seconds: None,
            scheduled_at: None,
            started_at: None,
            completed_at: None,
            next_run_at: Some(next_run),
            last_run_at: None,
            last_success_at: None,
            last_failure_at: None,
            status: JobStatus::Scheduled,
            run_count: 0,
            success_count: 0,
            failure_count: 0,
            max_retries: 3,
            retry_count: 0,
            retry_delay_seconds: 60,
            timeout_seconds: 300,
            last_error: None,
            last_duration_ms: None,
            avg_duration_ms: None,
            tags: None,
            created_by: None,
            locked_by: None,
            locked_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.job_repo.create(pool, &job).await
    }

    pub async fn schedule_interval(
        &self,
        pool: &SqlitePool,
        name: String,
        handler: String,
        interval_seconds: i64,
        payload: Option<serde_json::Value>,
    ) -> anyhow::Result<ScheduledJob> {
        let next_run = Utc::now() + chrono::Duration::seconds(interval_seconds);
        
        let job = ScheduledJob {
            base: BaseEntity::new(),
            name,
            job_type: JobType::Recurring,
            handler,
            payload,
            priority: JobPriority::Normal,
            cron_expression: None,
            interval_seconds: Some(interval_seconds),
            scheduled_at: None,
            started_at: None,
            completed_at: None,
            next_run_at: Some(next_run),
            last_run_at: None,
            last_success_at: None,
            last_failure_at: None,
            status: JobStatus::Scheduled,
            run_count: 0,
            success_count: 0,
            failure_count: 0,
            max_retries: 3,
            retry_count: 0,
            retry_delay_seconds: 60,
            timeout_seconds: 300,
            last_error: None,
            last_duration_ms: None,
            avg_duration_ms: None,
            tags: None,
            created_by: None,
            locked_by: None,
            locked_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.job_repo.create(pool, &job).await
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<ScheduledJob>> {
        self.job_repo.get_by_id(pool, id).await
    }

    pub async fn list(&self, pool: &SqlitePool, status: Option<JobStatus>, limit: i32, offset: i32) -> anyhow::Result<Vec<ScheduledJob>> {
        self.job_repo.list(pool, status, limit, offset).await
    }

    pub async fn cancel(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        let mut job = self.job_repo.get_by_id(pool, id).await?
            .ok_or_else(|| anyhow::anyhow!("Job not found"))?;
        
        if job.status == JobStatus::Running {
            return Err(anyhow::anyhow!("Cannot cancel a running job"));
        }
        
        job.status = JobStatus::Cancelled;
        self.job_repo.update(pool, &job).await
    }

    pub async fn retry(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<ScheduledJob> {
        let mut job = self.job_repo.get_by_id(pool, id).await?
            .ok_or_else(|| anyhow::anyhow!("Job not found"))?;
        
        job.status = JobStatus::Pending;
        job.retry_count = 0;
        job.scheduled_at = Some(Utc::now());
        
        self.job_repo.update(pool, &job).await?;
        Ok(job)
    }

    pub async fn get_executions(&self, pool: &SqlitePool, job_id: Uuid, limit: i32) -> anyhow::Result<Vec<JobExecution>> {
        self.execution_repo.list_by_job(pool, job_id, limit).await
    }

    pub async fn process_due_jobs(&self, pool: &SqlitePool, limit: i32) -> anyhow::Result<Vec<ScheduledJob>> {
        let jobs = self.job_repo.list_pending(pool, limit).await?;
        Ok(jobs)
    }

    pub async fn process_recurring_jobs(&self, pool: &SqlitePool) -> anyhow::Result<Vec<ScheduledJob>> {
        let now = Utc::now();
        let jobs = self.job_repo.list_scheduled(pool, now).await?;
        Ok(jobs)
    }

    pub async fn update_after_run(&self, pool: &SqlitePool, job_id: Uuid, success: bool, error: Option<String>, duration_ms: i64) -> anyhow::Result<()> {
        let mut job = self.job_repo.get_by_id(pool, job_id).await?
            .ok_or_else(|| anyhow::anyhow!("Job not found"))?;
        
        let now = Utc::now();
        job.run_count += 1;
        job.last_run_at = Some(now);
        job.last_duration_ms = Some(duration_ms);
        
        if success {
            job.success_count += 1;
            job.last_success_at = Some(now);
            job.last_error = None;
        } else {
            job.failure_count += 1;
            job.last_failure_at = Some(now);
            job.last_error = error;
        }
        
        job.avg_duration_ms = Some(
            ((job.avg_duration_ms.unwrap_or(0) * (job.run_count - 1)) + duration_ms) / job.run_count
        );
        
        if job.job_type == JobType::OneTime {
            job.status = if success { JobStatus::Completed } else { JobStatus::Failed };
            job.completed_at = Some(now);
        } else {
            job.status = JobStatus::Scheduled;
            
            if let Some(cron) = &job.cron_expression {
                if let Ok(next) = calculate_next_cron_run(cron) {
                    job.next_run_at = Some(next);
                }
            } else if let Some(interval) = job.interval_seconds {
                job.next_run_at = Some(now + chrono::Duration::seconds(interval));
            }
        }
        
        self.job_repo.update(pool, &job).await
    }

    pub async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        self.job_repo.delete(pool, id).await
    }
}

pub struct JobScheduleService {
    schedule_repo: SqliteJobScheduleRepository,
    job_service: JobService,
}

impl Default for JobScheduleService {
    fn default() -> Self {
        Self::new()
    }
}

impl JobScheduleService {
    pub fn new() -> Self {
        Self {
            schedule_repo: SqliteJobScheduleRepository,
            job_service: JobService::new(),
        }
    }

    pub async fn create(
        &self,
        pool: &SqlitePool,
        name: String,
        job_name: String,
        handler: String,
        schedule_type: ScheduleType,
        cron_expression: Option<String>,
        interval_minutes: Option<i32>,
        default_payload: Option<serde_json::Value>,
    ) -> anyhow::Result<JobSchedule> {
        let next_run = match &schedule_type {
            ScheduleType::Cron => {
                cron_expression.as_ref()
                    .map(|c| calculate_next_cron_run(c))
                    .transpose()?
            }
            ScheduleType::Interval => {
                interval_minutes.map(|m| Utc::now() + chrono::Duration::minutes(m as i64))
            }
            ScheduleType::Daily => {
                Some(Utc::now() + chrono::Duration::days(1))
            }
            ScheduleType::Weekly => {
                Some(Utc::now() + chrono::Duration::weeks(1))
            }
            ScheduleType::Monthly => {
                Some(Utc::now() + chrono::Duration::days(30))
            }
            ScheduleType::SpecificTimes => None,
        };
        
        let schedule = JobSchedule {
            base: BaseEntity::new(),
            name,
            job_template_id: None,
            job_name,
            handler,
            default_payload,
            schedule_type,
            cron_expression,
            interval_minutes,
            specific_times: None,
            run_on_days: None,
            timezone: "UTC".to_string(),
            start_date: None,
            end_date: None,
            next_scheduled_run: next_run,
            last_run: None,
            enabled: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.schedule_repo.create(pool, &schedule).await
    }

    pub async fn list(&self, pool: &SqlitePool) -> anyhow::Result<Vec<JobSchedule>> {
        self.schedule_repo.list(pool).await
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<JobSchedule>> {
        self.schedule_repo.get_by_id(pool, id).await
    }

    pub async fn enable(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        let mut schedule = self.schedule_repo.get_by_id(pool, id).await?
            .ok_or_else(|| anyhow::anyhow!("Schedule not found"))?;
        schedule.enabled = true;
        self.schedule_repo.update(pool, &schedule).await
    }

    pub async fn disable(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        let mut schedule = self.schedule_repo.get_by_id(pool, id).await?
            .ok_or_else(|| anyhow::anyhow!("Schedule not found"))?;
        schedule.enabled = false;
        self.schedule_repo.update(pool, &schedule).await
    }

    pub async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        self.schedule_repo.delete(pool, id).await
    }

    pub async fn trigger_due(&self, pool: &SqlitePool) -> anyhow::Result<Vec<ScheduledJob>> {
        let schedules = self.schedule_repo.list_enabled(pool).await?;
        let now = Utc::now();
        let mut jobs = Vec::new();
        
        for schedule in schedules {
            if let Some(next_run) = schedule.next_scheduled_run {
                if next_run <= now {
                    let job = self.job_service.submit(
                        pool,
                        schedule.job_name.clone(),
                        schedule.handler.clone(),
                        schedule.default_payload.clone(),
                        None,
                        None,
                        None,
                    ).await?;
                    jobs.push(job);
                    
                    let mut schedule = schedule;
                    schedule.last_run = Some(now);
                    schedule.next_scheduled_run = self.calculate_next_run(&schedule);
                    self.schedule_repo.update(pool, &schedule).await?;
                }
            }
        }
        
        Ok(jobs)
    }

    fn calculate_next_run(&self, schedule: &JobSchedule) -> Option<DateTime<Utc>> {
        match &schedule.schedule_type {
            ScheduleType::Cron => {
                schedule.cron_expression.as_ref()
                    .and_then(|c| calculate_next_cron_run(c).ok())
            }
            ScheduleType::Interval => {
                schedule.interval_minutes.map(|m| Utc::now() + chrono::Duration::minutes(m as i64))
            }
            ScheduleType::Daily => Some(Utc::now() + chrono::Duration::days(1)),
            ScheduleType::Weekly => Some(Utc::now() + chrono::Duration::weeks(1)),
            ScheduleType::Monthly => Some(Utc::now() + chrono::Duration::days(30)),
            ScheduleType::SpecificTimes => None,
        }
    }
}

fn calculate_next_cron_run(cron_expression: &str) -> anyhow::Result<DateTime<Utc>> {
    let schedule = CronSchedule::from_str(cron_expression)
        .map_err(|e| anyhow::anyhow!("Invalid cron expression: {}", e))?;
    
    let next = schedule
        .upcoming(Utc)
        .next()
        .ok_or_else(|| anyhow::anyhow!("No upcoming run found"))?;
    
    Ok(next)
}

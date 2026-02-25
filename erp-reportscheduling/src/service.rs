use chrono::{DateTime, Utc};
use cron::Schedule;
use erp_core::BaseEntity;
use sqlx::SqlitePool;
use std::str::FromStr;
use uuid::Uuid;

use crate::models::*;
use crate::repository::*;

pub struct ReportScheduleService {
    schedule_repo: SqliteReportScheduleRepository,
    execution_repo: SqliteScheduleExecutionRepository,
}

impl ReportScheduleService {
    pub fn new() -> Self {
        Self {
            schedule_repo: SqliteReportScheduleRepository,
            execution_repo: SqliteScheduleExecutionRepository,
        }
    }

    pub async fn create(
        &self,
        pool: &SqlitePool,
        name: String,
        description: Option<String>,
        report_type: String,
        report_config: serde_json::Value,
        frequency: ScheduleFrequency,
        format: ReportFormat,
        delivery_method: DeliveryMethod,
        recipients: Vec<String>,
        owner_id: Uuid,
    ) -> anyhow::Result<ReportSchedule> {
        let next_run = self.calculate_next_run(&frequency, None)?;
        
        let schedule = ReportSchedule {
            base: BaseEntity::new(),
            name,
            description,
            report_type,
            report_config,
            frequency,
            cron_expression: None,
            next_run_at: next_run,
            last_run_at: None,
            start_date: None,
            end_date: None,
            timezone: "UTC".to_string(),
            format,
            delivery_method,
            delivery_config: None,
            recipients,
            cc_recipients: None,
            bcc_recipients: None,
            email_subject: None,
            email_body: None,
            include_attachment: true,
            compress_output: false,
            compression_format: None,
            max_file_size_mb: None,
            retry_on_failure: true,
            max_retries: 3,
            retry_interval_minutes: 30,
            notify_on_success: false,
            notify_on_failure: true,
            notification_recipients: None,
            status: ScheduleStatus::Active,
            priority: 5,
            tags: None,
            owner_id,
            department_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.schedule_repo.create(pool, &schedule).await
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<ReportSchedule>> {
        self.schedule_repo.get_by_id(pool, id).await
    }

    pub async fn list(&self, pool: &SqlitePool, owner_id: Option<Uuid>, status: Option<ScheduleStatus>) -> anyhow::Result<Vec<ReportSchedule>> {
        self.schedule_repo.list(pool, owner_id, status).await
    }

    pub async fn pause(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        if let Some(mut schedule) = self.schedule_repo.get_by_id(pool, id).await? {
            schedule.status = ScheduleStatus::Paused;
            self.schedule_repo.update(pool, &schedule).await?;
        }
        Ok(())
    }

    pub async fn resume(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        if let Some(mut schedule) = self.schedule_repo.get_by_id(pool, id).await? {
            schedule.status = ScheduleStatus::Active;
            let next_run = self.calculate_next_run(&schedule.frequency, None)?;
            schedule.next_run_at = next_run;
            self.schedule_repo.update(pool, &schedule).await?;
        }
        Ok(())
    }

    pub async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        self.schedule_repo.delete(pool, id).await
    }

    pub async fn process_due(&self, pool: &SqlitePool) -> anyhow::Result<Vec<ScheduleExecution>> {
        let due_schedules = self.schedule_repo.list_due(pool, Utc::now()).await?;
        let mut executions = Vec::new();
        
        for schedule in due_schedules {
            let execution = self.execute_schedule(pool, &schedule).await?;
            executions.push(execution);
        }
        
        Ok(executions)
    }

    async fn execute_schedule(&self, pool: &SqlitePool, schedule: &ReportSchedule) -> anyhow::Result<ScheduleExecution> {
        let execution_number = chrono::Utc::now().timestamp();
        
        let execution = ScheduleExecution {
            base: BaseEntity::new(),
            schedule_id: schedule.base.id,
            execution_number,
            started_at: Some(Utc::now()),
            completed_at: None,
            duration_seconds: None,
            status: ExecutionStatus::Running,
            report_url: None,
            file_path: None,
            file_size_bytes: None,
            record_count: None,
            error_message: None,
            error_stack: None,
            retry_count: 0,
            delivery_attempts: 0,
            delivery_status: None,
            delivery_error: None,
            delivered_at: None,
            delivery_details: None,
            parameters: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let execution = self.execution_repo.create(pool, &execution).await?;
        
        let report_result = self.generate_report(pool, schedule).await;
        
        match report_result {
            Ok((url, size, count)) => {
                self.execution_repo.complete(pool, execution.base.id, url.clone(), size, count).await?;
                
                let next_run = self.calculate_next_run(&schedule.frequency, Some(Utc::now()))?;
                self.schedule_repo.update_next_run(pool, schedule.base.id, next_run, Utc::now()).await?;
                
                self.deliver_report(pool, schedule, &url).await?;
            }
            Err(e) => {
                self.execution_repo.update_status(pool, execution.base.id, ExecutionStatus::Failed, Some(e.to_string())).await?;
            }
        }
        
        self.execution_repo.get_by_id(pool, execution.base.id).await?.ok_or_else(|| anyhow::anyhow!("Execution not found"))
    }

    async fn generate_report(&self, _pool: &SqlitePool, schedule: &ReportSchedule) -> anyhow::Result<(String, i64, i64)> {
        let url = format!("/reports/generated/{}.{}", schedule.base.id, match schedule.format {
            ReportFormat::PDF => "pdf",
            ReportFormat::Excel => "xlsx",
            ReportFormat::CSV => "csv",
            ReportFormat::HTML => "html",
            ReportFormat::JSON => "json",
        });
        Ok((url, 1024, 100))
    }

    async fn deliver_report(&self, _pool: &SqlitePool, schedule: &ReportSchedule, _url: &str) -> anyhow::Result<()> {
        match schedule.delivery_method {
            DeliveryMethod::Email => {
            },
            DeliveryMethod::Download => {},
            _ => {}
        }
        Ok(())
    }

    fn calculate_next_run(&self, frequency: &ScheduleFrequency, _after: Option<DateTime<Utc>>) -> anyhow::Result<Option<DateTime<Utc>>> {
        let now = Utc::now();
        let next = match frequency {
            ScheduleFrequency::Once => None,
            ScheduleFrequency::Hourly => Some(now + chrono::Duration::hours(1)),
            ScheduleFrequency::Daily => Some(now + chrono::Duration::days(1)),
            ScheduleFrequency::Weekly => Some(now + chrono::Duration::weeks(1)),
            ScheduleFrequency::Monthly => Some(now + chrono::Duration::days(30)),
            ScheduleFrequency::Quarterly => Some(now + chrono::Duration::days(90)),
            ScheduleFrequency::Yearly => Some(now + chrono::Duration::days(365)),
            ScheduleFrequency::Custom => Some(now + chrono::Duration::days(1)),
        };
        Ok(next)
    }

    pub async fn get_executions(&self, pool: &SqlitePool, schedule_id: Uuid, limit: i32) -> anyhow::Result<Vec<ScheduleExecution>> {
        self.execution_repo.list_by_schedule(pool, schedule_id, limit).await
    }
}

pub struct ScheduleExecutionService {
    execution_repo: SqliteScheduleExecutionRepository,
}

impl ScheduleExecutionService {
    pub fn new() -> Self {
        Self {
            execution_repo: SqliteScheduleExecutionRepository,
        }
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<ScheduleExecution>> {
        self.execution_repo.get_by_id(pool, id).await
    }

    pub async fn retry(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        self.execution_repo.update_status(pool, id, ExecutionStatus::Pending, None).await
    }

    pub async fn cancel(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        self.execution_repo.update_status(pool, id, ExecutionStatus::Cancelled, Some("Cancelled by user".to_string())).await
    }
}

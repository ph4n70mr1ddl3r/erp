use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditType {
    Internal,
    External,
    Regulatory,
    Financial,
    Operational,
    IT,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditStatus {
    Planned,
    InProgress,
    Fieldwork,
    Reporting,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuditEngagement {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub audit_type: AuditType,
    pub status: AuditStatus,
    pub lead_auditor_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Testing,
    Completed,
    ReviewNeeded,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuditTask {
    pub id: Uuid,
    pub engagement_id: Uuid,
    pub title: String,
    pub description: String,
    pub assignee_id: Option<Uuid>,
    pub status: TaskStatus,
    pub due_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AuditEngagement {
    pub fn new(
        title: String,
        audit_type: AuditType,
        lead_auditor_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description: None,
            audit_type,
            status: AuditStatus::Planned,
            lead_auditor_id,
            start_date,
            end_date,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn start(&mut self) {
        if self.status == AuditStatus::Planned {
            self.status = AuditStatus::InProgress;
            self.updated_at = Utc::now();
        }
    }

    pub fn complete(&mut self) {
        self.status = AuditStatus::Completed;
        self.updated_at = Utc::now();
    }
}

impl AuditTask {
    pub fn new(engagement_id: Uuid, title: String, description: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            engagement_id,
            title,
            description,
            assignee_id: None,
            status: TaskStatus::Pending,
            due_date: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn assign(&mut self, auditor_id: Uuid) {
        self.assignee_id = Some(auditor_id);
        self.updated_at = Utc::now();
    }

    pub fn update_status(&mut self, status: TaskStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }
}

pub struct AuditService {}

impl AuditService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn check_engagement_readiness(&self, tasks: &[AuditTask]) -> bool {
        !tasks.is_empty() && tasks.iter().all(|t| t.assignee_id.is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_engagement_lifecycle() {
        let auditor_id = Uuid::new_v4();
        let start = NaiveDate::from_ymd_opt(2026, 4, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2026, 4, 30).unwrap();
        
        let mut engagement = AuditEngagement::new(
            "2026 Annual IT Audit".to_string(),
            AuditType::IT,
            auditor_id,
            start,
            end,
        );

        assert_eq!(engagement.status, AuditStatus::Planned);

        engagement.start();
        assert_eq!(engagement.status, AuditStatus::InProgress);

        engagement.complete();
        assert_eq!(engagement.status, AuditStatus::Completed);
    }

    #[test]
    fn test_audit_task_assignment() {
        let engagement_id = Uuid::new_v4();
        let auditor_id = Uuid::new_v4();
        let mut task = AuditTask::new(
            engagement_id,
            "Review User Access Logs".to_string(),
            "Verify that all privileged access was authorized.".to_string(),
        );

        assert_eq!(task.status, TaskStatus::Pending);
        assert!(task.assignee_id.is_none());

        task.assign(auditor_id);
        assert_eq!(task.assignee_id, Some(auditor_id));

        task.update_status(TaskStatus::InProgress);
        assert_eq!(task.status, TaskStatus::InProgress);
    }

    #[test]
    fn test_audit_service_readiness() {
        let service = AuditService::new();
        let engagement_id = Uuid::new_v4();
        let auditor_id = Uuid::new_v4();

        let mut t1 = AuditTask::new(engagement_id, "Task 1".to_string(), "Desc 1".to_string());
        let mut t2 = AuditTask::new(engagement_id, "Task 2".to_string(), "Desc 2".to_string());

        // Not ready: no assignments
        assert!(!service.check_engagement_readiness(&[t1.clone(), t2.clone()]));

        t1.assign(auditor_id);
        // Not ready: one task missing assignment
        assert!(!service.check_engagement_readiness(&[t1.clone(), t2.clone()]));

        t2.assign(auditor_id);
        // Ready: all tasks assigned
        assert!(service.check_engagement_readiness(&[t1, t2]));
    }
}

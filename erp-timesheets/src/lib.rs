use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TimesheetStatus {
    Draft,
    Submitted,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimesheetEntry {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub project_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub date: NaiveDate,
    pub hours: f64,
    pub description: Option<String>,
    pub is_billable: bool,
    pub status: TimesheetStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TimesheetEntry {
    pub fn new(
        employee_id: Uuid,
        date: NaiveDate,
        hours: f64,
        is_billable: bool,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            employee_id,
            project_id: None,
            task_id: None,
            date,
            hours,
            description: None,
            is_billable,
            status: TimesheetStatus::Draft,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn set_project_and_task(&mut self, project_id: Uuid, task_id: Option<Uuid>) {
        self.project_id = Some(project_id);
        self.task_id = task_id;
        self.updated_at = Utc::now();
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
        self.updated_at = Utc::now();
    }

    pub fn submit(&mut self) {
        if self.status == TimesheetStatus::Draft || self.status == TimesheetStatus::Rejected {
            self.status = TimesheetStatus::Submitted;
            self.updated_at = Utc::now();
        }
    }

    pub fn approve(&mut self) {
        if self.status == TimesheetStatus::Submitted {
            self.status = TimesheetStatus::Approved;
            self.updated_at = Utc::now();
        }
    }

    pub fn reject(&mut self) {
        if self.status == TimesheetStatus::Submitted {
            self.status = TimesheetStatus::Rejected;
            self.updated_at = Utc::now();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timesheet_entry_lifecycle() {
        let employee_id = Uuid::new_v4();
        let project_id = Uuid::new_v4();
        let date = NaiveDate::from_ymd_opt(2026, 3, 8).unwrap();
        
        let mut entry = TimesheetEntry::new(employee_id, date, 8.5, true);
        
        assert_eq!(entry.hours, 8.5);
        assert_eq!(entry.status, TimesheetStatus::Draft);
        assert_eq!(entry.is_billable, true);
        
        entry.set_project_and_task(project_id, None);
        assert_eq!(entry.project_id, Some(project_id));
        
        entry.set_description("Development work on core API".to_string());
        assert_eq!(entry.description.as_deref(), Some("Development work on core API"));
        
        // Submit
        entry.submit();
        assert_eq!(entry.status, TimesheetStatus::Submitted);
        
        // Reject
        entry.reject();
        assert_eq!(entry.status, TimesheetStatus::Rejected);
        
        // Resubmit
        entry.submit();
        assert_eq!(entry.status, TimesheetStatus::Submitted);
        
        // Approve
        entry.approve();
        assert_eq!(entry.status, TimesheetStatus::Approved);
    }
}

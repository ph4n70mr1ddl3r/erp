use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReportStatus {
    New,
    InReview,
    Investigating,
    Resolved,
    Dismissed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IncidentCategory {
    Fraud,
    Harassment,
    SafetyViolation,
    EthicsViolation,
    FinancialMisconduct,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WhistleblowerReport {
    pub id: Uuid,
    pub case_number: String,
    pub title: String,
    pub description: String,
    pub category: IncidentCategory,
    pub severity: Severity,
    pub status: ReportStatus,
    pub is_anonymous: bool,
    pub reporter_id: Option<Uuid>,
    pub reported_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub assigned_to: Option<Uuid>,
    pub resolution_notes: Option<String>,
}

impl WhistleblowerReport {
    pub fn new(
        case_number: String,
        title: String,
        description: String,
        category: IncidentCategory,
        severity: Severity,
        is_anonymous: bool,
        reporter_id: Option<Uuid>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            case_number,
            title,
            description,
            category,
            severity,
            status: ReportStatus::New,
            is_anonymous,
            reporter_id: if is_anonymous { None } else { reporter_id },
            reported_at: now,
            updated_at: now,
            assigned_to: None,
            resolution_notes: None,
        }
    }

    pub fn assign_investigator(&mut self, investigator_id: Uuid) {
        self.assigned_to = Some(investigator_id);
        self.status = ReportStatus::InReview;
        self.updated_at = Utc::now();
    }

    pub fn start_investigation(&mut self) {
        if self.status == ReportStatus::InReview || self.status == ReportStatus::New {
            self.status = ReportStatus::Investigating;
            self.updated_at = Utc::now();
        }
    }

    pub fn resolve(&mut self, notes: String) {
        self.status = ReportStatus::Resolved;
        self.resolution_notes = Some(notes);
        self.updated_at = Utc::now();
    }

    pub fn dismiss(&mut self, reason: String) {
        self.status = ReportStatus::Dismissed;
        self.resolution_notes = Some(reason);
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whistleblower_report_lifecycle() {
        let mut report = WhistleblowerReport::new(
            "WB-2026-001".to_string(),
            "Suspicious Financial Activity".to_string(),
            "I observed unauthorized transfers in the finance department.".to_string(),
            IncidentCategory::FinancialMisconduct,
            Severity::High,
            true,
            None,
        );

        assert_eq!(report.status, ReportStatus::New);
        assert!(report.is_anonymous);
        assert!(report.reporter_id.is_none());

        let investigator_id = Uuid::new_v4();
        report.assign_investigator(investigator_id);
        assert_eq!(report.status, ReportStatus::InReview);
        assert_eq!(report.assigned_to, Some(investigator_id));

        report.start_investigation();
        assert_eq!(report.status, ReportStatus::Investigating);

        report.resolve("Investigated and corrected the unauthorized transfers.".to_string());
        assert_eq!(report.status, ReportStatus::Resolved);
        assert!(report.resolution_notes.is_some());
    }

    #[test]
    fn test_non_anonymous_report() {
        let reporter_id = Uuid::new_v4();
        let report = WhistleblowerReport::new(
            "WB-2026-002".to_string(),
            "Safety Concern".to_string(),
            "Exposed wiring in warehouse.".to_string(),
            IncidentCategory::SafetyViolation,
            Severity::Medium,
            false,
            Some(reporter_id),
        );

        assert!(!report.is_anonymous);
        assert_eq!(report.reporter_id, Some(reporter_id));
    }
}

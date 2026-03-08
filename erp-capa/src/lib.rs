use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CapaType {
    Corrective,
    Preventive,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CapaSource {
    Audit,
    CustomerComplaint,
    InternalIncident,
    RiskAssessment,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CapaStatus {
    Open,
    UnderInvestigation,
    ActionPlanDrafted,
    Implementation,
    UnderReview,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CapaRequest {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub capa_type: CapaType,
    pub source: CapaSource,
    pub status: CapaStatus,
    pub root_cause_analysis: Option<String>,
    pub action_plan: Option<String>,
    pub reported_by: Uuid,
    pub assigned_to: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CapaRequest {
    pub fn new(
        title: String,
        description: String,
        capa_type: CapaType,
        source: CapaSource,
        reported_by: Uuid,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            capa_type,
            source,
            status: CapaStatus::Open,
            root_cause_analysis: None,
            action_plan: None,
            reported_by,
            assigned_to: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn assign_to(&mut self, assignee_id: Uuid) {
        self.assigned_to = Some(assignee_id);
        if self.status == CapaStatus::Open {
            self.status = CapaStatus::UnderInvestigation;
        }
        self.updated_at = Utc::now();
    }

    pub fn provide_root_cause(&mut self, root_cause: String) {
        self.root_cause_analysis = Some(root_cause);
        self.updated_at = Utc::now();
    }

    pub fn draft_action_plan(&mut self, plan: String) {
        self.action_plan = Some(plan);
        if self.status == CapaStatus::UnderInvestigation || self.status == CapaStatus::Open {
            self.status = CapaStatus::ActionPlanDrafted;
        }
        self.updated_at = Utc::now();
    }

    pub fn start_implementation(&mut self) {
        if self.status == CapaStatus::ActionPlanDrafted {
            self.status = CapaStatus::Implementation;
            self.updated_at = Utc::now();
        }
    }

    pub fn submit_for_review(&mut self) {
        if self.status == CapaStatus::Implementation {
            self.status = CapaStatus::UnderReview;
            self.updated_at = Utc::now();
        }
    }

    pub fn close(&mut self) {
        self.status = CapaStatus::Closed;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capa_lifecycle() {
        let reporter_id = Uuid::new_v4();
        let assignee_id = Uuid::new_v4();
        
        let mut capa = CapaRequest::new(
            "High defect rate in Batch 402".to_string(),
            "Multiple units failed quality control due to misaligned seals.".to_string(),
            CapaType::Corrective,
            CapaSource::InternalIncident,
            reporter_id,
        );

        assert_eq!(capa.status, CapaStatus::Open);
        assert_eq!(capa.assigned_to, None);

        // Assign investigation
        capa.assign_to(assignee_id);
        assert_eq!(capa.status, CapaStatus::UnderInvestigation);
        assert_eq!(capa.assigned_to, Some(assignee_id));

        // Root cause
        capa.provide_root_cause("Sensor misalignment on assembly line 2.".to_string());
        assert_eq!(capa.root_cause_analysis.as_deref(), Some("Sensor misalignment on assembly line 2."));

        // Action plan
        capa.draft_action_plan("Recalibrate sensors and add secondary validation check.".to_string());
        assert_eq!(capa.status, CapaStatus::ActionPlanDrafted);

        // Implementation
        capa.start_implementation();
        assert_eq!(capa.status, CapaStatus::Implementation);

        // Review
        capa.submit_for_review();
        assert_eq!(capa.status, CapaStatus::UnderReview);

        // Close
        capa.close();
        assert_eq!(capa.status, CapaStatus::Closed);
    }
}

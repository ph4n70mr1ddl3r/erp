use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IncidentSeverity {
    Minor,
    Moderate,
    Major,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IncidentType {
    Injury,
    Illness,
    EnvironmentalSpill,
    NearMiss,
    PropertyDamage,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IncidentStatus {
    Reported,
    UnderInvestigation,
    Mitigated,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SafetyIncident {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub incident_type: IncidentType,
    pub severity: IncidentSeverity,
    pub status: IncidentStatus,
    pub reported_by: Uuid,
    pub location: String,
    pub date_occurred: DateTime<Utc>,
    pub investigation_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SafetyIncident {
    pub fn new(
        title: String,
        description: String,
        incident_type: IncidentType,
        severity: IncidentSeverity,
        location: String,
        reported_by: Uuid,
        date_occurred: DateTime<Utc>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            incident_type,
            severity,
            status: IncidentStatus::Reported,
            reported_by,
            location,
            date_occurred,
            investigation_notes: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn start_investigation(&mut self) {
        if self.status == IncidentStatus::Reported {
            self.status = IncidentStatus::UnderInvestigation;
            self.updated_at = Utc::now();
        }
    }

    pub fn add_investigation_notes(&mut self, notes: String) {
        let mut new_notes = self.investigation_notes.clone().unwrap_or_default();
        if !new_notes.is_empty() {
            new_notes.push_str("\n");
        }
        new_notes.push_str(&notes);
        self.investigation_notes = Some(new_notes);
        self.updated_at = Utc::now();
    }

    pub fn mark_mitigated(&mut self) {
        self.status = IncidentStatus::Mitigated;
        self.updated_at = Utc::now();
    }

    pub fn resolve(&mut self) {
        self.status = IncidentStatus::Resolved;
        self.updated_at = Utc::now();
    }

    pub fn close(&mut self) {
        self.status = IncidentStatus::Closed;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safety_incident_lifecycle() {
        let reporter_id = Uuid::new_v4();
        let incident_time = Utc::now();
        
        let mut incident = SafetyIncident::new(
            "Slip and fall in warehouse".to_string(),
            "Employee slipped on wet floor near loading dock A.".to_string(),
            IncidentType::Injury,
            IncidentSeverity::Moderate,
            "Warehouse Zone A".to_string(),
            reporter_id,
            incident_time,
        );

        assert_eq!(incident.status, IncidentStatus::Reported);
        
        incident.start_investigation();
        assert_eq!(incident.status, IncidentStatus::UnderInvestigation);

        incident.add_investigation_notes("Interviewed witness. Checked security camera.".to_string());
        assert_eq!(incident.investigation_notes.as_deref(), Some("Interviewed witness. Checked security camera."));

        incident.add_investigation_notes("Roof leak identified as source of water.".to_string());
        assert!(incident.investigation_notes.as_ref().unwrap().contains("Roof leak identified"));

        incident.mark_mitigated();
        assert_eq!(incident.status, IncidentStatus::Mitigated);

        incident.resolve();
        assert_eq!(incident.status, IncidentStatus::Resolved);

        incident.close();
        assert_eq!(incident.status, IncidentStatus::Closed);
    }
}

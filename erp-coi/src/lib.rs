use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DisclosureStatus {
    Draft,
    Submitted,
    UnderReview,
    Approved,
    Mitigated,
    Rejected,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DisclosureType {
    OutsideEmployment,
    FinancialInterest,
    FamilyRelationship,
    GiftOrHospitality,
    BoardMembership,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CoiDisclosure {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub disclosure_type: DisclosureType,
    pub title: String,
    pub description: String,
    pub status: DisclosureStatus,
    pub mitigation_plan: Option<String>,
    pub reviewer_id: Option<Uuid>,
    pub review_notes: Option<String>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CoiDisclosure {
    pub fn new(employee_id: Uuid, disclosure_type: DisclosureType, title: String, description: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            employee_id,
            disclosure_type,
            title,
            description,
            status: DisclosureStatus::Draft,
            mitigation_plan: None,
            reviewer_id: None,
            review_notes: None,
            submitted_at: None,
            reviewed_at: None,
            expires_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn submit(&mut self) {
        if self.status == DisclosureStatus::Draft {
            let now = Utc::now();
            self.status = DisclosureStatus::Submitted;
            self.submitted_at = Some(now);
            self.updated_at = now;
        }
    }

    pub fn review(&mut self, reviewer_id: Uuid, status: DisclosureStatus, notes: Option<String>) {
        let now = Utc::now();
        self.reviewer_id = Some(reviewer_id);
        self.status = status;
        self.review_notes = notes;
        self.reviewed_at = Some(now);
        self.updated_at = now;
    }

    pub fn set_mitigation_plan(&mut self, plan: String) {
        self.mitigation_plan = Some(plan);
        self.updated_at = Utc::now();
    }
}

pub struct CoiService {
    // Service methods for managing disclosures
}

impl CoiService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn validate_disclosure(&self, disclosure: &CoiDisclosure) -> bool {
        !disclosure.title.is_empty() && !disclosure.description.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disclosure_lifecycle() {
        let employee_id = Uuid::new_v4();
        let mut disclosure = CoiDisclosure::new(
            employee_id,
            DisclosureType::OutsideEmployment,
            "Freelance Consulting".to_string(),
            "I am doing occasional web development consulting on weekends.".to_string(),
        );

        assert_eq!(disclosure.status, DisclosureStatus::Draft);
        assert!(disclosure.submitted_at.is_none());

        disclosure.submit();
        assert_eq!(disclosure.status, DisclosureStatus::Submitted);
        assert!(disclosure.submitted_at.is_some());

        let reviewer_id = Uuid::new_v4();
        disclosure.review(
            reviewer_id,
            DisclosureStatus::Mitigated,
            Some("Approved provided it does not interfere with primary duties.".to_string()),
        );

        assert_eq!(disclosure.status, DisclosureStatus::Mitigated);
        assert_eq!(disclosure.reviewer_id, Some(reviewer_id));
        assert!(disclosure.reviewed_at.is_some());

        disclosure.set_mitigation_plan("Will not use company equipment for consulting.".to_string());
        assert!(disclosure.mitigation_plan.is_some());
    }

    #[test]
    fn test_service_validation() {
        let service = CoiService::new();
        let employee_id = Uuid::new_v4();
        
        let valid_disclosure = CoiDisclosure::new(
            employee_id,
            DisclosureType::FinancialInterest,
            "Stock Ownership".to_string(),
            "Ownership in a competitor.".to_string(),
        );
        assert!(service.validate_disclosure(&valid_disclosure));

        let invalid_disclosure = CoiDisclosure::new(
            employee_id,
            DisclosureType::Other,
            "".to_string(),
            "".to_string(),
        );
        assert!(!service.validate_disclosure(&invalid_disclosure));
    }
}

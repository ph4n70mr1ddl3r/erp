use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VisitStatus {
    Pending,
    CheckedIn,
    CheckedOut,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Visitor {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Visit {
    pub id: Uuid,
    pub visitor_id: Uuid,
    pub host_id: Uuid, // Employee ID
    pub purpose: String,
    pub status: VisitStatus,
    pub location: String,
    pub check_in_time: Option<DateTime<Utc>>,
    pub check_out_time: Option<DateTime<Utc>>,
    pub planned_start: DateTime<Utc>,
    pub planned_end: Option<DateTime<Utc>>,
    pub nda_signed: bool,
    pub badge_issued: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Visitor {
    pub fn new(first_name: String, last_name: String, email: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            first_name,
            last_name,
            email,
            phone: None,
            company: None,
            created_at: now,
            updated_at: now,
        }
    }
}

impl Visit {
    pub fn new(visitor_id: Uuid, host_id: Uuid, purpose: String, planned_start: DateTime<Utc>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            visitor_id,
            host_id,
            purpose,
            status: VisitStatus::Pending,
            location: "Main Lobby".to_string(),
            check_in_time: None,
            check_out_time: None,
            planned_start,
            planned_end: None,
            nda_signed: false,
            badge_issued: false,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn check_in(&mut self) {
        let now = Utc::now();
        self.status = VisitStatus::CheckedIn;
        self.check_in_time = Some(now);
        self.updated_at = now;
    }

    pub fn check_out(&mut self) {
        let now = Utc::now();
        self.status = VisitStatus::CheckedOut;
        self.check_out_time = Some(now);
        self.updated_at = now;
    }

    pub fn sign_nda(&mut self) {
        self.nda_signed = true;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_visitor_visit_flow() {
        let visitor = Visitor::new(
            "John".to_string(),
            "Doe".to_string(),
            "john.doe@example.com".to_string(),
        );

        let host_id = Uuid::new_v4();
        let planned_start = Utc::now() + Duration::hours(1);
        let mut visit = Visit::new(
            visitor.id,
            host_id,
            "Business Meeting".to_string(),
            planned_start,
        );

        assert_eq!(visit.status, VisitStatus::Pending);
        assert!(!visit.nda_signed);

        visit.sign_nda();
        assert!(visit.nda_signed);

        visit.check_in();
        assert_eq!(visit.status, VisitStatus::CheckedIn);
        assert!(visit.check_in_time.is_some());

        visit.check_out();
        assert_eq!(visit.status, VisitStatus::CheckedOut);
        assert!(visit.check_out_time.is_some());
    }
}

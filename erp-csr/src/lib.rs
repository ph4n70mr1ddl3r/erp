use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DonationStatus {
    Pledged,
    Processing,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DonationType {
    Monetary,
    InKind,
    Service,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CharityPartner {
    pub id: Uuid,
    pub name: String,
    pub mission: Option<String>,
    pub tax_id: String,
    pub website: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Donation {
    pub id: Uuid,
    pub charity_id: Uuid,
    pub donor_id: Uuid, // Can be employee or company
    pub amount: f64,
    pub currency: String,
    pub donation_type: DonationType,
    pub status: DonationStatus,
    pub donation_date: NaiveDate,
    pub receipt_number: Option<String>,
    pub notes: Option<String>,
    pub matching_donation_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VolunteerActivity {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub charity_id: Uuid,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub points_per_hour: u32,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VolunteerLog {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub activity_id: Uuid,
    pub date: NaiveDate,
    pub hours: f64,
    pub status: String, // e.g., "Pending Approval", "Approved"
    pub points_earned: u32,
}

impl Donation {
    pub fn new_monetary(charity_id: Uuid, donor_id: Uuid, amount: f64, currency: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            charity_id,
            donor_id,
            amount,
            currency,
            donation_type: DonationType::Monetary,
            status: DonationStatus::Pledged,
            donation_date: Utc::now().date_naive(),
            receipt_number: None,
            notes: None,
            matching_donation_id: None,
            created_at: Utc::now(),
        }
    }

    pub fn complete(&mut self, receipt: String) {
        self.status = DonationStatus::Completed;
        self.receipt_number = Some(receipt);
    }
}

pub struct CsrService {
    // Placeholder for database logic
}

impl CsrService {
    pub fn calculate_volunteer_points(&self, hours: f64, points_per_hour: u32) -> u32 {
        (hours * points_per_hour as f64).round() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_donation_lifecycle() {
        let charity_id = Uuid::new_v4();
        let employee_id = Uuid::new_v4();
        
        let mut donation = Donation::new_monetary(charity_id, employee_id, 100.0, "USD".to_string());
        assert_eq!(donation.status, DonationStatus::Pledged);
        
        donation.complete("REC-999".to_string());
        assert_eq!(donation.status, DonationStatus::Completed);
        assert_eq!(donation.receipt_number.as_deref(), Some("REC-999"));
    }

    #[test]
    fn test_volunteer_point_calculation() {
        let service = CsrService {};
        let points = service.calculate_volunteer_points(2.5, 10);
        assert_eq!(points, 25);
    }
}

use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AssignmentStatus {
    Planning,
    Active,
    Completed,
    Extended,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AssignmentType {
    ShortTerm, // < 1 year
    LongTerm,  // 1-5 years
    PermanentTransfer,
    Commuter,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VisaRecord {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub country_code: String,
    pub visa_type: String,
    pub visa_number: String,
    pub expiry_date: NaiveDate,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InternationalAssignment {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub assignment_type: AssignmentType,
    pub home_country: String,
    pub host_country: String,
    pub home_entity_id: Uuid,
    pub host_entity_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub status: AssignmentStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelocationExpense {
    pub id: Uuid,
    pub assignment_id: Uuid,
    pub description: String,
    pub amount: f64,
    pub currency: String,
    pub approved: bool,
    pub date: NaiveDate,
}

impl InternationalAssignment {
    pub fn new(
        employee_id: Uuid,
        assignment_type: AssignmentType,
        home_country: String,
        host_country: String,
        home_entity_id: Uuid,
        host_entity_id: Uuid,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            employee_id,
            assignment_type,
            home_country,
            host_country,
            home_entity_id,
            host_entity_id,
            start_date,
            end_date,
            status: AssignmentStatus::Planning,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn activate(&mut self) {
        self.status = AssignmentStatus::Active;
        self.updated_at = Utc::now();
    }

    pub fn complete(&mut self) {
        self.status = AssignmentStatus::Completed;
        self.updated_at = Utc::now();
    }
}

pub struct MobilityService {}

impl MobilityService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn is_visa_expiring_soon(&self, visa: &VisaRecord, threshold_days: i64) -> bool {
        let today = Utc::now().date_naive();
        let days_until = (visa.expiry_date - today).num_days();
        days_until >= 0 && days_until <= threshold_days
    }

    pub fn calculate_total_relocation_cost(&self, expenses: &[RelocationExpense], currency: &str) -> f64 {
        expenses.iter()
            .filter(|e| e.currency == currency)
            .map(|e| e.amount)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assignment_lifecycle() {
        let emp_id = Uuid::new_v4();
        let home_id = Uuid::new_v4();
        let host_id = Uuid::new_v4();
        let start = NaiveDate::from_ymd_opt(2026, 6, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2028, 6, 1).unwrap();

        let mut assignment = InternationalAssignment::new(
            emp_id,
            AssignmentType::LongTerm,
            "US".to_string(),
            "DE".to_string(),
            home_id,
            host_id,
            start,
            end,
        );

        assert_eq!(assignment.status, AssignmentStatus::Planning);
        
        assignment.activate();
        assert_eq!(assignment.status, AssignmentStatus::Active);

        assignment.complete();
        assert_eq!(assignment.status, AssignmentStatus::Completed);
    }

    #[test]
    fn test_visa_expiry_check() {
        let service = MobilityService::new();
        let today = Utc::now().date_naive();
        
        let visa = VisaRecord {
            id: Uuid::new_v4(),
            employee_id: Uuid::new_v4(),
            country_code: "DE".to_string(),
            visa_type: "Work".to_string(),
            visa_number: "V123".to_string(),
            expiry_date: today + chrono::Duration::days(15),
            status: "Active".to_string(),
        };

        // Should be true if checking for 30 day threshold
        assert!(service.is_visa_expiring_soon(&visa, 30));
        // Should be false if checking for 10 day threshold
        assert!(!service.is_visa_expiring_soon(&visa, 10));
    }

    #[test]
    fn test_relocation_costs() {
        let service = MobilityService::new();
        let assignment_id = Uuid::new_v4();
        let date = Utc::now().date_naive();

        let expenses = vec![
            RelocationExpense {
                id: Uuid::new_v4(),
                assignment_id,
                description: "Flights".to_string(),
                amount: 1500.0,
                currency: "USD".to_string(),
                approved: true,
                date,
            },
            RelocationExpense {
                id: Uuid::new_v4(),
                assignment_id,
                description: "Shipping".to_string(),
                amount: 3000.0,
                currency: "USD".to_string(),
                approved: true,
                date,
            },
            RelocationExpense {
                id: Uuid::new_v4(),
                assignment_id,
                description: "Local Agent".to_string(),
                amount: 500.0,
                currency: "EUR".to_string(), // Different currency
                approved: true,
                date,
            },
        ];

        assert_eq!(service.calculate_total_relocation_cost(&expenses, "USD"), 4500.0);
        assert_eq!(service.calculate_total_relocation_cost(&expenses, "EUR"), 500.0);
    }
}

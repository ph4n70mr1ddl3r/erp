use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PolicyStatus {
    Draft,
    Active,
    Expired,
    Cancelled,
    PendingRenewal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InsuranceType {
    Property,
    GeneralLiability,
    ProfessionalLiability,
    DirectorsAndOfficers,
    WorkersCompensation,
    Cyber,
    Health,
    Fleet,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InsurancePolicy {
    pub id: Uuid,
    pub policy_number: String,
    pub insurer_name: String,
    pub insurance_type: InsuranceType,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub total_premium: f64,
    pub currency: String,
    pub status: PolicyStatus,
    pub coverage_limit: f64,
    pub deductible: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClaimStatus {
    Reported,
    UnderInvestigation,
    Approved,
    Paid,
    Denied,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InsuranceClaim {
    pub id: Uuid,
    pub policy_id: Uuid,
    pub claim_number: String,
    pub date_of_loss: DateTime<Utc>,
    pub date_reported: DateTime<Utc>,
    pub status: ClaimStatus,
    pub estimated_loss: f64,
    pub settlement_amount: Option<f64>,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl InsurancePolicy {
    pub fn new(
        policy_number: String,
        insurer_name: String,
        insurance_type: InsuranceType,
        start_date: NaiveDate,
        end_date: NaiveDate,
        total_premium: f64,
        currency: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            policy_number,
            insurer_name,
            insurance_type,
            start_date,
            end_date,
            total_premium,
            currency,
            status: PolicyStatus::Draft,
            coverage_limit: 0.0,
            deductible: 0.0,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn activate(&mut self) {
        self.status = PolicyStatus::Active;
        self.updated_at = Utc::now();
    }

    pub fn is_active_on(&self, date: NaiveDate) -> bool {
        self.status == PolicyStatus::Active && date >= self.start_date && date <= self.end_date
    }
}

impl InsuranceClaim {
    pub fn new(policy_id: Uuid, claim_number: String, date_of_loss: DateTime<Utc>, description: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            policy_id,
            claim_number,
            date_of_loss,
            date_reported: now,
            status: ClaimStatus::Reported,
            estimated_loss: 0.0,
            settlement_amount: None,
            description,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_status(&mut self, status: ClaimStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    pub fn settle(&mut self, amount: f64) {
        self.settlement_amount = Some(amount);
        self.status = ClaimStatus::Paid;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_lifecycle() {
        let start = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2026, 12, 31).unwrap();
        let mut policy = InsurancePolicy::new(
            "POL-12345".to_string(),
            "Global Assurance".to_string(),
            InsuranceType::Cyber,
            start,
            end,
            12000.0,
            "USD".to_string(),
        );

        assert_eq!(policy.status, PolicyStatus::Draft);
        assert!(!policy.is_active_on(NaiveDate::from_ymd_opt(2026, 6, 1).unwrap()));

        policy.activate();
        assert_eq!(policy.status, PolicyStatus::Active);
        assert!(policy.is_active_on(NaiveDate::from_ymd_opt(2026, 6, 1).unwrap()));
        assert!(!policy.is_active_on(NaiveDate::from_ymd_opt(2027, 1, 1).unwrap()));
    }

    #[test]
    fn test_claim_flow() {
        let policy_id = Uuid::new_v4();
        let loss_date = Utc::now();
        let mut claim = InsuranceClaim::new(
            policy_id,
            "CLM-2026-001".to_string(),
            loss_date,
            "Data breach incident".to_string(),
        );

        assert_eq!(claim.status, ClaimStatus::Reported);
        
        claim.update_status(ClaimStatus::UnderInvestigation);
        assert_eq!(claim.status, ClaimStatus::UnderInvestigation);

        claim.settle(50000.0);
        assert_eq!(claim.status, ClaimStatus::Paid);
        assert_eq!(claim.settlement_amount, Some(50000.0));
    }
}
